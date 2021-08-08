use nvapi::sys::handles::NvPhysicalGpuHandle;
use nvapi::sys::i2c::NV_I2C_INFO_V3;
use nvapi::sys::nvapi_QueryInterface;
use nvapi::Status;
use crate::color::{Color, RgbMode, RgbSpeed, RgbDevice, RgbBrightness};


//================================================================================================================================================================================================
//Fusion RGB / NVAPI constants

const NVAPI_IC2_WRITE_EX_ADDRESS: u32 = 0x283AC65A;
const NVAPI_IC2_READ_EX_ADDRESS: u32 = 0x4D7B0709;

const RGB_FUSION_LED_COLOR_ADDRESS: u8 = 0x40;
const RGB_FUSION_MODE_SPEED_ADDRESS: u8 = 0x88;

type FnNvapiIc2WriteExType = extern "C" fn(NvPhysicalGpuHandle, *mut NV_I2C_INFO_V3, *mut u32) -> nvapi::Status;
type FnNvapiIc2ReadExType = extern "C" fn(NvPhysicalGpuHandle, *mut NV_I2C_INFO_V3, *mut u32) -> nvapi::Status;

#[allow(dead_code)]
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum RgbFusionMode
{
    Static        = 0x01,
    Breathing     = 0x02,
    Flashing      = 0x04,
    DualFlashing  = 0x08,
    SpectrumCycle = 0x11,
}

#[allow(dead_code)]
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum RgbFusionSpeed
{
    Slowest = 0x00,
    Normal  = 0x05,
    Fastest = 0x09
}




//================================================================================================================================================================================================
//Rtx2080 RgbDevice

pub struct Rtx2080
{
    name                : String,
    nvapi_ic2_write_ex  : FnNvapiIc2WriteExType,
    #[allow(dead_code)]
    nvapi_ic2_read_ex   : FnNvapiIc2ReadExType,
    handle              : NvPhysicalGpuHandle,

    color               : Color,
    mode                : RgbMode,
    speed               : RgbSpeed,
}

impl Rtx2080
{
    pub fn new() -> Self
    {
        nvapi::initialize().unwrap();

        let gpus = nvapi::PhysicalGpu::enumerate().unwrap();
        let gpu = gpus.first().unwrap();
        let name = gpu.full_name().unwrap();
        let handle = *gpu.handle();

        //Query the address of extended ic2 read/write functions
        let fn_nvapi_ic2_write_ex_address = nvapi_QueryInterface(NVAPI_IC2_WRITE_EX_ADDRESS).unwrap(); //read ex
        let fn_nvapi_ic2_read_ex_address = nvapi_QueryInterface(NVAPI_IC2_READ_EX_ADDRESS).unwrap(); //write ex

        //Convert the addresses into callable functions
        let nvapi_ic2_write_ex: FnNvapiIc2WriteExType = unsafe { std::mem::transmute(fn_nvapi_ic2_write_ex_address as *const ()) };
        let nvapi_ic2_read_ex: FnNvapiIc2ReadExType = unsafe { std::mem::transmute(fn_nvapi_ic2_read_ex_address as *const ()) };

        Rtx2080
        {
            name,
            handle,
            nvapi_ic2_write_ex,
            nvapi_ic2_read_ex,

            color: Color::new(0,0,0),
            mode: RgbMode::Static,
            speed: RgbSpeed::Slow,
        }
    }

    fn write(&self, data: [u8; 4])
    {
        let mut data_buffer = data.clone();

        let mut ic2_data =  nvapi::sys::i2c::NV_I2C_INFO::zeroed();
        ic2_data.version = nvapi::sys::i2c::NV_I2C_INFO_VER3;
        ic2_data.i2cDevAddress = 0x47 << 1;
        ic2_data.pbData = data_buffer.as_mut_ptr();
        ic2_data.cbSize = 4;

        ic2_data.bIsDDCPort = 0;
        ic2_data.portId = 1;
        ic2_data.bIsPortIdSet = 1;

        ic2_data.i2cSpeed = 0xFFFF;
        ic2_data.i2cSpeedKhz = nvapi::sys::i2c::NVAPI_I2C_SPEED_DEFAULT;

        let mut data_buf2 = [0; 2];

        match (self.nvapi_ic2_write_ex)(self.handle, &mut ic2_data, data_buf2.as_mut_ptr())
        {
            Status::Ok => return,
            status => panic!("nvapi_ic2_write_ex error: {}", status)
        }
    }
}


impl RgbDevice for Rtx2080
{
    fn set_color(&mut self, color: Color)
    {
        self.color = color;
    }

    fn set_mode(&mut self, mode: RgbMode)
    {
        self.mode = mode;
    }

    fn set_speed(&mut self, speed: RgbSpeed)
    {
        self.speed = speed;
    }

    fn set_brightness(&mut self, _brightness: RgbBrightness) { /* rtx 2080 does not seem to support brightness */ }

    fn get_name(&self) -> &String
    {
        return &self.name;
    }

    fn display(&mut self)
    {
        //Write the color
        let mut data_buffer =
        [
            RGB_FUSION_LED_COLOR_ADDRESS,
            self.color.r,
            self.color.g,
            self.color.b,
        ];
        self.write(data_buffer);

        //Convert enum types to the rtx2080 specific ones, write the mode and speed.
        let rgb_fusion_mode = match self.mode
        {
            RgbMode::Static => RgbFusionMode::Static,
        };

        let rgb_fusion_speed = match self.speed
        {
            RgbSpeed::Slow => RgbFusionSpeed::Slowest,
            RgbSpeed::Medium => RgbFusionSpeed::Normal,
            RgbSpeed::Fast => RgbFusionSpeed::Fastest,
        };

        data_buffer =
        [
            RGB_FUSION_MODE_SPEED_ADDRESS,
            rgb_fusion_mode as u8,
            rgb_fusion_speed as u8,
            0x63,
        ];
        self.write(data_buffer);
    }
}
