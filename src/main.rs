extern crate sdl2;

use std::env;
use std::fs;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod cpu;
mod display;

use cpu::Cpu;
use display::Display;

fn load_rom() -> Vec<u8> {
    let args: Vec<String> = env::args().collect();
    let rom_path = &args[1];
    match fs::read(rom_path) {
        Ok(vec) => vec,
        Err(e) => panic!("{}", e),
    }
}

fn init_sdl() -> sdl2::Sdl {
    sdl2::init().expect("Failed to initialize SDL2")
}

fn run(sdl_context: &sdl2::Sdl, mut cpu: Cpu) {
    let mut event_pump = sdl_context.event_pump().expect("Failed to pump");
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        cpu.cycle()
    }
}

fn main() {
    let rom = load_rom();
    let sdl_context = init_sdl();
    let display = Display::new(&sdl_context);
    let cpu = Cpu::new(rom, display);
    run(&sdl_context, cpu);
}
