use crate::physic::vec2::UVec2;

pub const EMPTY: char = '\0';

pub struct Sprite {
    sprite: Vec<Vec<char>>,
    anchor: UVec2,
}

impl Sprite {
    #[inline]
    pub fn sprite(&self) -> &[Vec<char>] {
        &self.sprite
    }

    #[inline]
    pub const fn anchor(&self) -> UVec2 {
        self.anchor
    }

    pub fn dbg_player_sprite() -> Self {
        let sprite = vec![vec!['o'], vec!['π']];
        let anchor = UVec2(0, 1);

        Self {
            sprite,
            anchor
        }
    }
}