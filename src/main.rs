extern crate sdl2;

use std::{env, fs, thread, time};

mod cpu;
mod display;
mod keyboard;

use cpu::Cpu;
use display::Display;
use keyboard::Keyboard;

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

fn main() {
    let rom = load_rom();
    let sdl_context = init_sdl();
    let display = Display::new(&sdl_context);
    let keyboard = Keyboard::new(&sdl_context);
    let mut cpu = Cpu::new(rom, display, keyboard);
    loop {
        cpu.cycle();
        thread::sleep(time::Duration::from_millis(2));
    }
}
