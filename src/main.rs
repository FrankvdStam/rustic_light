mod color;
mod z390;
mod rtx2080;
mod animation;

use rtx2080::Rtx2080;
use crate::color::RgbDevice;
use std::thread::sleep;
use std::time::{Duration, SystemTime};





fn main()
{
    let start = SystemTime::now();

    let mut rgb_devices: Vec<Box<dyn RgbDevice>> = z390::get_z390_rgb_devices();
    rgb_devices.push(Box::new(Rtx2080::new()));

    loop
    {
        let millis = SystemTime::now().duration_since(start).unwrap().as_millis();

        animation::color_spectrum(&mut rgb_devices, millis);

        for d in rgb_devices.iter_mut()
        {
            d.display();
        }

        sleep(Duration::from_millis(100));
    }
}

