use blinksy::{
    color::Okhsl,
    layout::Layout1d,
    layout1d,
    patterns::blink::{Blink, BlinkParams},
    ControlBuilder,
};
use blinksy_desktop::{
    driver::{Desktop, DesktopError},
    time::elapsed_in_ms,
};
use std::{thread::sleep, time::Duration};

layout1d!(StripLayout, 15);

fn main() {
    Desktop::new_1d::<StripLayout>().start(|driver| {
        let mut control = ControlBuilder::new_1d()
            .with_layout::<StripLayout, { StripLayout::PIXEL_COUNT }>()
            .with_pattern::<Blink<_>>(BlinkParams {
                color1: Okhsl::new(25.0 / 360.0, 0.9, 0.65),  // Warm coral
                color2: Okhsl::new(270.0 / 360.0, 0.7, 0.65), // Soft lavender
                color1_ms: 600,
                color2_ms: 200,
            })
            .with_driver(driver)
            .with_frame_buffer_size::<{ StripLayout::PIXEL_COUNT }>()
            .build();

        loop {
            if let Err(DesktopError::WindowClosed) = control.tick(elapsed_in_ms()) {
                break;
            }

            sleep(Duration::from_millis(16));
        }
    });
}
