use esp_idf_hal::units::Hertz;
use esp_idf_sys::{camera, esp, EspError};

use super::{CameraModel, CameraSensor, FrameSize, ImageFormat, Setting};

pub struct Ov2640;

impl CameraModel for Ov2640 {
    type FrameSize = Ov2640FrameSize;
    type ImageFormat = Ov2640ImageFormat;
    const XCLK_DEFAULT_FREQUENCY: Hertz = Hertz(20_000_000);
}

#[derive(Debug, Clone, Copy)]
pub enum Ov2640ImageFormat {
    Grayscale,
    JPEG { quality: i32 },
    RGB565,
    YUV422,
}

impl Default for Ov2640ImageFormat {
    fn default() -> Self {
        Ov2640ImageFormat::JPEG { quality: 12 }
    }
}

impl ImageFormat for Ov2640ImageFormat {
    fn pixel_format(&self) -> u32 {
        match self {
            Ov2640ImageFormat::JPEG { .. } => camera::pixformat_t_PIXFORMAT_JPEG,
            Ov2640ImageFormat::Grayscale => camera::pixformat_t_PIXFORMAT_GRAYSCALE,
            Ov2640ImageFormat::RGB565 => camera::pixformat_t_PIXFORMAT_RGB565,
            Ov2640ImageFormat::YUV422 => camera::pixformat_t_PIXFORMAT_YUV422,
        }
    }

    fn quality(&self) -> i32 {
        match self {
            Ov2640ImageFormat::JPEG { quality } => *quality,
            _ => 0,
        }
    }
}

impl Setting<Ov2640> for Ov2640ImageFormat {
    fn apply<'a>(&self, sensor: &mut CameraSensor<'a, Ov2640>) -> Result<(), EspError> {
        match self {
            Ov2640ImageFormat::JPEG { quality } => {
                esp!(unsafe {
                    (*sensor.sensor).set_pixformat.unwrap()(
                        sensor.sensor,
                        camera::pixformat_t_PIXFORMAT_JPEG,
                    )
                })?;
                esp!(unsafe { (*sensor.sensor).set_quality.unwrap()(sensor.sensor, *quality) })
            }
            format => {
                esp!(unsafe {
                    (*sensor.sensor).set_pixformat.unwrap()(sensor.sensor, format.pixel_format())
                })
            }
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, Default)]
#[repr(u32)]
pub enum Ov2640FrameSize {
    /// 320x240
    QVGA = camera::framesize_t_FRAMESIZE_QVGA,
    /// 352x288
    CIF = camera::framesize_t_FRAMESIZE_CIF,
    /// 640x480
    VGA = camera::framesize_t_FRAMESIZE_VGA,
    /// 800x600
    SVGA = camera::framesize_t_FRAMESIZE_SVGA,
    /// 1024x768
    XGA = camera::framesize_t_FRAMESIZE_XGA,
    /// 1280x1024
    SXGA = camera::framesize_t_FRAMESIZE_SXGA,
    #[default]
    /// 1600x1200
    UXGA = camera::framesize_t_FRAMESIZE_UXGA,
}

impl FrameSize for Ov2640FrameSize {
    fn frame_size(&self) -> u32 {
        *self as u32
    }
}

impl Setting<Ov2640> for Ov2640FrameSize {
    fn apply<'a>(&self, sensor: &mut CameraSensor<'a, Ov2640>) -> Result<(), EspError> {
        esp!(unsafe { (*sensor.sensor).set_framesize.unwrap()(sensor.sensor, *self as u32) })
    }
}
