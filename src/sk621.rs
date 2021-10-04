use cooler_master_sdk::CoolerMasterDevice;
use cooler_master_sdk::ffi::DeviceIndex;
use crate::color::{Color, RgbDevice, RgbMode, RgbSpeed, RgbBrightness};
use std::thread::sleep;
use std::time::Duration;

pub struct Sk621
{
    name: String,
    color: Color,
    device: CoolerMasterDevice,
}

impl Sk621
{
    pub fn new() -> Self
    {

        let mut sk621 = Sk621
        {
            name: "sk621".to_string(),
            color: Color::new(0, 0, 0),
            device: CoolerMasterDevice::new(DeviceIndex::SK621),
        };

        //Constructing the CoolerMasterDevice will attempt to claim software control over the keyboard
        //However, it might fail - especially during startup of this service it might not be initialized yet
        while
            match sk621.device.set_led_control(true)
            {
                Ok(_) => false,
                Err(_) => true
            }
        {
            sleep(Duration::from_secs(1));
        }

        return sk621;
    }
}

//Sk621 an obviously set the rgb lighting per-key. I don't think it's a good idea to turn each key into an rgb device.
impl RgbDevice for Sk621
{
    fn set_color(&mut self, color: Color)
    {
        self.color = color;
    }

    //Not supported:
    fn set_mode(&mut self, _mode: RgbMode){}
    fn set_speed(&mut self, _speed: RgbSpeed){}
    fn set_brightness(&mut self, _brightness: RgbBrightness){}

    fn get_name(&self) -> &String
    {
        return &self.name;
    }
    fn display(&mut self)
    {
        self.device.set_full_color(self.color.r, self.color.g, self.color.b);
    }
}