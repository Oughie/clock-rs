use crate::{
    color::Color,
    segment::Segment::{self, *},
};

pub enum Character {
    Num(u32),
    Colon,
    Empty,
}

impl Character {
    const COLON: [Segment; 5] = [Empty, Center, Empty, Center, Empty];
    const NUMBERS: [Segment; 50] = [
        Full, Sides, Sides, Sides, Full, Right, Right, Right, Right, Right, Full, Right, Full,
        Left, Full, Full, Right, Full, Right, Full, Sides, Sides, Full, Right, Right, Full, Left,
        Full, Right, Full, Full, Left, Full, Sides, Full, Full, Right, Right, Right, Right, Full,
        Sides, Full, Sides, Full, Full, Sides, Full, Right, Full,
    ];

    fn segment(&self, row: usize) -> &Segment {
        match self {
            Self::Num(n) => &Self::NUMBERS[*n as usize * 5 + row],
            Self::Colon => &Self::COLON[row],
            Self::Empty => &Empty,
        }
    }

    pub fn fmt(&self, color: &Color, row: usize) -> String {
        self.segment(row).fmt(color)
    }
}
