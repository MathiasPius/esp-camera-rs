use esp_idf_hal::gpio::{Pin as _, Pins};

use crate::ov2640::Ov2640;

use super::{PinConfig, PinLayout};

pub struct ESP32CAM;

impl PinLayout<Ov2640> for ESP32CAM {
    fn pin_config(pins: &Pins) -> PinConfig {
        PinConfig {
            pwdn: pins.gpio32.pin(),
            xclk: pins.gpio0.pin(),
            reset: 0xff,
            d0: pins.gpio5.pin(),
            d1: pins.gpio18.pin(),
            d2: pins.gpio19.pin(),
            d3: pins.gpio21.pin(),
            d4: pins.gpio36.pin(),
            d5: pins.gpio39.pin(),
            d6: pins.gpio34.pin(),
            d7: pins.gpio35.pin(),
            vsync: pins.gpio25.pin(),
            href: pins.gpio23.pin(),
            pclk: pins.gpio22.pin(),
            sda: pins.gpio26.pin(),
            scl: pins.gpio27.pin(),
        }
    }
}
