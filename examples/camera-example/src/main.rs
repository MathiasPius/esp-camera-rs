use embedded_svc::http::Method;
use embedded_svc::io::Write;
use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::{gpio::PinDriver, peripherals::Peripherals};
use esp_idf_svc::http::server::{Configuration as HttpServerConfig, EspHttpServer};
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use log::{error, info};
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("initializing peripherals");
    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    info!("initializing WiFi");
    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(peripherals.modem, sysloop.clone(), Some(nvs))?,
        sysloop,
    )?;

    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: env!("WIFI_SSID").try_into().unwrap(),
        bssid: None,
        auth_method: AuthMethod::None,
        password: env!("WIFI_PASS").try_into().unwrap(),
        channel: None,
    }))?;

    wifi.start()?;
    wifi.connect()?;
    wifi.wait_netif_up()?;
    info!(
        "Wifi is ready, ip {:?}",
        wifi.wifi().sta_netif().get_ip_info()?
    );

    let mut led = PinDriver::output(peripherals.pins.gpio2)?;
    led.set_low()?;

    info!("Initialize the camera");
    let camera = esp_camera_rs::Camera::new(
        None,
        peripherals.pins.gpio15,
        peripherals.pins.gpio11,
        peripherals.pins.gpio9,
        peripherals.pins.gpio8,
        peripherals.pins.gpio10,
        peripherals.pins.gpio12,
        peripherals.pins.gpio18,
        peripherals.pins.gpio17,
        peripherals.pins.gpio16,
        peripherals.pins.gpio6,
        peripherals.pins.gpio7,
        peripherals.pins.gpio13,
        peripherals.pins.gpio4,
        peripherals.pins.gpio5,
        esp_idf_sys::camera::pixformat_t_PIXFORMAT_JPEG,
        esp_idf_sys::camera::framesize_t_FRAMESIZE_UXGA,
        esp_idf_sys::camera::camera_fb_location_t_CAMERA_FB_IN_PSRAM,
    )?;

    info!("initializing http servert");

    let state = std::sync::Arc::new(std::sync::Mutex::new((led, camera)));

    let mut httpserver = EspHttpServer::new(&HttpServerConfig::default())?;
    info!("preocessing http requests");

    httpserver.fn_handler("/", Method::Get, |request| {
        info!("taking a picture");
        let (ref mut led, camera) = &mut *state.lock().unwrap();

        led.set_high()?;
        let framebuffer = camera.get_framebuffer();
        led.set_low()?;
        if let Some(framebuffer) = framebuffer {
            info!(
                "took picture: {width}x{height} {size} bytes",
                width = framebuffer.width(),
                height = framebuffer.height(),
                size = framebuffer.data().len(),
            );
            let mut response =
                request.into_response(200, Some("Ok"), &[("Content-Type", "image/jpeg")])?;
            response.write_all(framebuffer.data())
        } else {
            error!("failed to take image");
            let mut response = request.into_status_response(500)?;
            response.write_all(b"camera failed")
        }
        .map(|_| ())
    })?;

    loop {
        std::thread::sleep(Duration::from_millis(1000));
    }
}
