// code updated from https://github.com/Kezii/esp32cam_rs/blob/master/src/espcam.rs

use std::ffi::c_int;
use std::marker::PhantomData;

use esp_idf_hal::gpio::*;
use esp_idf_hal::peripheral::Peripheral;
use esp_idf_sys::*;

impl<'a> Camera<'a> {
    pub fn new(params: &CameraParams) -> Result<Self, esp_idf_sys::EspError> {
        let config = camera::camera_config_t {
            pin_pwdn: params.power,
            pin_xclk: params.clock,
            pin_reset: -1,
            pin_d0: params.d0,
            pin_d1: params.d1,
            pin_d2: params.d2,
            pin_d3: params.d3,
            pin_d4: params.d4,
            pin_d5: params.d5,
            pin_d6: params.d6,
            pin_d7: params.d7,
            pin_vsync: params.vertical_sync,
            pin_href: params.horizontal_reference,
            pin_pclk: params.pixel_clock,

            xclk_freq_hz: 20_000_000,
            ledc_timer: esp_idf_sys::ledc_timer_t_LEDC_TIMER_0,
            ledc_channel: esp_idf_sys::ledc_channel_t_LEDC_CHANNEL_0,

            pixel_format: params.pixel_format,
            frame_size: params.frame_size,

            jpeg_quality: 12,
            fb_count: 1,
            grab_mode: camera::camera_grab_mode_t_CAMERA_GRAB_WHEN_EMPTY,

            fb_location: params.fb_location,

            __bindgen_anon_1: camera::camera_config_t__bindgen_ty_1 {
                pin_sccb_sda: params.sda,
            },
            __bindgen_anon_2: camera::camera_config_t__bindgen_ty_2 {
                pin_sccb_scl: params.scl,
            },
            ..Default::default()
        };

        esp_idf_sys::esp!(unsafe { camera::esp_camera_init(&config) })?;
        Ok(Self { _p: PhantomData })
    }

    pub fn get_framebuffer(&self) -> Option<FrameBuffer> {
        let fb = unsafe { camera::esp_camera_fb_get() };
        if fb.is_null() {
            None
        } else {
            Some(FrameBuffer {
                fb,
                _p: PhantomData,
            })
        }
    }

    pub fn sensor(&self) -> CameraSensor<'a> {
        CameraSensor {
            sensor: unsafe { camera::esp_camera_sensor_get() },
            _p: PhantomData,
        }
    }
}

impl<'a> Drop for Camera<'a> {
    fn drop(&mut self) {
        esp!(unsafe { camera::esp_camera_deinit() }).expect("error during esp_camera_deinit")
    }
}

pub struct FrameBuffer<'a> {
    fb: *mut camera::camera_fb_t,
    _p: PhantomData<&'a camera::camera_fb_t>,
}

impl<'a> FrameBuffer<'a> {
    pub fn data(&self) -> &'a [u8] {
        unsafe { std::slice::from_raw_parts((*self.fb).buf, (*self.fb).len) }
    }

    pub fn width(&self) -> usize {
        unsafe { (*self.fb).width }
    }

    pub fn height(&self) -> usize {
        unsafe { (*self.fb).height }
    }

    pub fn format(&self) -> camera::pixformat_t {
        unsafe { (*self.fb).format }
    }

    pub fn timestamp(&self) -> camera::timeval {
        unsafe { (*self.fb).timestamp }
    }

    pub fn fb_return(&self) {
        unsafe { camera::esp_camera_fb_return(self.fb) }
    }
}

impl Drop for FrameBuffer<'_> {
    fn drop(&mut self) {
        self.fb_return();
    }
}

pub struct CameraSensor<'a> {
    sensor: *mut camera::sensor_t,
    _p: PhantomData<&'a camera::sensor_t>,
}

macro_rules! define_set_function {
    ( $set_name: ident, bool) => {
        pub fn $set_name(&self, enable: bool) -> Result<(), EspError> {
            esp!(unsafe { (*self.sensor).$set_name.unwrap()(self.sensor, enable as i32) })
        }
    };
    ( $set_name: ident, $type: ty) => {
        pub fn $set_name(&self, level: $type) -> Result<(), EspError> {
            esp!(unsafe { (*self.sensor).$set_name.unwrap()(self.sensor, level) })
        }
    };
}
macro_rules! define_get_set_function {
    ($name:ident, $set_name: ident, bool) => {
        pub fn $name(&self) -> bool {
            unsafe { (*self.sensor).status.$name != 0 }
        }
        define_set_function!($set_name, bool);
    };
    ($name:ident, $set_name: ident, $type: ty) => {
        pub fn $name(&self) -> $type {
            unsafe { (*self.sensor).status.$name as $type }
        }
        define_set_function!($set_name, $type);
    };
}

