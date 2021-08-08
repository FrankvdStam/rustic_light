extern crate hidapi;
use hidapi::{HidDevice, HidApi};
use crate::color::{Color, RgbMode, RgbSpeed, RgbDevice};
use std::cell::RefCell;

pub struct Z390<'a>
{
    device: HidDevice,
    data_packet: RefCell<DataPacket>,
    rgb_devices: Vec<Z390RgbDevice<'a>>,
}

impl<'a> Z390<'a>
{
    fn get_z390_rgb_devices(device: &'a HidDevice, data_packet: &'a RefCell<DataPacket>) -> Vec<Box<dyn RgbDevice>>
    {
        let mut result: Vec<Box<dyn RgbDevice>> = Vec::new();

        result.push(Box::new(Z390RgbDevice::new(device, "".to_string(), data_packet)));

        return result;
    }



    pub fn new() -> Self
    {
        let device = find_hid_device(MSI_VENDOR_ID, MPG_Z390_GAMING_PRO_CARBON);
        let data_packet = RefCell::new(DataPacket::new());




        let mut rgb_devices = Vec::new();


        let z390 = Z390
        {
            device,
            data_packet: data_packet,
            rgb_devices
        };


        //rgb_devices.push(Z390RgbDevice::new(&device, "".to_string(), &data_packet));

        return z390;
    }

    //pub fn get_rgb_devices(&self) -> Vec<Box<dyn RgbDevice>>
    //{
    //    let mut result: Vec<Box<dyn RgbDevice>> = Vec::new();
//
    //    result.push(Box::new(Z390RgbDevice::new(&self.device, "".to_string(), &self.data_packet)));
//
    //    return result;
    //}
}



struct Z390RgbDevice<'a>
{
    name                : String,
    device              : &'a HidDevice,
    data_packet         : &'a RefCell<DataPacket>,

    color               : Color,
    mode                : RgbMode,
    speed               : RgbSpeed,
}

impl<'a> Z390RgbDevice<'a>
{
    pub fn new(device: &'a HidDevice, name: String, data_packet: &'a RefCell<DataPacket>) -> Self
    {
        Z390RgbDevice
        {
            name,
            device,
            data_packet,

            color: Color::new(0,0,0),
            mode: RgbMode::Static,
            speed: RgbSpeed::Slow,
        }
    }
}

impl<'a> RgbDevice for Z390RgbDevice<'a>
{
    fn set_color(&mut self, color: Color) {
        todo!()
    }

    fn set_mode(&mut self, mode: RgbMode) {
        todo!()
    }

    fn set_speed(&mut self, speed: RgbSpeed) {
        todo!()
    }

    fn get_name(&self) -> &String {
        todo!()
    }

    fn display(&self) {
        todo!()
    }
}



















































pub struct Motherboard
{
    device: HidDevice,
}

impl Motherboard
{
    pub fn new() -> Motherboard
    {
        let device = find_hid_device(MSI_VENDOR_ID, MPG_Z390_GAMING_PRO_CARBON);

        Motherboard
        {
            device,
        }
    }

    pub fn set_color(&self, color: Color)
    {
        let mut zone_data = ZoneData::new();

        //zone_data.effect = MsiMode::Static as u8;
        zone_data.effect = MsiMode::Static as u8;
        zone_data.color = color;
        zone_data.speed_and_brightness_flags = ((MsiBrightness::Level100 as u8) << 2 ) | ((MsiSpeed::Low as u8) & 0x03);
        zone_data.color2 = color;
        zone_data.color_flags = 0;
        zone_data.padding = 0;


        let mut data_packet = DataPacket::new();
        data_packet.set_all(zone_data);
        let buffer = data_packet.to_bytes();
        //println!("{:?}", buffer);

        self.device.send_feature_report(&buffer);
    }
}


