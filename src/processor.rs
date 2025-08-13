use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use crate::display::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::font;

const RAM_SIZE: usize = 4096;
const NUM_REGS: usize = 16;
const STACK_SIZE: usize = 16;
const NUM_KEYS: usize = 16;
const PROGRAM_START: usize = 0x200;

pub struct Chip8 {
    memory: [u8; RAM_SIZE],
    display: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    keys: [bool; NUM_KEYS],
    pc: u16,
    i_reg: u16,
    sp: u16,
    stack: [u16; STACK_SIZE],
    v_reg: [u8; NUM_REGS],
    delay_timer: u8,
    sound_timer: u8,
}

impl Chip8 {
    pub fn new() -> Self {
        let mut new_chip8: Chip8 = Self {
            memory: [0; RAM_SIZE],
            display: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            pc: PROGRAM_START as u16,
            i_reg: 0,
            sp: 0,
            stack: [0; STACK_SIZE],
            keys: [false; NUM_KEYS],
            delay_timer: 0,
            sound_timer: 0,
            v_reg: [0; 16],
        };

        new_chip8.memory[..font::FONTSET_SIZE].copy_from_slice(&font::FONTSET);

        return new_chip8
    }

    pub fn reset(&mut self){
        self.memory = [0; RAM_SIZE];
        self.display = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
        self.pc = PROGRAM_START as u16;
        self.i_reg = 0;
        self.sp = 0;
        self.stack = [0; STACK_SIZE];
        self.keys = [false; NUM_KEYS];
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.v_reg = [0; 16];
        self.memory[..font::FONTSET_SIZE].copy_from_slice(&font::FONTSET);
    }

    fn push(&mut self, val: u16){
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }

    fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }

    pub fn dump_mem(&mut self){
        println!("{:x?}", self.memory);
    }

    pub fn get_display(&mut self) -> [bool; SCREEN_HEIGHT * SCREEN_WIDTH] {
        self.display
    }

    pub fn update_keys(&mut self, inputs: [bool; 16]) {
        self.keys = inputs;
    }

    pub fn load_file(&mut self, path: &str){
        let path = Path::new(&path);
        let display = path.display();

        let mut buffer= Vec::<u8>::new();

        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };

        if let Err(why) = file.read_to_end(&mut buffer) {
            panic!("couldn't read {}: {}", display, why);
        }

        let start = PROGRAM_START;
        let end = PROGRAM_START + buffer.len();
        self.memory[start..end].copy_from_slice(&buffer);
    }

    pub fn tick(&mut self){
        let op = self.fetch();
        self.execute(op);
    }

    pub fn tick_timers(&mut self){
        if self.delay_timer > 0 {
            self.delay_timer -= 1
        }

        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                //BEEP
            }
            self.sound_timer -= 1
        }
    }

    fn fetch(&mut self) -> u16 {
        let high_byte = self.memory[self.pc as usize] as u16;
        let low_byte = self.memory[(self.pc + 1) as usize] as u16;

        let op = (high_byte << 8) | low_byte;

        self.pc += 2;
        op
    }

    fn execute(&mut self, op: u16){
        let dig1 = (op & 0xF000) >> 12;
        let dig2 = (op & 0x0F00) >> 8;
        let dig3 = (op & 0x00F0) >> 4;
        let dig4 = op & 0x000F;

        match(dig1, dig2, dig3, dig4) {
            
            //NOP
            (0, 0, 0, 0) => return,

            //CLS
            (0, 0, 0xE, 0) => {
                self.display = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
            },

            //RET
            (0, 0, 0xE, 0xE) => {
                self.pc = self.pop();
            },

            //JP
            (1, _, _, _) => {
                let nnn = op & 0xFFF;
                self.pc = nnn;
            },

            //CALL addr
            (2, _, _, _) => {
                let nnn = op & 0xFFF;

                self.push(self.pc);
                self.pc = nnn;
            },

            //SE VX, byte
            (3, _, _, _) => {
                let x = dig2 as usize;
                let kk = (op & 0xFF) as u8;

                if self.v_reg[x] == kk {
                    self.pc += 2
                };
            },

            //SNE Vx, byte
            (4, _, _, _) => {
                let x = dig2 as usize;
                let kk = (op & 0xFF) as u8;

                if self.v_reg[x] != kk {
                    self.pc += 2
                };
            },

            //SE, Vx, Vy
            (5, _, _, 0) => {
                let x = dig2 as usize;
                let y = dig3 as usize;

                if self.v_reg[x] == self.v_reg[y] {
                    self.pc += 2;
                }
            },

            //LD Vx, byte
            (6, _, _, _) => {
                let x = dig2 as usize;
                let kk = (op & 0xFF) as u8;

                self.v_reg[x] = kk;
            },

            //ADD Vx, byte
            (7, _, _, _) => {
                let x = dig2 as usize;
                let kk = (op & 0xFF) as u8;

                self.v_reg[x] = self.v_reg[x].wrapping_add(kk);
            },

            //LD Vx, Vy
            (8, _, _, 0) => {
                let x = dig2 as usize;
                let y = dig3 as usize;

                self.v_reg[x] = self.v_reg[y];
            },

            //OR Vx, Vy
            (8, _, _, 1) => {
                let x = dig2 as usize;
                let y = dig3 as usize;

                self.v_reg[x] = self.v_reg[x] | self.v_reg[y];
            },

            //AND Vx, Vy
            (8, _, _, 2) => {
                let x = dig2 as usize;
                let y = dig3 as usize;

                self.v_reg[x] = self.v_reg[x] & self.v_reg[y];
            },

            //XOR Vx, Vy
            (8, _, _, 3) => {
                let x = dig2 as usize;
                let y = dig3 as usize;

                self.v_reg[x] = self.v_reg[x] ^ self.v_reg[y];
            },

            //ADD Vx, Vy
            (8, _, _, 4) => {
                let x = dig2 as usize;
                let y = dig3 as usize;

                let (sum, carry) = self.v_reg[x].overflowing_add(self.v_reg[y]);
                let new_vf = if carry { 1 } else { 0 };

                self.v_reg[x] = sum;
                self.v_reg[0xF] = new_vf;
            },
            
            //SUB Vx, Vy
            (8, _, _, 5) => {
                let x = dig2 as usize;
                let y = dig3 as usize;

                let (sum, carry) = self.v_reg[x].overflowing_sub(self.v_reg[y]);
                let new_vf = if carry { 0 } else { 1 };

                self.v_reg[x] = sum;
                self.v_reg[0xF] = new_vf;
            },

            //SHR Vx {, Vy}
            (8, _, _, 6) => {
                let x = dig2 as usize;
                let lsb = self.v_reg[x] & 1;

                self.v_reg[x] >>= 1;
                self.v_reg[0xF] = lsb;
            }, 

            //SUBN Vx, Vy
            (8, _, _, 7) => {
                let x = dig2 as usize;
                let y = dig3 as usize;

                let (sum, carry) = self.v_reg[y].overflowing_sub(self.v_reg[x]);
                let new_vf = if carry { 0 } else { 1 };

                self.v_reg[x] = sum;
                self.v_reg[0xF] = new_vf;
            },

            //SHL Vx, {Vy}
            (8, _, _, 0xE) => {
                let x = dig2 as usize;
                let msb = (self.v_reg[x] >> 7) & 1;

                self.v_reg[x] <<= 1;
                self.v_reg[0xF] = msb;
            },

            //SNE Vx, Vy
            (9, _, _, 0) => {
                let x = dig2 as usize;
                let y = dig3 as usize;

                if self.v_reg[x] != self.v_reg[y] {
                    self.pc += 2
                };
            },

            //LD I, addr
            (0xA, _, _, _) => {
                let nnn = op & 0xFFF;

                self.i_reg = nnn;
            },

            //JP V0, addr
            (0xB, _, _, _) => {
                let nnn = op & 0xFFF;

                self.pc = nnn + self.v_reg[0] as u16;
            },

            //RND Vx, byte
            (0xC, _, _, _) => {
                let x = dig2 as usize;
                let kk = (op & 0xFF) as u8;
                let rand = rand::random_range(0..255) as u8;

                self.v_reg[x] = rand & kk;
            },

            //DRW Vx, Vy, nibble
            (0xD, _, _, _) => {
                let x_coord = self.v_reg[dig2 as usize] as u16;
                let y_coord = self.v_reg[dig3 as usize] as u16;
                let num_rows = dig4;
                let mut flipped = false;
                
                for y_line in 0..num_rows {
                    let addr = self.i_reg + y_line as u16;
                    let pixels = self.memory[addr as usize];

                    for x_line in 0..8 {
                        if pixels & (0b10000000 >> x_line) != 0 {
                            let x = (x_coord + x_line) as usize % SCREEN_WIDTH;
                            let y = (y_coord + y_line) as usize % SCREEN_HEIGHT;

                            let idx = x + SCREEN_WIDTH * y;
                            
                            flipped |= self.display[idx];
                            self.display[idx] = true;
                        }
                    }
                }

                if flipped {
                    self.v_reg[0xF] = 1;
                } else {
                    self.v_reg[0xF] = 0;
                }
            },

            //SKP Vx
            (0xE, _, 9, 0xE) => {
                let key_pressed = self.keys[self.v_reg[dig2 as usize] as usize];

                if key_pressed {
                    self.pc += 2;
                }
            },

            //SKNP Vx
            (0xE, _, 0xA, 1) => {
                let key_pressed = self.keys[self.v_reg[dig2 as usize] as usize];

                if !key_pressed {
                    self.pc += 2;
                }
            },

            //LD Vx, DT
            (0xF, _, 0, 7) => {
                self.v_reg[dig2 as usize] = self.delay_timer;
            },

            //LD, Vx, K
            (0xF, _, 0, 0xA) => {
                let x = dig2 as usize;
                let mut pressed = false;

                for i in 0..self.keys.len() {
                    if self.keys[i] == true {
                        self.v_reg[x] = i as u8;
                        pressed = true;
                        break;
                    }
                }

                if !pressed {
                    self.pc -= 2;
                }
            },

            //LD DT, Vx
            (0xF, _, 1, 5) => {
                self.delay_timer = self.v_reg[dig2 as usize];
            },

            //LD ST, Vx
            (0xF, _, 1, 8) => {
                self.sound_timer = self.v_reg[dig2 as usize];
            },

            //ADD I, Vx
            (0xF, _, 1, 0xE) => {
                self.i_reg += self.v_reg[dig2 as usize] as u16;
            },

            //LD F, Vx
            (0xF, _, 2, 9) => {
                let digit = dig2 as usize;

                self.i_reg = (self.v_reg[digit] * 5) as u16;
            },

            //LD B, Vx
            (0xF, _, 3, 3) => {
                let number = dig2 as f32;

                let hundreds = (number / 100.0).floor() as u8;
                let tens = ((number / 10.0) % 10.0).floor() as u8;
                let ones = (number % 10.0) as u8;

                self.memory[self.i_reg as usize] = hundreds;
                self.memory[(self.i_reg as usize) + 1] = tens;
                self.memory[(self.i_reg as usize) + 2] = ones;
            },

            //LD [I], Vx
            (0xF, _, 5, 5) => {
                let x = dig2 as usize;
                let i = self.i_reg as usize;

                for index in 0..=x {
                    self.memory[i + index] = self.v_reg[index];
                }
            },

            //LD Vx, [I]
            (0xF, _, 6, 5) => {
                let x = dig2 as usize;
                let i = self.i_reg as usize;

                for index in 0..=x {
                    self.v_reg[index] = self.memory[i + index];
                }
            },

            //Panic
            (_, _, _, _) => unimplemented!("Unimplemented opcode: {:x}", op),
        }
    }
}