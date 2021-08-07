use std::fmt::Formatter;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Color
{
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color
{
    pub fn new(r: u8, g: u8, b: u8) -> Self
    {
        Color
        {
            r,
            g,
            b,
        }
    }
}


impl std::fmt::Display for Color
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "rgb({}, {}, {})", self.r, self.g, self.b)
    }
}