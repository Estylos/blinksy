#![no_std]
#![no_main]

#[cfg(feature = "backtrace")]
use esp_backtrace as _;

#[cfg(feature = "defmt")]
use esp_println as _;

use blinksy::{
    driver::ClocklessDriver,
    layout::Layout1d,
    layout1d,
    leds::Ws2812,
    patterns::rainbow::{Rainbow, RainbowParams},
    ControlBuilder,
};

use blinksy_esp::{rmt::rmt_buffer_size, rmt::ClocklessRmtBuilder, time::elapsed};

esp_bootloader_esp_idf::esp_app_desc!();

#[esp_hal::main]
fn main() -> ! {
    // esp-hal configuration
    let cpu_clock = esp_hal::clock::CpuClock::max();
    let config = esp_hal::Config::default().with_cpu_clock(cpu_clock);
    let p = esp_hal::init(config);

    // Setup ESP32 RMT driver
    let rmt_clk_freq = esp_hal::time::Rate::from_mhz(80);
    let rmt = esp_hal::rmt::Rmt::new(p.RMT, rmt_clk_freq).unwrap();

    let data_pin = p.GPIO4; // Change to the GPIO pin connected to your WS2812 data line
    let rmt_channel = rmt.channel0;

    // Define a 1D layout (strip) of 12 pixels
    layout1d!(Layout, 12);

    // Setup the WS2812 driver and writer
    let ws2812_driver = {
        const RMT_BUFF_SIZE: usize = rmt_buffer_size::<Ws2812>(Layout::PIXEL_COUNT);
        ClocklessDriver::default()
            .with_led::<Ws2812>() // Specify the LED type
            .with_writer(
                ClocklessRmtBuilder::default()
                    .with_rmt_buffer_size::<RMT_BUFF_SIZE>()
                    .with_led::<Ws2812>()
                    .with_channel(rmt_channel)
                    .with_pin(data_pin)
                    .build(),
            )
    };

    // Setup the Blinksy controller
    let mut control = ControlBuilder::new_1d()
        .with_layout::<Layout, { Layout::PIXEL_COUNT }>() // Specify the layout
        .with_pattern::<Rainbow>(RainbowParams::default()) // Use the Rainbow pattern with default parameters
        .with_driver(ws2812_driver)
        .with_frame_buffer_size::<{ Ws2812::frame_buffer_size(Layout::PIXEL_COUNT) }>()
        .build();

    // Set initial brightness (0.0 to 1.0)
    control.set_brightness(0.5);

    let delay = esp_hal::delay::Delay::new();

    loop {
        let elapsed_in_ms = elapsed().as_millis();
        control.tick(elapsed_in_ms).unwrap(); // Update the control with elapsed time, permitting pattern animation

        delay.delay_millis(10);
    }
}
