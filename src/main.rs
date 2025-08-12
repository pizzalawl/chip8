use std::env;
use raylib::prelude::*;

use crate::processor::{Chip8, SCALE, SCREEN_HEIGHT, SCREEN_WIDTH};

pub mod font;
pub mod processor;

fn main(){
    let args: Vec<String> = env::args().collect();

    let mut emulator = processor::Chip8::new();
    emulator.load_file(&args[1]);

    let (mut rl, thread) = raylib::init().size((SCREEN_WIDTH * SCALE) as i32, (SCREEN_HEIGHT * SCALE) as i32).title("Hello, World").build();
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
         
        d.clear_background(Color::WHITE);

        for x in 0..SCREEN_WIDTH {
            for y in 0..SCREEN_HEIGHT {
                let idx = y * SCREEN_WIDTH + x;
                if emulator.display[idx] == true {
                    d.draw_rectangle( (x * SCALE) as i32, (y * SCALE) as i32, SCALE as i32, SCALE as i32, Color::BLACK);
                }
            }
        }

        emulator.tick();
        emulator.tick_timers();
    }
}