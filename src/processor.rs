use crate::font;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

const RAM_SIZE: usize = 4096;
const NUM_REGS: usize = 16;
const STACK_SIZE: usize = 16;
const NUM_KEYS: usize = 16;

struct Chip8 {
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
            pc: 0,
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
        self.pc = 0;
        self.i_reg = 0;
        self.sp = 0;
        self.stack = [0; STACK_SIZE];
        self.keys = [false; NUM_KEYS];
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.v_reg = [0; 16];
        self.memory[..font::FONTSET_SIZE].copy_from_slice(&font::FONTSET);
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
            //TODO - Opcode Implementations
            
            //Panic
            (_, _, _, _) => unimplemented!("Unimplemented opcode: {}", op),
        }
    }
}