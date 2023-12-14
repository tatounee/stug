use std::io::{self, Write};

use crossterm::{cursor::MoveTo, queue, style::Print};

use crate::physic::vec2::{UVec2, Vec2};

use super::sprite::{self, Sprite};

pub struct Window {
    offset: UVec2,
    size: UVec2,
}

impl Window {
    pub fn full_size_window() -> io::Result<Self> {
        let size = crossterm::terminal::size()?;
        let size = UVec2(size.0 as usize, size.1 as usize);

        Ok(Self {
            offset: UVec2::default(),
            size,
        })
    }

    pub fn draw_sprite(&self, coords: Vec2, sprite: &Sprite) -> io::Result<()> {
        let mut stdout = io::stdout();
        let Vec2(x_pos, y_pos) = coords;

        for (y_sprite, line) in sprite.sprite().iter().enumerate() {
            for (x_sprite, character) in line.iter().enumerate() {
                let x = self.offset.0 + x_pos.round() as usize + x_sprite - sprite.anchor().0;
                let y = self.offset.1 + y_pos.round() as usize + y_sprite - sprite.anchor().1;

                if *character != sprite::EMPTY {
                    queue!(stdout, MoveTo(x as u16, y as u16), Print(character))?;
                }
            }
        }

        stdout.flush()?;

        Ok(())
    }

    pub fn draw_sprites(sprites: &[(Vec2, Sprite)]) -> io::Result<()> {
        let mut stdout = io::stdout();

        for (Vec2(x_pos, y_pos), sprite) in sprites {
            for (y_sprite, line) in sprite.sprite().iter().enumerate() {
                for (x_sprite, character) in line.iter().enumerate() {
                    let x = x_pos.round() as usize + x_sprite - sprite.anchor().0;
                    let y = y_pos.round() as usize + y_sprite - sprite.anchor().1;

                    queue!(stdout, MoveTo(x as u16, y as u16), Print(character))?;
                }
            }
        }

        stdout.flush()?;

        Ok(())
    }
}
