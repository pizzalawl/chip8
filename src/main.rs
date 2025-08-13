use std::env;

pub mod font;
pub mod processor;
pub mod display;

fn main(){
    let args: Vec<String> = env::args().collect();

    let mut emulator = processor::Chip8::new();
    emulator.load_file(&args[1]);

    let mut screen = display::Display::new();

    while !screen.should_close() {
        emulator.update_keys(screen.get_inputs());
        emulator.tick();
        emulator.tick_timers();
        screen.draw(&emulator.get_display());
    }
}