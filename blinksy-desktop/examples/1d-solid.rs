use blinksy::{
    ControlBuilder, color::{Srgb}, layout::Layout1d, layout1d, patterns::solid::{Solid, SolidParams}
};
use blinksy_desktop::{
    driver::{Desktop, DesktopError},
    time::elapsed_in_ms,
};
use std::{thread::sleep, time::Duration};

layout1d!(StripLayout, 5);

fn main() {
    Desktop::new_1d::<StripLayout>().start(|driver| {
        let mut control = ControlBuilder::new_1d()
            .with_layout::<StripLayout, { StripLayout::PIXEL_COUNT }>()
            .with_pattern::<Solid<_>>(SolidParams {
                color: Srgb::new(0.2, 0.8, 0.4), // Mint green
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
