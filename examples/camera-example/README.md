This example assumes you have already set up all the tools necessary to compile Rust for ESP32. Follow [The Rust on ESP Book](https://docs.esp-rs.org/book/) for setup steps.

When you need to export `WIFI_SSID` and `WIFI_PASS` environment variables and run the example by using `cargo run -r`, it will compile and flash the program to an ESP32S3 with a camera. The pins are set up for this [Freenove board](https://github.com/Freenove/Freenove_ESP32_S3_WROOM_Board).

The program will:

1. Connect to your WiFi.
1. Initialize the onboard OV2640 camera as well as a LED.
1. Start HTTP server.
1. Take a jpeg-encoded picture using the camera for each GET on `/` url and send it.
1. Print trace information about each step and action.
