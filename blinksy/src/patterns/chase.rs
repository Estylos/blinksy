//! # Chase Pattern
//!
//! The chase pattern animates moving segments of color along the LEDs.
//! It displays `count` segments of `color2` (each `width` LEDs long)
//! moving across a background of `color1`.
//!
//! Chase pattern accepts any color space chosen by the user ([`Srgb`], [`Hsv`], [`Okhsv`]...).
//!
//! ## Examples
//!
//! ### 1D Example
//!
//! ```rust,ignore
//! use blinksy::{
//!     ControlBuilder,
//!     color::Srgb,
//!     layout::Layout1d,
//!     layout1d,
//!     patterns::chase::{Chase, ChaseParams}
//! };
//!
//! // Define a 1D layout
//! layout1d!(StripLayout, 15);
//!
//! // Create a Chase pattern
//! let control = ControlBuilder::new_1d()
//!     .with_layout::<StripLayout, { StripLayout::PIXEL_COUNT }>()
//!     .with_pattern::<Chase<_>>(ChaseParams {
//!         color1: Srgb::new(1., 0., 0.), // Red background
//!         color2: Srgb::new(0., 0., 1.), // Blue chase
//!         width: 3,
//!         count: 2,
//!         delay_ms: 200
//!     })
//!     .with_driver(/* Your driver */)
//!     .with_frame_buffer_size::</* Length of frame buffer */>()
//!     .build();
//! ```
//!
//! ### 3D Example (with [`Okhsl`] colors)
//!
//! ```rust,ignore
//! let control = ControlBuilder::new_3d()
//!     .with_layout::<CubeVolumeLayout, { CubeVolumeLayout::PIXEL_COUNT }>()
//!     .with_pattern::<Chase<_>>(ChaseParams {
//!         color1: Okhsl::new(300.0 / 360.0, 1.0, 0.3), // Deep purple background
//!         color2: Okhsl::new(50.0 / 360.0, 1.0, 0.6),  // Gold chase
//!         width: 6,
//!         count: 25,
//!         delay_ms: 10       
//!     })
//!     .with_driver(/* Your driver */)
//!     .with_frame_buffer_size::</* Length of frame buffer */>()
//!     .build();
//! ```
//!
//! [`Srgb`]: crate::color::Srgb
//! [`Hsv`]: crate::color::Hsv
//! [`Okhsv`]: crate::color::Okhsv
//! [`Okhsl`]: crate::color::Okhsl

use crate::{
    color::Srgb,
    layout::{Layout1d, Layout2d, Layout3d},
    markers::{Dim1d, Dim2d, Dim3d},
    pattern::Pattern,
};

/// Configuration parameters for the Chase pattern.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ChaseParams<Color> {
    /// Background color
    pub color1: Color,
    /// Chase color
    pub color2: Color,
    /// Width of the chase effect in number of LEDs
    pub width: usize,
    /// Number of chases going around the layout
    pub count: usize,
    /// Delay in milliseconds between animation steps
    pub delay_ms: u64,
}

impl Default for ChaseParams<Srgb> {
    fn default() -> Self {
        Self {
            color1: Srgb::new(1.0, 0.0, 0.0), // Red
            color2: Srgb::new(0.0, 0.0, 1.0), // Blue
            width: 2,
            count: 1,
            delay_ms: 100,
        }
    }
}

/// Chase pattern implementation.
///
/// Animates moving segments of color along the LEDs.
/// It displays `count` segments of `color2` (each `width` LEDs long)
/// moving across a background of `color1`.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Chase<Color> {
    /// Configuration parameters
    params: ChaseParams<Color>,
}

