use std::time::Duration;

use rusty_time::timer::Timer;

use crate::frame::{self, Drawable, Frame};

pub struct Piercer {
    pub x: usize,
    pub y: usize,
    timer: Timer,
    pub exploding: bool,
}

impl Piercer {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            timer: Timer::from_millis(150),
            exploding: false,
        }
    }
    pub fn update(&mut self, delta: Duration) {
        self.timer.update(delta);
        if self.timer.ready {
            if self.y > 0 {
                self.y -= 1;
            }
            self.timer.reset();
        }
    }
    pub fn explode(&mut self) {
        self.exploding = true;
        self.timer = Timer::from_millis(250);
    }
    pub fn dead(&self) -> bool {
        (self.timer.ready) || (self.y == 0)
    }
}

impl Drawable for Piercer {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = if self.exploding { "*" } else { "O" }
    }
}
