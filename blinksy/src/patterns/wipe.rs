//! # Wipe Pattern
//!
//! The wipe pattern switches LEDs one by one from start to end in a loop:
//! first wiping from `color1` to `color2`, then wiping back from `color2`
//! to `color1`, and repeating indefinitely.
//!
//! Wipe pattern accepts any color space chosen by the user ([`Srgb`], [`Hsv`], [`Okhsv`]...).
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
//!     patterns::wipe::{Wipe, WipeParams}
//! };
//!
//! // Define a 1D layout
//! layout1d!(StripLayout, 15);
//!
//! // Create a Wipe pattern
//! let control = ControlBuilder::new_1d()
//!     .with_layout::<StripLayout, { StripLayout::PIXEL_COUNT }>()
//!     .with_pattern::<Wipe<_>>(WipeParams {
//!         color1: Srgb::new(1., 0., 0.), // Red start color
//!         color2: Srgb::new(0., 0., 1.), // Blue end color
//!         delay_ms: 100
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
//!     .with_pattern::<Wipe<_>>(WipeParams {
//!         color1: Okhsl::new(300.0 / 360.0, 1.0, 0.3), // Deep purple
//!         color2: Okhsl::new(50.0 / 360.0, 1.0, 0.6),  // Gold
//!         delay_ms: 50
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

/// Configuration parameters for the Wipe pattern.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct WipeParams<Color> {
    /// First color
    pub color1: Color,
    /// Second color
    pub color2: Color,
    /// Delay in milliseconds between animation steps
    pub delay_ms: u64,
}

impl Default for WipeParams<Srgb> {
    fn default() -> Self {
        Self {
            color1: Srgb::new(1.0, 0.0, 0.0), // Red
            color2: Srgb::new(0.0, 0.0, 1.0), // Blue
            delay_ms: 100,
        }
    }
}

/// Wipe pattern implementation.
///
/// Switches LEDs one by one from start to end in a continuous loop:
/// first wiping from `color1` to `color2`, then wiping back from `color2`
/// to `color1`, and repeating.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Wipe<Color> {
    /// Configuration parameters
    params: WipeParams<Color>,
}

/// 1D implementation
impl<Layout, Color> Pattern<Dim1d, Layout> for Wipe<Color>
where
    Color: Copy,
    Layout: Layout1d,
{
    type Params = WipeParams<Color>;
    type Color = Color;

    /// Creates a new wipe pattern with the specified parameters.
    fn new(params: Self::Params) -> Self {
        Self { params }
    }

    /// Generates colors for a 1D layout.
    fn tick(&self, time_in_ms: u64) -> impl Iterator<Item = Self::Color> {
        let Self { params } = self;
        let WipeParams {
            color1,
            color2,
            delay_ms,
        } = *params;

        let num_leds = Layout::PIXEL_COUNT;

        // The full cycle is 2 * num_leds steps:
        // - phase 1 (step < num_leds):  wipe color1 to color2, LED i is color2 if i < step
        // - phase 2 (step >= num_leds): wipe color2 to color1, LED i is color1 if i < (step - num_leds)
        let step = if delay_ms > 0 {
            (time_in_ms / delay_ms) as usize % (2 * num_leds)
        } else {
            0
        };

        Layout::points().enumerate().map(move |(i, _)| {
            if step < num_leds {
                // Phase 1: wiping from color1 to color2
                if i < step {
                    color2
                } else {
                    color1
                }
            } else {
                // Phase 2: wiping back from color2 to color1
                if i < step - num_leds {
                    color1
                } else {
                    color2
                }
            }
        })
    }
}

/// 2D implementation
impl<Layout, Color> Pattern<Dim2d, Layout> for Wipe<Color>
where
    Color: Copy,
    Layout: Layout2d,
{
    type Params = WipeParams<Color>;
    type Color = Color;

    /// Creates a new wipe pattern with the specified parameters.
    fn new(params: Self::Params) -> Self {
        Self { params }
    }

    /// Generates colors for a 2D layout.
    fn tick(&self, time_in_ms: u64) -> impl Iterator<Item = Self::Color> {
        let Self { params } = self;
        let WipeParams {
            color1,
            color2,
            delay_ms,
        } = *params;

        let num_leds = Layout::PIXEL_COUNT;

        let step = if delay_ms > 0 {
            (time_in_ms / delay_ms) as usize % (2 * num_leds)
        } else {
            0
        };

        Layout::points().enumerate().map(move |(i, _)| {
            if step < num_leds {
                if i < step {
                    color2
                } else {
                    color1
                }
            } else {
                if i < step - num_leds {
                    color1
                } else {
                    color2
                }
            }
        })
    }
}

/// 3D implementation
impl<Layout, Color> Pattern<Dim3d, Layout> for Wipe<Color>
where
    Color: Copy,
    Layout: Layout3d,
{
    type Params = WipeParams<Color>;
    type Color = Color;

    /// Creates a new wipe pattern with the specified parameters.
    fn new(params: Self::Params) -> Self {
        Self { params }
    }

    /// Generates colors for a 3D layout.
    fn tick(&self, time_in_ms: u64) -> impl Iterator<Item = Self::Color> {
        let Self { params } = self;
        let WipeParams {
            color1,
            color2,
            delay_ms,
        } = *params;

        let num_leds = Layout::PIXEL_COUNT;

        let step = if delay_ms > 0 {
            (time_in_ms / delay_ms) as usize % (2 * num_leds)
        } else {
            0
        };

        Layout::points().enumerate().map(move |(i, _)| {
            if step < num_leds {
                if i < step {
                    color2
                } else {
                    color1
                }
            } else {
                if i < step - num_leds {
                    color1
                } else {
                    color2
                }
            }
        })
    }
}