///Find a hid device matching vendor and product ID
fn find_hid_device(vendor_id: u16, product_id: u16) -> HidDevice
{
    match HidApi::new()
    {
        Ok(api) =>
            {
               // let mut devices = api.device_list();

                //for d in devices
                //{
                //    println!("{} {} {} {}",
                //             d.vendor_id(),
                //             d.product_id(),
                //             match d.manufacturer_string()
                //             {
                //                 Some(str) => str,
                //                 None => "",
                //             },
                //             match d.product_string()
                //             {
                //                 Some(str) => str,
                //                 None => "",
                //             });
                //}

                let mut devices = api.device_list();
                let result = devices.find(|d| d.vendor_id() == vendor_id && d.product_id() == product_id);
                match result
                {
                    Some(device) =>
                        {
                            //return device.clone(),
                            let device_result = device.open_device(&api);
                            match device_result
                            {
                                Ok(device) => return device,
                                Err(e) => panic!("Unable to get hid device: {}", e.to_string()),
                            }
                        },
                    None => panic!("Unable to find hid device")
                }
            },
        Err(e) => panic!("Unable to load hid devices: {}", e.to_string()),
    }
}



#[allow(dead_code)]
#[repr(u8)]
enum MsiZone
{
    None                = 0,
    JRgb1               = 1,
    JRgb2               = 2,
    JPipe1              = 3,
    JPipe2              = 4,
    JRainbow1           = 5,
    JRainbow2           = 6,
    JCorsair            = 7,
    JCorsairOuterll120  = 8,
    OnBoardLed0         = 9,
    OnBoardLed1         = 10,
    OnBoardLed2         = 11,
    OnBoardLed3         = 12,
    OnBoardLed4         = 13,
    OnBoardLed5         = 14,
    OnBoardLed6         = 15,
    OnBoardLed7         = 16,
    OnBoardLed8         = 17,
    OnBoardLed9         = 18,
    OnBoardLed10        = 19
}


#[repr(C)]
#[derive(Clone, Copy)]
struct ZoneData
{
    effect: u8,
    color: Color,
    speed_and_brightness_flags: u8,
    color2: Color,
    color_flags: u8,
    padding    : u8,
}

impl ZoneData
{
    pub fn new() -> Self
    {
        ZoneData
        {
            effect: MsiMode::Static as u8,
            color: Color::new(0,0,0),
            speed_and_brightness_flags: 0,
            color2: Color::new(0,0,0),
            color_flags: 0,
            padding: 0,
        }
    }
}



#[allow(dead_code)]
#[repr(C)]
struct DataPacket
{
    report_id            : u8,                      // Report ID
    j_rgb_1              : ZoneData,                // 1
    j_rainbow_1          : ZoneData,                // 11
    j_corsair_1          : ZoneData,                // 21
    j_corsair_outerll120 : ZoneData,                // 31
    on_board_led         : ZoneData,                // 41
    on_board_led_1       : ZoneData,                // 51
    on_board_led_2       : ZoneData,                // 61
    on_board_led_3       : ZoneData,                // 71
    on_board_led_4       : ZoneData,                // 81
    on_board_led_5       : ZoneData,                // 91
    on_board_led_6       : ZoneData,                // 101
    on_board_led_7       : ZoneData,                // 111
    on_board_led_8       : ZoneData,                // 121
    on_board_led_9       : ZoneData,                // 131
    on_board_led_10      : ZoneData,                // 141
    j_rgb_2              : ZoneData,                // 151
    save_data            : u8,                      // 161
}


#[derive(Clone, Copy)]
enum ZoneIndex
{
    JRgb1               = 1,
    JRainbow1           = 11,
    JCorsair1           = 21,
    JCorsairOuterll120  = 31,
    OnBoardLed          = 41,
    OnBoardLed1         = 51,
    OnBoardLed2         = 61,
    OnBoardLed3         = 71,
    OnBoardLed4         = 81,
    OnBoardLed5         = 91,
    OnBoardLed6         = 101,
    OnBoardLed7         = 111,
    OnBoardLed8         = 121,
    OnBoardLed9         = 131,
    OnBoardLed10        = 141,
    JRgb2               = 151,
}




