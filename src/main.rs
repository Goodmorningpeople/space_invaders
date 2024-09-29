use std::{
    error::Error,
    io::{self, stdout},
    sync::mpsc::{self, channel},
    thread,
    time::{Duration, Instant},
};

use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use rusty_audio::Audio;
use space_invaders::{
    frame::{self, new_frame, Drawable},
    invaders::{self, Invaders},
    player::Player,
    render::{self, render},
    shot::Shot,
    NUM_COLS,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "explode.wav");
    audio.add("lose", "lose.wav");
    audio.add("move", "move.wav");
    audio.add("startup", "startup.wav");
    audio.add("pew", "pew.wav");
    audio.add("win", "win.wav");
    audio.play("startup");

    // Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Render loop in a seperate thread
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    'mainloop: loop {
        let mut player = Player::new();
        let mut invaders = Invaders::new(2000, NUM_COLS - 2, 9);
        let mut instant = Instant::now();
        // Game loop
        'gameloop: loop {
            // Per-frame init
            let delta = instant.elapsed();
            instant = Instant::now();
            let mut curr_frame = new_frame();

            // Input
            while event::poll(Duration::default())? {
                if let Event::Key(key_event) = event::read()? {
                    match key_event.code {
                        KeyCode::Esc | KeyCode::Char('q') => {
                            audio.play("lose");
                            break 'gameloop;
                        }
                        KeyCode::Left => player.move_left(),
                        KeyCode::Right => player.move_right(),
                        KeyCode::Char(' ') | KeyCode::Enter => {
                            if player.shoot() {
                                audio.play("pew");
                            }
                        }
                        KeyCode::Char('e') => {
                            if player.pierce() {
                                audio.play("pew");
                            }
                        }
                        _ => {}
                    }
                }
            }
            // Updates
            player.update(delta);
            if invaders.update(delta) {
                audio.play("move");
            }
            if player.detect_hits(&mut invaders) {
                audio.play("explode");
            }
            if player.detect_pierce(&mut invaders) {
                audio.play("explode");
            }

            // Draw & render
            player.draw(&mut curr_frame);
            invaders.draw(&mut curr_frame);
            let _ = render_tx.send(curr_frame);
            thread::sleep(Duration::from_millis(1));

            // Win or lose?
            if invaders.all_killed() {
                audio.play("win");
                break 'gameloop;
            }
            if invaders.reached_bottom() {
                audio.play("lose");
                break 'gameloop;
            }
        }
        'menuloop: loop {
            while event::poll(Duration::default())? {
                if let Event::Key(key_event) = event::read()? {
                    match key_event.code {
                        KeyCode::Char(' ') | KeyCode::Enter => {
                            audio.play("startup");
                            break 'menuloop;
                        }
                        KeyCode::Char('q') | KeyCode::Esc => break 'mainloop,
                        _ => {}
                    }
                }
            }
        }
    }
    // Cleanup
    drop(render_tx);
    render_handle.join().unwrap();
    audio.wait();
    stdout.execute(LeaveAlternateScreen)?;
    stdout.execute(Show)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
