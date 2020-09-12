extern crate sdl2;

use std::{env, fs, thread, time};

mod cpu;
mod display;
mod keyboard;

#[cfg(target_os = "emscripten")]
pub mod emscripten;

use cpu::Cpu;
use display::Display;
use keyboard::Keyboard;

const FIFTEEN_PUZZLE: [u8; 384] = [
    0, 224, 108, 0, 76, 0, 110, 15, 162, 3, 96, 32, 240, 85, 0, 224, 34, 190, 34, 118, 34, 142, 34,
    94, 34, 70, 18, 16, 97, 0, 98, 23, 99, 4, 65, 16, 0, 238, 162, 232, 241, 30, 240, 101, 64, 0,
    18, 52, 240, 41, 210, 53, 113, 1, 114, 5, 100, 3, 132, 18, 52, 0, 18, 34, 98, 23, 115, 6, 18,
    34, 100, 3, 132, 226, 101, 3, 133, 210, 148, 80, 0, 238, 68, 3, 0, 238, 100, 1, 132, 228, 34,
    166, 18, 70, 100, 3, 132, 226, 101, 3, 133, 210, 148, 80, 0, 238, 68, 0, 0, 238, 100, 255, 132,
    228, 34, 166, 18, 94, 100, 12, 132, 226, 101, 12, 133, 210, 148, 80, 0, 238, 68, 0, 0, 238,
    100, 252, 132, 228, 34, 166, 18, 118, 100, 12, 132, 226, 101, 12, 133, 210, 148, 80, 0, 238,
    68, 12, 0, 238, 100, 4, 132, 228, 34, 166, 18, 142, 162, 232, 244, 30, 240, 101, 162, 232, 254,
    30, 240, 85, 96, 0, 162, 232, 244, 30, 240, 85, 142, 64, 0, 238, 60, 0, 18, 210, 34, 28, 34,
    216, 34, 28, 162, 248, 253, 30, 240, 101, 141, 0, 0, 238, 124, 255, 205, 15, 0, 238, 125, 1,
    96, 15, 141, 2, 237, 158, 18, 216, 237, 161, 18, 226, 0, 238, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
    11, 12, 13, 14, 15, 0, 13, 0, 1, 2, 4, 5, 6, 8, 9, 10, 12, 14, 3, 7, 11, 15, 132, 228, 34, 166,
    18, 118, 100, 12, 132, 226, 101, 12, 133, 210, 148, 80, 0, 238, 68, 12, 0, 238, 100, 4, 132,
    228, 34, 166, 18, 142, 162, 232, 244, 30, 240, 101, 162, 232, 254, 30, 240, 85, 96, 0, 162,
    232, 244, 30, 240, 85, 142, 64, 0, 238, 60, 0, 18, 210, 34, 28, 34, 216, 34, 28, 162, 248, 253,
    30, 240, 101, 141, 0, 0, 238, 124, 255, 205, 15, 0, 238, 125, 1, 96, 15, 141, 2, 237, 158, 18,
    216, 237, 161, 18, 226, 0, 238, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0, 13, 0, 1,
    2, 4, 5, 6, 8,
];

fn load_rom() -> Vec<u8> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        FIFTEEN_PUZZLE.to_vec()
    } else {
        let rom_path = &args[1];
        match fs::read(rom_path) {
            Ok(vec) => {
                println!("{:?}", vec);
                vec
            }
            Err(e) => panic!("{}", e),
        }
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

    let mut main_loop = || {
        cpu.cycle();
        thread::sleep(time::Duration::from_millis(2));
    };

    #[cfg(target_os = "emscripten")]
    {
        use emscripten::emscripten;
        emscripten::set_main_loop_callback(main_loop);
    }

    #[cfg(not(target_os = "emscripten"))]
    loop {
        main_loop();
    }
}