impl DataPacket
{
    pub fn new() -> Self
    {
        DataPacket
        {
            report_id           : 0x52,
            j_rgb_1             : ZoneData::new(),
            j_rainbow_1         : ZoneData::new(),
            j_corsair_1         : ZoneData::new(),
            j_corsair_outerll120: ZoneData::new(),
            on_board_led        : ZoneData::new(),
            on_board_led_1      : ZoneData::new(),
            on_board_led_2      : ZoneData::new(),
            on_board_led_3      : ZoneData::new(),
            on_board_led_4      : ZoneData::new(),
            on_board_led_5      : ZoneData::new(),
            on_board_led_6      : ZoneData::new(),
            on_board_led_7      : ZoneData::new(),
            on_board_led_8      : ZoneData::new(),
            on_board_led_9      : ZoneData::new(),
            on_board_led_10     : ZoneData::new(),
            j_rgb_2             : ZoneData::new(),
            save_data           : 0,
        }
    }


    pub fn set_all(&mut self, zone_data: ZoneData)
    {
        self.j_rgb_1              = zone_data;
        self.j_rainbow_1          = zone_data;
        self.j_corsair_1          = zone_data;
        self.j_corsair_outerll120 = zone_data;
        self.on_board_led         = zone_data;
        self.on_board_led_1       = zone_data;
        self.on_board_led_2       = zone_data;
        self.on_board_led_3       = zone_data;
        self.on_board_led_4       = zone_data;
        self.on_board_led_5       = zone_data;
        self.on_board_led_6       = zone_data;
        self.on_board_led_7       = zone_data;
        self.on_board_led_8       = zone_data;
        self.on_board_led_9       = zone_data;
        self.on_board_led_10      = zone_data;
        self.j_rgb_2              = zone_data;
    }


    pub fn set_color(&mut self, color: Color)
    {
        self.j_rgb_1                .color = color;
        self.j_rainbow_1            .color = color;
        self.j_corsair_1            .color = color;
        self.j_corsair_outerll120   .color = color;
        self.on_board_led           .color = color;
        self.on_board_led_1         .color = color;
        self.on_board_led_2         .color = color;
        self.on_board_led_3         .color = color;
        self.on_board_led_4         .color = color;
        self.on_board_led_5         .color = color;
        self.on_board_led_6         .color = color;
        self.on_board_led_7         .color = color;
        self.on_board_led_8         .color = color;
        self.on_board_led_9         .color = color;
        self.on_board_led_10        .color = color;
        self.j_rgb_2                .color = color;
    }

    pub fn set_color2(&mut self, color: Color)
    {
        self.j_rgb_1.color2               = color;
        self.j_rainbow_1.color2           = color;
        self.j_corsair_1.color2           = color;
        self.j_corsair_outerll120.color2  = color;
        self.on_board_led.color2          = color;
        self.on_board_led_1.color2        = color;
        self.on_board_led_2.color2        = color;
        self.on_board_led_3.color2        = color;
        self.on_board_led_4.color2        = color;
        self.on_board_led_5.color2        = color;
        self.on_board_led_6.color2        = color;
        self.on_board_led_7.color2        = color;
        self.on_board_led_8.color2        = color;
        self.on_board_led_9.color2        = color;
        self.on_board_led_10.color2       = color;
        self.j_rgb_2.color2               = color;
    }

