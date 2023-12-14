use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, cursor::{Hide, Show},
};
use draw::window::Window;
use game::Game;
use std::{
    io::{self},
    thread,
    time::Duration,
};

mod draw;
mod game;
mod physic;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen, Hide)?;

    run()?;
    // thread::sleep(Duration::from_secs(5));

    execute!(io::stdout(), LeaveAlternateScreen, Show)?;
    disable_raw_mode()?;
    Ok(())
}

fn run() -> io::Result<()> {
    let window = Window::full_size_window()?;

    let game = Game::new(window);
    game.draw()?;

    thread::sleep(Duration::from_secs(5));

    Ok(())
}
