//! Camera Sensor Configuration

// https://randomnerdtutorials.com/esp32-cam-ov2640-camera-settings/
// https://heyrick.eu/blog/index.php?diary=20210418

pub mod ov2640;

use std::marker::PhantomData;

use esp_idf_hal::units::Hertz;
use esp_idf_sys::*;

pub trait CameraModel: Sized {
    type FrameSize: Setting<Self> + FrameSize;
    type ImageFormat: Setting<Self> + ImageFormat;
    const XCLK_DEFAULT_FREQUENCY: Hertz;
}

pub trait Setting<Model>: Default {
    fn apply<'a>(&self, sensor: &mut CameraSensor<'a, Model>) -> Result<(), EspError>;
}

pub trait ImageFormat {
    fn pixel_format(&self) -> u32;
    fn quality(&self) -> i32;
}

pub trait FrameSize {
    fn frame_size(&self) -> u32;
}

pub struct CameraSensor<'a, Model> {
    sensor: *mut camera::sensor_t,
    _p: PhantomData<(&'a camera::sensor_t, Model)>,
}

impl<'a, Model> CameraSensor<'a, Model> {
    pub fn set(&mut self, setting: &impl Setting<Model>) -> Result<(), EspError> {
        setting.apply(self)?;
        Ok(())
    }
}

impl<'a, Model> CameraSensor<'a, Model>
where
    Model: CameraModel,
{
    pub fn set_image_format(
        &mut self,
        format: &<Model as CameraModel>::ImageFormat,
    ) -> Result<(), EspError> {
        self.set(format)
    }

    pub fn set_frame_size(
        &mut self,
        frame_size: &<Model as CameraModel>::FrameSize,
    ) -> Result<(), EspError> {
        self.set(frame_size)
    }
}

impl<'a, Model> Default for CameraSensor<'a, Model> {
    fn default() -> Self {
        CameraSensor {
            sensor: unsafe { camera::esp_camera_sensor_get() },
            _p: PhantomData,
        }
    }
}

