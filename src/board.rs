use esp_idf_hal::{gpio::Pins, units::Hertz};
use esp_idf_sys::camera;

mod esp32cam;
pub use esp32cam::ESP32CAM;

use crate::{CameraModel, FrameSize, ImageFormat as _};

pub trait PinLayout<Model> {
    fn pin_config(pins: &Pins) -> PinConfig;
}

pub struct Config<Model>
where
    Model: CameraModel,
{
    pub image_format: Model::ImageFormat,
    pub frame_size: Model::FrameSize,
    pub pin_configuration: PinConfig,
    pub xclk_frequency: Hertz,
    pub frame_buffer: FrameBufferConfig,
    pub led_control: LEDControl,
}

impl<Model: CameraModel> Config<Model> {
    pub fn from_pins<Board: PinLayout<Model>>(pins: &Pins) -> Self {
        Self {
            image_format: Model::ImageFormat::default(),
            frame_size: Model::FrameSize::default(),
            pin_configuration: Board::pin_config(pins),
            xclk_frequency: Model::XCLK_DEFAULT_FREQUENCY,
            frame_buffer: FrameBufferConfig::default(),
            led_control: LEDControl::default(),
        }
    }

    pub(crate) fn into_config(self) -> camera::camera_config_t {
        camera::camera_config_t {
            pin_pwdn: self.pin_configuration.pwdn,
            pin_xclk: self.pin_configuration.xclk,
            pin_reset: 0xff,

            pin_d0: self.pin_configuration.d0,
            pin_d1: self.pin_configuration.d1,
            pin_d2: self.pin_configuration.d2,
            pin_d3: self.pin_configuration.d3,
            pin_d4: self.pin_configuration.d4,
            pin_d5: self.pin_configuration.d5,
            pin_d6: self.pin_configuration.d6,
            pin_d7: self.pin_configuration.d7,
            pin_vsync: self.pin_configuration.vsync,
            pin_href: self.pin_configuration.href,
            pin_pclk: self.pin_configuration.pclk,

            xclk_freq_hz: self.xclk_frequency.0 as i32,
            ledc_timer: self.led_control.timer,
            ledc_channel: self.led_control.channel,

            pixel_format: self.image_format.pixel_format(),
            frame_size: self.frame_size.frame_size(),

            jpeg_quality: self.image_format.quality(),
            fb_count: self.frame_buffer.count,
            grab_mode: self.frame_buffer.grab_mode as u32,

            fb_location: self.frame_buffer.location as u32,

            __bindgen_anon_1: camera::camera_config_t__bindgen_ty_1 {
                pin_sccb_sda: self.pin_configuration.sda,
            },
            __bindgen_anon_2: camera::camera_config_t__bindgen_ty_2 {
                pin_sccb_scl: self.pin_configuration.scl,
            },

            ..Default::default()
        }
    }
}

pub struct PinConfig {
    pub pwdn: i32,
    pub xclk: i32,
    pub reset: i32,
    pub d0: i32,
    pub d1: i32,
    pub d2: i32,
    pub d3: i32,
    pub d4: i32,
    pub d5: i32,
    pub d6: i32,
    pub d7: i32,
    pub vsync: i32,
    pub href: i32,
    pub pclk: i32,
    pub sda: i32,
    pub scl: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct FrameBufferConfig {
    pub location: FrameBufferLocation,
    pub grab_mode: GrabMode,
    pub count: usize,
}

impl Default for FrameBufferConfig {
    fn default() -> Self {
        Self {
            count: 1,
            grab_mode: GrabMode::default(),
            location: FrameBufferLocation::default(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LEDControl {
    pub timer: u32,
    pub channel: u32,
}

impl Default for LEDControl {
    fn default() -> Self {
        Self {
            timer: camera::ledc_timer_t_LEDC_TIMER_0,
            channel: camera::ledc_channel_t_LEDC_CHANNEL_0,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(u32)]
pub enum GrabMode {
    #[default]
    WhenEmpty = camera::camera_grab_mode_t_CAMERA_GRAB_WHEN_EMPTY,
    Latest = camera::camera_grab_mode_t_CAMERA_GRAB_LATEST,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(u32)]
pub enum FrameBufferLocation {
    #[default]
    PSRAM = camera::camera_fb_location_t_CAMERA_FB_IN_PSRAM,
    DRAM = camera::camera_fb_location_t_CAMERA_FB_IN_DRAM,
}
