use raylib::prelude::*;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
pub const SCALE: usize = 10;

pub struct Display {
    rl: RaylibHandle,
    thread: RaylibThread
}

impl Display {
    pub fn new() -> Self {
        let (rl, thread) = raylib::init()
            .size((SCREEN_WIDTH * SCALE) as i32, (SCREEN_HEIGHT * SCALE) as i32)
            .title("Hello, World")
            .build();
        Self { rl, thread }
    }

    pub fn draw(&mut self, pixels: &[bool]) {
        let mut d = self.rl.begin_drawing(&self.thread);
        d.clear_background(Color::WHITE);

        for x in 0..SCREEN_WIDTH {
            for y in 0..SCREEN_HEIGHT {
                let idx = y * SCREEN_WIDTH + x;
                if pixels[idx] == true {
                    d.draw_rectangle( (x * SCALE) as i32, (y * SCALE) as i32, SCALE as i32, SCALE as i32, Color::BLACK);
                }
            }
        }
    }

    pub fn should_close(&self) -> bool {
        self.rl.window_should_close()
    }
}