/// 1D implementation
impl<Layout, Color> Pattern<Dim1d, Layout> for Chase<Color>
where
    Color: Copy,
    Layout: Layout1d,
{
    type Params = ChaseParams<Color>;
    type Color = Color;

    /// Creates a new chase pattern with the specified parameters.
    fn new(params: Self::Params) -> Self {
        Self { params }
    }

    /// Generates colors for a 1D layout.
    fn tick(&self, time_in_ms: u64) -> impl Iterator<Item = Self::Color> {
        let Self { params } = self;
        let ChaseParams {
            color1,
            color2,
            width,
            count,
            delay_ms,
        } = *params;

        let num_leds = Layout::PIXEL_COUNT;

        // The chase advances by one LED every delay_ms. We wrap around the full strip length
        // Eg: with delay_ms=100 and num_leds=10, the offset will advance every 100ms as follows:
        // time_in_ms: 0, 100, 200, 300, 400, 500, 600, 700, 800, 900, 1000
        // offset:     0, 1,   2,   3,   4,   5,   6,   7,   8,   9,   0
        let offset = if delay_ms > 0 {
            (time_in_ms / delay_ms) as usize % num_leds
        } else {
            0
        };

        Layout::points().enumerate().map(move |(i, _)| {
            // Shift the index backwards by the current offset to simulate forward motion
            // Eg: with num_leds=10 and offset=3 (t=300ms), the mapping will be:
            // i:   0, 1, 2, 3, 4, 5, 6, 7, 8, 9
            // pos: 7, 8, 9, 0, 1, 2, 3, 4, 5, 6
            let pos = (i + num_leds - offset) % num_leds;

            // Check if this position belongs to any of the chases
            // Each chase starts at (c * num_leds / count) and occupies `width` LEDs
            let is_chase = (0..count).any(|c| {
                let chase_start = (c * num_leds) / count;
                // Calculate the distance from pos to chase_start
                let distance = (pos + num_leds - chase_start) % num_leds;
                distance < width
            });

            if is_chase {
                color2
            } else {
                color1
            }
        })
    }
}

/// 2D implementation
impl<Layout, Color> Pattern<Dim2d, Layout> for Chase<Color>
where
    Color: Copy,
    Layout: Layout2d,
{
    type Params = ChaseParams<Color>;
    type Color = Color;

    /// Creates a new chase pattern with the specified parameters.
    fn new(params: Self::Params) -> Self {
        Self { params }
    }

    /// Generates colors for a 2D layout.
    fn tick(&self, time_in_ms: u64) -> impl Iterator<Item = Self::Color> {
        let Self { params } = self;
        let ChaseParams {
            color1,
            color2,
            width,
            count,
            delay_ms,
        } = *params;

        let num_leds = Layout::PIXEL_COUNT;

        let offset = if delay_ms > 0 {
            (time_in_ms / delay_ms) as usize % num_leds
        } else {
            0
        };

        Layout::points().enumerate().map(move |(i, _)| {
            let pos = (i + num_leds - offset) % num_leds;

            let is_chase = (0..count).any(|c| {
                let chase_start = (c * num_leds) / count;
                let distance = (pos + num_leds - chase_start) % num_leds;
                distance < width
            });

            if is_chase {
                color2
            } else {
                color1
            }
        })
    }
}

/// 3D implementation
impl<Layout, Color> Pattern<Dim3d, Layout> for Chase<Color>
where
    Color: Copy,
    Layout: Layout3d,
{
    type Params = ChaseParams<Color>;
    type Color = Color;

    /// Creates a new chase pattern with the specified parameters.
    fn new(params: Self::Params) -> Self {
        Self { params }
    }

    /// Generates colors for a 3D layout.
    fn tick(&self, time_in_ms: u64) -> impl Iterator<Item = Self::Color> {
        let Self { params } = self;
        let ChaseParams {
            color1,
            color2,
            width,
            count,
            delay_ms,
        } = *params;

        let num_leds = Layout::PIXEL_COUNT;

        let offset = if delay_ms > 0 {
            (time_in_ms / delay_ms) as usize % num_leds
        } else {
            0
        };

        Layout::points().enumerate().map(move |(i, _)| {
            let pos = (i + num_leds - offset) % num_leds;

            let is_chase = (0..count).any(|c| {
                let chase_start = (c * num_leds) / count;
                let distance = (pos + num_leds - chase_start) % num_leds;
                distance < width
            });

            if is_chase {
                color2
            } else {
                color1
            }
        })
    }
}