    pub fn to_bytes(&self) -> [u8; 162]
    {
        let mut buffer: [u8; 162] = [0; 162];

        buffer[0] = self.report_id;
        DataPacket::write_zone_data(&self.j_rgb_1, ZoneIndex::JRgb1, &mut buffer);
        DataPacket::write_zone_data(&self.j_rainbow_1, ZoneIndex::JRainbow1, &mut buffer);
        DataPacket::write_zone_data(&self.j_corsair_1, ZoneIndex::JCorsair1, &mut buffer);
        DataPacket::write_zone_data(&self.j_corsair_outerll120, ZoneIndex::JCorsairOuterll120, &mut buffer);
        DataPacket::write_zone_data(&self.on_board_led, ZoneIndex::OnBoardLed, &mut buffer);
        DataPacket::write_zone_data(&self.on_board_led_1, ZoneIndex::OnBoardLed1, &mut buffer);
        DataPacket::write_zone_data(&self.on_board_led_2, ZoneIndex::OnBoardLed2, &mut buffer);
        DataPacket::write_zone_data(&self.on_board_led_3, ZoneIndex::OnBoardLed3, &mut buffer);
        DataPacket::write_zone_data(&self.on_board_led_4, ZoneIndex::OnBoardLed4, &mut buffer);
        DataPacket::write_zone_data(&self.on_board_led_5, ZoneIndex::OnBoardLed5, &mut buffer);
        DataPacket::write_zone_data(&self.on_board_led_6, ZoneIndex::OnBoardLed6, &mut buffer);
        DataPacket::write_zone_data(&self.on_board_led_7, ZoneIndex::OnBoardLed7, &mut buffer);
        DataPacket::write_zone_data(&self.on_board_led_8, ZoneIndex::OnBoardLed8, &mut buffer);
        DataPacket::write_zone_data(&self.on_board_led_9, ZoneIndex::OnBoardLed9, &mut buffer);
        DataPacket::write_zone_data(&self.on_board_led_10, ZoneIndex::OnBoardLed10, &mut buffer);
        DataPacket::write_zone_data(&self.j_rgb_2, ZoneIndex::JRgb2, &mut buffer);
        buffer[161] = self.save_data;

        return buffer;
    }

    fn write_zone_data(zone_data: &ZoneData, zone_index: ZoneIndex, buffer: &mut [u8])
    {
        buffer[zone_index as usize + 0] = zone_data.effect;
        buffer[zone_index as usize + 1] = zone_data.color.r;
        buffer[zone_index as usize + 2] = zone_data.color.g;
        buffer[zone_index as usize + 3] = zone_data.color.b;
        buffer[zone_index as usize + 4] = zone_data.speed_and_brightness_flags;
        buffer[zone_index as usize + 5] = zone_data.color2.r;
        buffer[zone_index as usize + 6] = zone_data.color2.g;
        buffer[zone_index as usize + 7] = zone_data.color2.b;
        buffer[zone_index as usize + 8] = zone_data.color_flags;
        buffer[zone_index as usize + 9] = zone_data.padding;
    }
}





const MSI_VENDOR_ID: u16 = 0x1462;
const MPG_Z390_GAMING_PRO_CARBON: u16 = 0x7b17;




#[allow(dead_code)]
#[repr(u8)]
enum MsiMode
{
    Disable                     = 0,
    Static                      = 1,
    Breathing                   = 2,
    Flashing                    = 3,
    DoubleFlashing              = 4,
    Lightning                   = 5,
    MsiMarquee                  = 6,
    Meteor                      = 7,
    WaterDrop                   = 8,
    MsiRainbow                  = 9,
    Pop                         = 10,
    Rap                         = 11,
    Jazz                        = 12,
    Play                        = 13,
    Movie                       = 14,
    ColorRing                   = 15,
    Planetary                   = 16,
    DoubleMeteor                = 17,
    Energy                      = 18,
    Blink                       = 19,
    Clock                       = 20,
    ColorPulse                  = 21,
    ColorShift                  = 22,
    ColorWave                   = 23,
    Marquee                     = 24,
    Rainbow                     = 25,
    RainbowWave                 = 26,
    Visor                       = 27,
    Jrainbow                    = 28,
    RainbowFlashing             = 29,
    RainbowDoubleFlashing       = 30,
    Random                      = 31,
    FanControl                  = 32,
    Disable2                    = 33,
    ColorRingFlashing           = 34,
    ColorRingDoubleFlashing     = 35,
    Stack                       = 36,
    CorsairQue                  = 37,
    Fire                        = 38,
    Lava                        = 39,
}

#[allow(dead_code)]
#[repr(u8)]
enum MsiSpeed
{
    Low = 0,
    Medium = 1,
    High = 2,
}

#[allow(dead_code)]
#[repr(u8)]
enum MsiBrightness
{
    Off = 0,
    Level10 = 1,
    Level20 = 2,
    Level30 = 3,
    Level40 = 4,
    Level50 = 5,
    Level60 = 6,
    Level70 = 7,
    Level80 = 8,
    Level90 = 9,
    Level100 = 10,
}
