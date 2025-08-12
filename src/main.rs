use std::env;

pub mod font;
pub mod processor;
fn main(){
    let args: Vec<String> = env::args().collect();
    let mut emulator = processor::Chip8::new();
    emulator.load_file(&args[1]);
    loop {
        emulator.tick();
        emulator.tick_timers();
    }
}