use crate::color::Color;

#[derive(Clone, Copy)]
pub enum Segment {
    Full,
    Left,
    Center,
    Right,
    Sides,
    Empty,
}

impl Segment {
    pub fn fmt(&self, color: Color) -> String {
        let color = color.background();
        let reset = "\x1B[0m";

        match self {
            Self::Full => format!("{color}      {reset} "),
            Self::Left => format!("{color}  {reset}     "),
            Self::Center => format!(" {color}  {reset}  "),
            Self::Right => format!("    {color}  {reset} "),
            Self::Sides => format!("{color}  {reset}  {color}  {reset} "),
            Self::Empty => String::from("     "),
        }
    }
}