/*
impl<Model, 'a> CameraSensor<Model, 'a> {
    pub fn set(&mut self, setting: &impl Setting) -> Result<(), EspError> {
        setting.apply(self)?;
        Ok(())
    }
}


#[derive(Debug, Clone, Copy, Default)]
pub enum Brightness {
    Lowest = -2,
    Low = -1,
    #[default]
    Normal = 0,
    High = 1,
    Highest = 2,
}

impl<Model> Setting<Model> for Brightness
where
    Model: CameraModel,
{
    fn apply<'a>(&self, sensor: &mut CameraSensor<Model, 'a>) -> Result<(), EspError> {
        esp!(unsafe { (*sensor.sensor).set_brightness.unwrap()(sensor.sensor, *self as i32) })
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Contrast {
    Lowest = -2,
    Low = -1,
    #[default]
    Normal = 0,
    High = 1,
    Highest = 2,
}

impl<Model: CameraModel> Setting<Model> for Contrast {
    fn apply<'a>(&self, sensor: &mut CameraSensor<Model, 'a>) -> Result<(), EspError> {
        esp!(unsafe { (*sensor.sensor).set_contrast.unwrap()(sensor.sensor, *self as i32) })
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Saturation {
    Lowest = -2,
    Low = -1,
    #[default]
    Normal = 0,
    High = 1,
    Highest = 2,
}

impl<Model: CameraModel> Setting<Model> for Saturation {
    fn apply<'a>(&self, sensor: &mut CameraSensor<Model, 'a>) -> Result<(), EspError> {
        esp!(unsafe { (*sensor.sensor).set_saturation.unwrap()(sensor.sensor, *self as i32) })
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Effect {
    #[default]
    None = 0,
    Negative = 1,
    Grayscale = 2,
    RedTint = 3,
    GreenTint = 4,
    BlueTint = 5,
    Sepia = 6,
}

impl<Model> Setting<Model> for Effect
where
    Model: CameraModel,
{
    fn apply<'a>(&self, sensor: &mut CameraSensor<Model, 'a>) -> Result<(), EspError> {
        esp!(unsafe { (*sensor.sensor).set_special_effect.unwrap()(sensor.sensor, *self as i32) })
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum WhiteBalanceMode {
    #[default]
    Auto = 0,
    Sunny = 1,
    Cloudy = 2,
    Office = 3,
    Home = 4,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum WhiteBalance {
    #[default]
    Disabled,
    Enabled {
        mode: WhiteBalanceMode,
        gain: bool,
    },
}

impl<Model> Setting<Model> for WhiteBalance
where
    Model: CameraModel,
{
    fn apply<'a>(&self, sensor: &mut CameraSensor<Model, 'a>) -> Result<(), EspError> {
        match self {
            WhiteBalance::Disabled => {
                esp!(unsafe { (*sensor.sensor).set_whitebal.unwrap()(sensor.sensor, 0) })?;
            }
            WhiteBalance::Enabled { mode, gain } => {
                esp!(unsafe { (*sensor.sensor).set_whitebal.unwrap()(sensor.sensor, 1) })?;
                esp!(unsafe {
                    (*sensor.sensor).set_wb_mode.unwrap()(sensor.sensor, *mode as i32)
                })?;
                esp!(unsafe {
                    (*sensor.sensor).set_awb_gain.unwrap()(sensor.sensor, *gain as i32)
                })?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Level {
    Lowest = -2,
    Low = -1,
    #[default]
    Normal = 0,
    High = 1,
    Highest = 2,
}

#[derive(Debug, Clone, Copy)]
pub enum Exposure {
    Auto {
        gain: Level,
        /// Whether to enable Auto Exposure Control 2 (different computation method?)
        use_control_2: bool,
    },
    Manual {
        /// 0-1200. Defaults to 300
        gain: u16,
    },
}

impl Default for Exposure {
    fn default() -> Self {
        Exposure::Auto {
            gain: Level::default(),
            use_control_2: false,
        }
    }
}

impl<Model> Setting<Model> for Exposure
where
    Model: CameraModel,
{
    fn apply<'a>(&self, sensor: &mut CameraSensor<Model, 'a>) -> Result<(), EspError> {
        match self {
            Exposure::Auto {
                gain,
                use_control_2,
            } => {
                esp!(unsafe { (*sensor.sensor).set_exposure_ctrl.unwrap()(sensor.sensor, 0) })?;
                esp!(unsafe {
                    (*sensor.sensor).set_ae_level.unwrap()(sensor.sensor, *gain as i32)
                })?;
                esp!(unsafe {
                    (*sensor.sensor).set_aec2.unwrap()(sensor.sensor, *use_control_2 as i32)
                })?;
            }
            Exposure::Manual { gain } => {
                esp!(unsafe { (*sensor.sensor).set_exposure_ctrl.unwrap()(sensor.sensor, 1) })?;
                esp!(unsafe {
                    (*sensor.sensor).set_aec_value.unwrap()(sensor.sensor, *gain as i32)
                })?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Iso {
    #[default]
    X2 = 0,
    X4 = 1,
    X8 = 2,
    X16 = 3,
    X32 = 4,
    X64 = 5,
    X128 = 6,
}

#[derive(Debug, Clone, Copy)]
pub enum Gain {
    Auto { ceiling: Iso },
    Manual { gain: u8 },
}

impl Default for Gain {
    fn default() -> Self {
        Gain::Auto { ceiling: Iso::X2 }
    }
}

impl Setting for Gain {
    fn apply<'a>(&self, sensor: &mut CameraSensor<'a>) -> Result<(), EspError> {
        match self {
            Gain::Auto { ceiling } => {
                esp!(unsafe { (*sensor.sensor).set_gain_ctrl.unwrap()(sensor.sensor, 0) })?;
                esp!(unsafe {
                    (*sensor.sensor).set_gainceiling.unwrap()(sensor.sensor, *ceiling as u32)
                })?;
            }
            Gain::Manual { gain } => {
                esp!(unsafe { (*sensor.sensor).set_gain_ctrl.unwrap()(sensor.sensor, 1) })?;
                esp!(unsafe {
                    (*sensor.sensor).set_agc_gain.unwrap()(sensor.sensor, *gain as i32)
                })?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum BlackPixelCompensation {
    #[default]
    Disabled = 0,
    Enabled = 1,
}

impl Setting for BlackPixelCompensation {
    fn apply<'a>(&self, sensor: &mut CameraSensor<'a>) -> Result<(), EspError> {
        esp!(unsafe { (*sensor.sensor).set_bpc.unwrap()(sensor.sensor, *self as i32,) })
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum WhitePixelCompensation {
    #[default]
    Disabled = 0,
    Enabled = 1,
}

impl Setting for WhitePixelCompensation {
    fn apply<'a>(&self, sensor: &mut CameraSensor<'a>) -> Result<(), EspError> {
        esp!(unsafe { (*sensor.sensor).set_wpc.unwrap()(sensor.sensor, *self as i32,) })
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum RawGammaMode {
    #[default]
    Disabled = 0,
    Enabled = 1,
}

impl Setting for RawGammaMode {
    fn apply<'a>(&self, sensor: &mut CameraSensor<'a>) -> Result<(), EspError> {
        esp!(unsafe { (*sensor.sensor).set_raw_gma.unwrap()(sensor.sensor, *self as i32,) })
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum LensCorrection {
    #[default]
    Disabled = 0,
    Enabled = 1,
}

impl Setting for LensCorrection {
    fn apply<'a>(&self, sensor: &mut CameraSensor<'a>) -> Result<(), EspError> {
        esp!(unsafe { (*sensor.sensor).set_lenc.unwrap()(sensor.sensor, *self as i32,) })
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum VerticalMirroring {
    #[default]
    Disabled = 0,
    Enabled = 1,
}

impl Setting for VerticalMirroring {
    fn apply<'a>(&self, sensor: &mut CameraSensor<'a>) -> Result<(), EspError> {
        esp!(unsafe { (*sensor.sensor).set_vflip.unwrap()(sensor.sensor, *self as i32,) })
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum HorizontalMirroring {
    #[default]
    Disabled = 0,
    Enabled = 1,
}

impl Setting for HorizontalMirroring {
    fn apply<'a>(&self, sensor: &mut CameraSensor<'a>) -> Result<(), EspError> {
        esp!(unsafe { (*sensor.sensor).set_hmirror.unwrap()(sensor.sensor, *self as i32,) })
    }
}

/// Adds colored bars across the output image.
///
/// For debugging purposes, I would assume?
#[derive(Debug, Clone, Copy, Default)]
pub enum ColorBar {
    #[default]
    Disabled = 0,
    Enabled = 1,
}

impl Setting for ColorBar {
    fn apply<'a>(&self, sensor: &mut CameraSensor<'a>) -> Result<(), EspError> {
        esp!(unsafe { (*sensor.sensor).set_colorbar.unwrap()(sensor.sensor, *self as i32) })
    }
}


impl<'a> CameraSensor<'a> {
    pub fn init_status(&self) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).init_status.unwrap()(self.sensor) })
    }
    pub fn reset(&self) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).reset.unwrap()(self.sensor) })
    }
    pub fn set_pixformat(&self, format: camera::pixformat_t) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_pixformat.unwrap()(self.sensor, format) })
    }
    pub fn set_framesize(&self, framesize: camera::framesize_t) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_framesize.unwrap()(self.sensor, framesize) })
    }
    pub fn set_contrast(&self, level: i32) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_contrast.unwrap()(self.sensor, level) })
    }
    pub fn set_brightness(&self, level: i32) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_brightness.unwrap()(self.sensor, level) })
    }
    pub fn set_saturation(&self, level: i32) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_saturation.unwrap()(self.sensor, level) })
    }
    pub fn set_sharpness(&self, level: i32) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_sharpness.unwrap()(self.sensor, level) })
    }
    pub fn set_denoise(&self, level: i32) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_denoise.unwrap()(self.sensor, level) })
    }
    pub fn set_gainceiling(&self, gainceiling: camera::gainceiling_t) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_gainceiling.unwrap()(self.sensor, gainceiling) })
    }
    pub fn set_quality(&self, quality: i32) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_quality.unwrap()(self.sensor, quality) })
    }
    pub fn set_colorbar(&self, enable: bool) -> Result<(), EspError> {
        esp!(unsafe {
            (*self.sensor).set_colorbar.unwrap()(self.sensor, if enable { 1 } else { 0 })
        })
    }
    pub fn set_whitebal(&self, enable: bool) -> Result<(), EspError> {
        esp!(unsafe {
            (*self.sensor).set_whitebal.unwrap()(self.sensor, if enable { 1 } else { 0 })
        })
    }
    pub fn set_gain_ctrl(&self, enable: bool) -> Result<(), EspError> {
        esp!(unsafe {
            (*self.sensor).set_gain_ctrl.unwrap()(self.sensor, if enable { 1 } else { 0 })
        })
    }
    pub fn set_exposure_ctrl(&self, enable: bool) -> Result<(), EspError> {
        esp!(unsafe {
            (*self.sensor).set_exposure_ctrl.unwrap()(self.sensor, if enable { 1 } else { 0 })
        })
    }
    pub fn set_hmirror(&self, enable: bool) -> Result<(), EspError> {
        esp!(unsafe {
            (*self.sensor).set_hmirror.unwrap()(self.sensor, if enable { 1 } else { 0 })
        })
    }
    pub fn set_vflip(&self, enable: bool) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_vflip.unwrap()(self.sensor, if enable { 1 } else { 0 }) })
    }
    pub fn set_aec2(&self, enable: bool) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_aec2.unwrap()(self.sensor, if enable { 1 } else { 0 }) })
    }
    pub fn set_awb_gain(&self, enable: bool) -> Result<(), EspError> {
        esp!(unsafe {
            (*self.sensor).set_awb_gain.unwrap()(self.sensor, if enable { 1 } else { 0 })
        })
    }
    pub fn set_agc_gain(&self, gain: i32) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_agc_gain.unwrap()(self.sensor, gain) })
    }
    pub fn set_aec_value(&self, gain: i32) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_aec_value.unwrap()(self.sensor, gain) })
    }
    pub fn set_special_effect(&self, effect: i32) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_special_effect.unwrap()(self.sensor, effect) })
    }
    pub fn set_wb_mode(&self, mode: i32) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_wb_mode.unwrap()(self.sensor, mode) })
    }
    pub fn set_ae_level(&self, level: i32) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_ae_level.unwrap()(self.sensor, level) })
    }
    pub fn set_dcw(&self, enable: bool) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_dcw.unwrap()(self.sensor, if enable { 1 } else { 0 }) })
    }
    pub fn set_bpc(&self, enable: bool) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_bpc.unwrap()(self.sensor, if enable { 1 } else { 0 }) })
    }
    pub fn set_wpc(&self, enable: bool) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_wpc.unwrap()(self.sensor, if enable { 1 } else { 0 }) })
    }
    pub fn set_raw_gma(&self, enable: bool) -> Result<(), EspError> {
        esp!(unsafe {
            (*self.sensor).set_raw_gma.unwrap()(self.sensor, if enable { 1 } else { 0 })
        })
    }
    pub fn set_lenc(&self, enable: bool) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_lenc.unwrap()(self.sensor, if enable { 1 } else { 0 }) })
    }
    pub fn get_reg(&self, reg: i32, mask: i32) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).get_reg.unwrap()(self.sensor, reg, mask) })
    }
    pub fn set_reg(&self, reg: i32, mask: i32, value: i32) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_reg.unwrap()(self.sensor, reg, mask, value) })
    }
    pub fn set_res_raw(
        &self,
        start_x: i32,
        start_y: i32,
        end_x: i32,
        end_y: i32,
        offset_x: i32,
        offset_y: i32,
        total_x: i32,
        total_y: i32,
        output_x: i32,
        output_y: i32,
        scale: bool,
        binning: bool,
    ) -> Result<(), EspError> {
        esp!(unsafe {
            (*self.sensor).set_res_raw.unwrap()(
                self.sensor,
                start_x,
                start_y,
                end_x,
                end_y,
                offset_x,
                offset_y,
                total_x,
                total_y,
                output_x,
                output_y,
                scale,
                binning,
            )
        })
    }
    pub fn set_pll(
        &self,
        bypass: i32,
        mul: i32,
        sys: i32,
        root: i32,
        pre: i32,
        seld5: i32,
        pclken: i32,
        pclk: i32,
    ) -> Result<(), EspError> {
        esp!(unsafe {
            (*self.sensor).set_pll.unwrap()(
                self.sensor,
                bypass,
                mul,
                sys,
                root,
                pre,
                seld5,
                pclken,
                pclk,
            )
        })
    }
    pub fn set_xclk(&self, timer: i32, xclk: i32) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).set_xclk.unwrap()(self.sensor, timer, xclk) })
    }
}

*/
