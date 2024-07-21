use std::fmt;

use crate::{character::Character, color::Color};

pub struct CharacterDisplay {
    color: Color,
    row: usize,
    character: Character,
}

impl CharacterDisplay {
    pub fn new(color: Color, character: Character, row: usize) -> Self {
        Self {
            color,
            character,
            row,
        }
    }
}

impl fmt::Display for CharacterDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let segment = self.character.segment(self.row);
        write!(f, "{}", segment.fmt(self.color))
    }
}
