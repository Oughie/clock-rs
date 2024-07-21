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
        let color_bg = color.background();
        let reset = Color::Reset.background();

        match self {
            Self::Full => format!("{color_bg}      {reset} "),
            Self::Left => format!("{color_bg}  {reset}     "),
            Self::Center => format!(" {color_bg}  {reset}  "),
            Self::Right => format!("    {color_bg}  {reset} "),
            Self::Sides => format!("{color_bg}  {reset}  {color_bg}  {reset} "),
            Self::Empty => String::from("     "),
        }
    }
}
