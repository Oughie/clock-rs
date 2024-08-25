use crate::color::Color;

pub enum Segment {
    Full,
    Left,
    Center,
    Right,
    Sides,
    Empty,
}

impl Segment {
    pub fn fmt(&self, color: &Color) -> String {
        let color = color.background();
        let reset = Color::RESET;

        match self {
            Self::Full => format!("{color}      {reset} "),
            Self::Left => format!("{color}  {reset}     "),
            Self::Center => format!(" {color}  {reset}  "),
            Self::Right => format!("    {color}  {reset} "),
            Self::Sides => format!("{color}  {reset}  {color}  {reset} "),
            Self::Empty => "     ".into(),
        }
    }
}
