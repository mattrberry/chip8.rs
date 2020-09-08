use rand::prelude::*;

const PC: u16 = 0x200;
const FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Cpu {
    memory: [u8; 4096],
    v: [u8; 16],
    i: u16,
    pc: u16,
    stack: Vec<u16>, // not enforcing stack size of 16
}

impl Cpu {
    pub fn new(rom: Vec<u8>) -> Cpu {
        let mut memory = [0u8; 4096];
        memory[..FONTSET.len()].clone_from_slice(&FONTSET);
        memory[PC as usize..PC as usize + rom.len()].clone_from_slice(&rom);
        println!("ROM: {:?}", rom);
        println!("memory size: {}", memory.len());
        Cpu {
            memory: memory,
            v: [0; 16],
            i: 0,
            pc: PC,
            stack: Vec::<u16>::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.process_opcode(opcode);
        }
    }

    fn read_u8(&mut self) -> u8 {
        let byte = self.memory[self.pc as usize];
        self.pc = self.pc.wrapping_add(1);
        return byte;
    }

    fn read_opcode(&mut self) -> u16 {
        return (self.read_u8() as u16) << 8 | self.read_u8() as u16;
    }

    fn process_opcode(&mut self, opcode: u16) {
        println!("pc: {:#X}, opcode: {:#X}", self.pc, opcode);
        let op_1 = (opcode & 0xF000) >> 12;
        let op_2 = (opcode & 0x0F00) >> 8;
        let op_3 = (opcode & 0x00F0) >> 4;
        let op_4 = opcode & 0x000F;

        let nn = (opcode & 0x00FF) as u8;
        let nnn = opcode & 0x0FFF;

        match (op_1, op_2, op_3, op_4) {
            (0x0, 0x0, 0xE, 0x0) => (), // clear the display
            (0x0, 0x0, 0xE, 0xE) => self.pc = self.stack.pop().unwrap(),
            (0x1, _, _, _) => self.pc = nnn,
            (0x2, _, _, _) => {
                self.stack.push(self.pc);
                self.pc = nnn;
            }
            (0x3, x @ _, _, _) => {
                if self.v[x as usize] == nn {
                    self.pc += 2;
                }
            }
            (0x4, x @ _, _, _) => {
                if self.v[x as usize] != nn {
                    self.pc += 2;
                }
            }
            (0x5, x @ _, y @ _, _) => {
                if self.v[x as usize] == self.v[y as usize] {
                    self.pc += 2;
                }
            }
            (0x6, x @ _, _, _) => self.v[x as usize] = nn,
            (0x7, x @ _, _, _) => self.v[x as usize] = self.v[x as usize].wrapping_add(nn),
            (0x8, x @ _, y @ _, 0x0) => self.v[x as usize] = self.v[y as usize],
            (0x8, x @ _, y @ _, 0x1) => self.v[x as usize] |= self.v[y as usize],
            (0x8, x @ _, y @ _, 0x2) => self.v[x as usize] &= self.v[y as usize],
            (0x8, x @ _, y @ _, 0x3) => self.v[x as usize] ^= self.v[y as usize],
            (0x8, x @ _, y @ _, 0x4) => {
                self.v[x as usize] = self.v[x as usize].wrapping_add(self.v[y as usize]);
                self.v[0xF] = if self.v[x as usize] < self.v[y as usize] {
                    1
                } else {
                    0
                };
            }
            (0x8, x @ _, y @ _, 0x5) => {
                self.v[0xF] = if self.v[y as usize] > self.v[x as usize] {
                    0
                } else {
                    1
                };
                self.v[x as usize] = self.v[x as usize].wrapping_sub(self.v[y as usize]);
            }
            (0x8, x @ _, _, 0x6) => {
                self.v[0xF] = self.v[x as usize] & 0x1;
                self.v[x as usize] >>= 1;
            }
            (0x8, x @ _, y @ _, 0x7) => {
                self.v[0xF] = if self.v[x as usize] > self.v[y as usize] {
                    0
                } else {
                    1_u8
                };
                self.v[x as usize] = self.v[y as usize].wrapping_sub(self.v[x as usize]);
            }
            (0x8, x @ _, _, 0xE) => {
                self.v[0xF] = (self.v[x as usize] & 0x80) >> 7;
                self.v[x as usize] <<= 1;
            }
            (0x9, x @ _, y @ _, _) => {
                if self.v[x as usize] != self.v[y as usize] {
                    self.pc += 2
                }
            }
            (0xA, _, _, _) => self.i = nnn,
            (0xB, _, _, _) => self.pc = self.v[0] as u16 + nnn,
            (0xC, x @ _, _, _) => {
                let rand: u8 = random();
                self.v[x as usize] = rand & nn;
            }
            (0xD, x @ _, _, _) => (),     // draw sprite
            (0xE, x @ _, 0x9, 0xE) => (), // skip next op if key pressed
            (0xE, x @ _, 0xA, 0x1) => (), // skip next op if key not pressed
            (0xF, x @ _, 0x0, 0x7) => (), // load delay timer to v[x]
            (0xF, x @ _, 0x0, 0xA) => (), // wait for keypress then load to v[x]
            (0xF, x @ _, 0x1, 0x5) => (), // store v[x] to delay timer
            (0xF, x @ _, 0x1, 0x8) => (), // store v[x] to sound timer
            (0xF, x @ _, 0x1, 0xE) => self.i = self.i.wrapping_add(self.v[x as usize] as u16),
            (0xF, x @ _, 0x2, 0x9) => self.i = self.v[x as usize] as u16 * 5,
            (0xF, x @ _, 0x3, 0x3) => {
                self.memory[self.i as usize] = (self.v[x as usize] / 100) as u8;
                self.memory[self.i as usize + 1] = ((self.v[x as usize] / 10) % 10) as u8;
                self.memory[self.i as usize + 2] = ((self.v[x as usize] % 100) % 10) as u8;
            }
            (0xF, x @ _, 0x5, 0x5) => {
                self.memory[self.i as usize..=self.i as usize + x as usize]
                    .clone_from_slice(&self.v[..=x as usize + 1]);
            },
            _ => panic!("Unimplemented opcode: {:#X}", opcode),
        }
    }
}
