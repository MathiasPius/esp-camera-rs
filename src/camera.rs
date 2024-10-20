use esp_idf_sys::{camera, esp, EspError};
use std::marker::PhantomData;

use crate::{board::Config, CameraModel, CameraSensor, FrameBuffer, Setting};

pub struct Camera<'a, Model> {
    _p: PhantomData<&'a Model>,
}

impl<'a, Model> Camera<'a, Model>
where
    Model: CameraModel,
{
    pub fn new(config: Config<Model>) -> Result<Self, EspError> {
        esp!(unsafe { camera::esp_camera_init(&config.into_config()) })?;
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

    pub fn sensor(&self) -> CameraSensor<'a, Model> {
        CameraSensor::<'a, Model>::default()
    }

    pub fn set(&self, setting: &impl Setting<Model>) -> Result<(), EspError> {
        self.sensor().set(setting)
    }
}

impl<'a, Model> Drop for Camera<'a, Model> {
    fn drop(&mut self) {
        esp!(unsafe { camera::esp_camera_deinit() }).expect("error during esp_camera_deinit");
    }
}
