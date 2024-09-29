use std::time::Duration;

use crate::{
    frame::{Drawable, Frame},
    invaders::Invaders,
    piercer::{self, Piercer},
    shot::Shot,
    NUM_COLS, NUM_ROWS,
};

pub struct Player {
    x: usize,
    y: usize,
    shots: Vec<Shot>,
    piercers: Vec<Piercer>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: NUM_COLS / 2,
            y: NUM_ROWS - 1,
            shots: Vec::new(),
            piercers: Vec::new(),
        }
    }
    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }
    pub fn move_right(&mut self) {
        if self.x < NUM_COLS - 1 {
            self.x += 1;
        }
    }
    pub fn shoot(&mut self) -> bool {
        if self.shots.len() < 3 {
            self.shots.push(Shot::new(self.x, self.y - 1));
            true
        } else {
            false
        }
    }
    pub fn update(&mut self, delta: Duration) {
        for shot in self.shots.iter_mut() {
            shot.update(delta);
        }
        self.shots.retain(|shot| !shot.dead());
        for piercer in self.piercers.iter_mut() {
            piercer.update(delta);
        }
        self.piercers.retain(|piercer| !piercer.dead());
    }
    pub fn detect_hits(&mut self, invaders: &mut Invaders) -> bool {
        let mut hit_something = false;
        for shot in self.shots.iter_mut() {
            if !shot.exploding {
                if invaders.kill_invader_at(shot.x, shot.y) {
                    hit_something = true;
                    shot.explode();
                }
            }
        }
        hit_something
    }

    pub fn detect_pierce(&mut self, invaders: &mut Invaders) -> bool {
        let mut pierce_something = false;
        for piercer in self.piercers.iter_mut() {
            if invaders.kill_invader_at(piercer.x, piercer.y) {
                pierce_something = true;
            }
        }
        pierce_something
    }
    pub fn pierce(&mut self) -> bool {
        if self.piercers.len() < 1 {
            self.piercers.push(Piercer::new(self.x, self.y - 1));
            true
        } else {
            false
        }
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = "A";
        for shot in self.shots.iter() {
            shot.draw(frame);
        }
        for piercer in self.piercers.iter() {
            piercer.draw(frame);
        }
    }
}
