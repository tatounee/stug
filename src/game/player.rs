use crate::{draw::sprite::Sprite, physic::vec2::Vec2};

pub struct Player {
    coords: Vec2,
    sprite: Sprite,
}

impl Player {
    #[inline]
    pub const fn coords(&self) -> Vec2 {
        self.coords
    }

    #[inline]
    pub const fn sprite(&self) -> &Sprite {
        &self.sprite
    }

    pub fn new() -> Self {
        let sprite = Sprite::dbg_player_sprite();
        Self {
            coords: Vec2(10., 10.),
            sprite,
        }
    }
}
