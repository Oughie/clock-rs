use crate::segment::Segment::{self, *};

pub enum Character {
    Num(u32),
    Colon,
}

impl Character {
    pub const CHARACTERS: [Segment; 55] = [
        Full, Sides, Sides, Sides, Full, Right, Right, Right, Right, Right, Full, Right, Full,
        Left, Full, Full, Right, Full, Right, Full, Sides, Sides, Full, Right, Right, Full, Left,
        Full, Right, Full, Full, Left, Full, Sides, Full, Full, Right, Right, Right, Right, Full,
        Sides, Full, Sides, Full, Full, Sides, Full, Right, Full, Empty, Center, Empty, Center,
        Empty,
    ];

    pub fn segment(&self, row: usize) -> Segment {
        match self {
            Self::Num(n) => Self::CHARACTERS[*n as usize * 5 + row],
            Self::Colon => Self::CHARACTERS[50 + row],
        }
    }
}
