use blinksy::{
    color::Srgb,
    layout::{Layout3d, Shape3d, Vec3},
    layout3d,
    patterns::wipe::{Wipe, WipeParams},
    ControlBuilder,
};
use blinksy_desktop::{
    driver::{Desktop, DesktopError},
    time::elapsed_in_ms,
};
use core::f32::consts::TAU;
use std::{thread::sleep, time::Duration};

// Five horizontal rings stacked vertically forming a jar
layout3d!(
    CylinderLayout,
    [
        Shape3d::Arc {
            center: Vec3::new(0., -1.0, 0.),
            axis_u: Vec3::new(0.6, 0., 0.),
            axis_v: Vec3::new(0., 0., 0.6),
            start_angle_in_radians: 0.0,
            end_angle_in_radians: TAU,
            pixel_count: 36,
        },
        Shape3d::Arc {
            center: Vec3::new(0., -0.5, 0.),
            axis_u: Vec3::new(0.8, 0., 0.),
            axis_v: Vec3::new(0., 0., 0.8),
            start_angle_in_radians: 0.0,
            end_angle_in_radians: TAU,
            pixel_count: 48,
        },
        Shape3d::Arc {
            center: Vec3::new(0., 0.0, 0.),
            axis_u: Vec3::new(1.0, 0., 0.),
            axis_v: Vec3::new(0., 0., 1.0),
            start_angle_in_radians: 0.0,
            end_angle_in_radians: TAU,
            pixel_count: 60,
        },
        Shape3d::Arc {
            center: Vec3::new(0., 0.5, 0.),
            axis_u: Vec3::new(0.8, 0., 0.),
            axis_v: Vec3::new(0., 0., 0.8),
            start_angle_in_radians: 0.0,
            end_angle_in_radians: TAU,
            pixel_count: 48,
        },
        Shape3d::Arc {
            center: Vec3::new(0., 1.0, 0.),
            axis_u: Vec3::new(0.6, 0., 0.),
            axis_v: Vec3::new(0., 0., 0.6),
            start_angle_in_radians: 0.0,
            end_angle_in_radians: TAU,
            pixel_count: 36,
        },
    ]
);

fn main() {
    Desktop::new_3d::<CylinderLayout>().start(|driver| {
        let mut control = ControlBuilder::new_3d()
            .with_layout::<CylinderLayout, { CylinderLayout::PIXEL_COUNT }>()
            .with_pattern::<Wipe<_>>(WipeParams {
                color1: Srgb::new(0.0, 0.8, 1.0), // Cyan
                color2: Srgb::new(1.0, 0.0, 0.5), // Magenta
                delay_ms: 3,
            })
            .with_driver(driver)
            .with_frame_buffer_size::<{ CylinderLayout::PIXEL_COUNT }>()
            .build();

        loop {
            if let Err(DesktopError::WindowClosed) = control.tick(elapsed_in_ms()) {
                break;
            }

            sleep(Duration::from_millis(16));
        }
    });
}