impl<'a> CameraSensor<'a> {
    pub fn init_status(&self) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).init_status.unwrap()(self.sensor) })
    }
    pub fn reset(&self) -> Result<(), EspError> {
        esp!(unsafe { (*self.sensor).reset.unwrap()(self.sensor) })
    }
    define_get_set_function!(framesize, set_framesize, camera::framesize_t);
    define_set_function!(set_pixformat, camera::pixformat_t);
    define_get_set_function!(contrast, set_contrast, i32);
    define_get_set_function!(brightness, set_brightness, i32);
    define_get_set_function!(saturation, set_saturation, i32);
    define_get_set_function!(sharpness, set_sharpness, i32);
    define_get_set_function!(denoise, set_denoise, i32);
    define_get_set_function!(gainceiling, set_gainceiling, u32);
    define_get_set_function!(quality, set_quality, i32);
    define_get_set_function!(colorbar, set_colorbar, bool);
    define_set_function!(set_whitebal, bool);
    define_set_function!(set_gain_ctrl, bool);
    define_set_function!(set_exposure_ctrl, bool);
    define_get_set_function!(hmirror, set_hmirror, bool);
    define_get_set_function!(vflip, set_vflip, bool);
    define_get_set_function!(aec2, set_aec2, bool);
    define_get_set_function!(awb_gain, set_awb_gain, bool);
    define_get_set_function!(agc_gain, set_agc_gain, bool);
    define_get_set_function!(aec_value, set_aec_value, bool);
    define_get_set_function!(special_effect, set_special_effect, i32);
    define_get_set_function!(wb_mode, set_wb_mode, i32);
    define_get_set_function!(ae_level, set_ae_level, i32);
    define_get_set_function!(dcw, set_dcw, bool);
    define_get_set_function!(bpc, set_bpc, bool);
    define_get_set_function!(wpc, set_wpc, bool);
    define_get_set_function!(raw_gma, set_raw_gma, bool);
    define_get_set_function!(lenc, set_lenc, bool);

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

pub struct Camera<'a> {
    _p: PhantomData<&'a ()>,
}

pub struct CameraParams<'a> {
    power: c_int,
    clock: c_int,
    d0: c_int,
    d1: c_int,
    d2: c_int,
    d3: c_int,
    d4: c_int,
    d5: c_int,
    d6: c_int,
    d7: c_int,
    vertical_sync: c_int,
    horizontal_reference: c_int,
    pixel_clock: c_int,
    sda: c_int,
    scl: c_int,
    pixel_format: camera::pixformat_t,
    frame_size: camera::framesize_t,
    fb_location: camera::camera_fb_location_t,
    _p: PhantomData<&'a ()>,
}

impl CameraParams<'static> {
    pub fn new() -> CameraParams<'static> {
        Self {
            power: -1,
            clock: -1,
            d0: -1,
            d1: -1,
            d2: -1,
            d3: -1,
            d4: -1,
            d5: -1,
            d6: -1,
            d7: -1,
            vertical_sync: -1,
            horizontal_reference: -1,
            pixel_clock: -1,
            sda: -1,
            scl: -1,
            pixel_format: camera::pixformat_t_PIXFORMAT_JPEG,
            frame_size: camera::framesize_t_FRAMESIZE_UXGA,
            fb_location: camera::camera_fb_location_t_CAMERA_FB_IN_PSRAM,
            _p: PhantomData,
        }
    }
}

macro_rules! define_set_pin_function {
    ($name:ident, $direction:ty) => {
        concat_idents::concat_idents!(
            fn_name = set_,
            $name,
            _pin {
            pub fn fn_name(self, p: impl Peripheral<P = impl $direction> + 'a) -> CameraParams<'a> {
                CameraParams {
                    $name: p.into_ref().pin(),
                    _p: PhantomData,
                    ..self
                }
            }
        });
    };
}

impl<'a> CameraParams<'a> {
    define_set_pin_function!(power, OutputPin);
    define_set_pin_function!(clock, OutputPin);
    define_set_pin_function!(d0, IOPin);
    define_set_pin_function!(d1, IOPin);
    define_set_pin_function!(d2, IOPin);
    define_set_pin_function!(d3, IOPin);
    define_set_pin_function!(d4, IOPin);
    define_set_pin_function!(d5, IOPin);
    define_set_pin_function!(d6, IOPin);
    define_set_pin_function!(d7, IOPin);
    define_set_pin_function!(vertical_sync, IOPin);
    define_set_pin_function!(horizontal_reference, IOPin);
    define_set_pin_function!(pixel_clock, IOPin);
    define_set_pin_function!(sda, IOPin);
    define_set_pin_function!(scl, OutputPin);
}
