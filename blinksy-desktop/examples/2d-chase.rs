use blinksy::{
    color::Srgb,
    layout::{Layout2d, Shape2d, Vec2},
    layout2d,
    patterns::chase::{Chase, ChaseParams},
    ControlBuilder,
};
use blinksy_desktop::{
    driver::{Desktop, DesktopError},
    time::elapsed_in_ms,
};
use core::f32::consts::TAU;
use std::{thread::sleep, time::Duration};

// Five rings of increasing radius
layout2d!(
    RingsLayout,
    [
        Shape2d::Arc {
            center: Vec2::new(0., 0.),
            axis_u: Vec2::new(0.2, 0.),
            axis_v: Vec2::new(0., 0.2),
            start_angle_in_radians: 0.0,
            end_angle_in_radians: TAU,
            pixel_count: 12,
        },
        Shape2d::Arc {
            center: Vec2::new(0., 0.),
            axis_u: Vec2::new(0.4, 0.),
            axis_v: Vec2::new(0., 0.4),
            start_angle_in_radians: 0.0,
            end_angle_in_radians: TAU,
            pixel_count: 24,
        },
        Shape2d::Arc {
            center: Vec2::new(0., 0.),
            axis_u: Vec2::new(0.6, 0.),
            axis_v: Vec2::new(0., 0.6),
            start_angle_in_radians: 0.0,
            end_angle_in_radians: TAU,
            pixel_count: 36,
        },
        Shape2d::Arc {
            center: Vec2::new(0., 0.),
            axis_u: Vec2::new(0.8, 0.),
            axis_v: Vec2::new(0., 0.8),
            start_angle_in_radians: 0.0,
            end_angle_in_radians: TAU,
            pixel_count: 48,
        },
        Shape2d::Arc {
            center: Vec2::new(0., 0.),
            axis_u: Vec2::new(1.0, 0.),
            axis_v: Vec2::new(0., 1.0),
            start_angle_in_radians: 0.0,
            end_angle_in_radians: TAU,
            pixel_count: 60,
        },
    ]
);

fn main() {
    Desktop::new_2d::<RingsLayout>().start(|driver| {
        let mut control = ControlBuilder::new_2d()
            .with_layout::<RingsLayout, { RingsLayout::PIXEL_COUNT }>()
            .with_pattern::<Chase<_>>(ChaseParams {
                color1: Srgb::new(0.05, 0.0, 0.15), // Deep violet background
                color2: Srgb::new(1.0, 0.5, 0.0),   // Orange chase
                width: 12,
                count: 3,
                delay_ms: 20,
            })
            .with_driver(driver)
            .with_frame_buffer_size::<{ RingsLayout::PIXEL_COUNT }>()
            .build();

        loop {
            if let Err(DesktopError::WindowClosed) = control.tick(elapsed_in_ms()) {
                break;
            }

            sleep(Duration::from_millis(16));
        }
    });
}
