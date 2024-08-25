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
        Full, Sides, Sides, Sides, Full, // 0
        Right, Right, Right, Right, Right, // 1
        Full, Right, Full, Left, Full, // 2
        Full, Right, Full, Right, Full, // 3
        Sides, Sides, Full, Right, Right, // 4
        Full, Left, Full, Right, Full, // 5
        Full, Left, Full, Sides, Full, // 6
        Full, Right, Right, Right, Right, // 7
        Full, Sides, Full, Sides, Full, // 8
        Full, Sides, Full, Right, Full, // 9
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
