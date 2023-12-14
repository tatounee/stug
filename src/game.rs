use std::io;

use crate::draw::window::Window;

mod enemy;
mod player;

use enemy::Enemy;
use player::Player;

pub struct Game {
    player: Player,
    enemies: Vec<Enemy>,
    window: Window,
}

impl Game {
    pub fn new(window: Window) -> Self {
        let player = Player::new();

        Self {
            player,
            enemies: Vec::new(),
            window,
        }
    }

    pub fn draw(&self) -> io::Result<()> {
        let coords = self.player.coords();
        let sprite = self.player.sprite();
        self.window.draw_sprite(coords, sprite)
    }
}
