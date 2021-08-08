use std::fmt::Formatter;

pub trait RgbDevice
{
    fn set_color(&mut self, color: Color);
    fn set_mode(&mut self, mode: RgbMode);
    fn set_speed(&mut self, speed: RgbSpeed);
    fn set_brightness(&mut self, brightness: RgbBrightness);
    fn get_name(&self) -> &String;
    fn display(&mut self);
}



#[allow(dead_code)]
pub enum RgbBrightness
{
    Level10,
    Level20,
    Level30,
    Level40,
    Level50,
    Level60,
    Level70,
    Level80,
    Level90,
    Level100,
}

#[allow(dead_code)]
pub enum RgbSpeed
{
    Slow,
    Medium,
    Fast
}
pub enum RgbMode
{
    Static,
}



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