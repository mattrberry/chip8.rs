use std::env;
use std::fs;

mod cpu;

use cpu::Cpu;

fn load_rom(path: &str) -> Vec<u8> {
    match fs::read(path) {
        Ok(vec) => vec,
        Err(e) => panic!("{}", e),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let rom_path = &args[1];
    let rom = load_rom(rom_path);
    let mut cpu = Cpu::new(rom);
    cpu.run();
}
