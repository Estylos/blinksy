//! # Blink Pattern
//!
//! The blink pattern alternates between two colors at regular intervals.
//!
//! Blink pattern accepts any color space chosen by the user ([`Srgb`], [`Hsv`], [`Okhsv`]...).
//!
//! ## Example
//!
//! ```rust,ignore
//! use blinksy::{
//!     ControlBuilder,
//!     color::{Srgb, Okhsv},
//!     layout::Layout1d,
//!     layout1d,
//!     patterns::blink::{Blink, BlinkParams}
//! };
//!
//! // Define a 1D layout
//! layout1d!(Layout, 60);
//!
//! // Create a Blink pattern with Srgb colors
//! let control = ControlBuilder::new_1d()
//!     .with_layout::<Layout, { Layout::PIXEL_COUNT }>()
//!     .with_pattern::<Blink<Srgb>>(BlinkParams {
//!         color1: Srgb::new(1.0, 0.0, 0.0),  // Red
//!         color2: Srgb::new(0.0, 1.0, 0.0),  // Green
//!         color1_ms: 1000,
//!         color2_ms: 1000,
//!     })
//!     .with_driver(/* Your driver */)
//!     .with_frame_buffer_size::</* Length of frame buffer */>()
//!     .build();
//!
//! // Or use Okhsv for perceptually uniform colors
//! let control = ControlBuilder::new_1d()
//!     .with_layout::<Layout, { Layout::PIXEL_COUNT }>()
//!     .with_pattern::<Blink<Okhsv>>(BlinkParams {
//!         color1: Okhsv::new(29./360., 1.0, 0.57),   // Red
//!         color2: Okhsv::new(142./360., 1.0, 0.87),  // Green
//!         color1_ms: 1000,
//!         color2_ms: 1000,
//!     })
//!     .with_driver(/* Your driver */)
//!     .with_frame_buffer_size::</* Length of frame buffer */>()
//!     .build();
//! ```
//! [`Srgb`]: crate::color::Srgb
//! [`Hsv`]: crate::color::Hsv
//! [`Okhsv`]: crate::color::Okhsv

use crate::{
    color::Srgb,
    layout::{Layout1d, Layout2d, Layout3d},
    markers::{Dim1d, Dim2d, Dim3d},
    pattern::Pattern,
};

/// Configuration parameters for the Blink pattern.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct BlinkParams<Color> {
    /// First color
    pub color1: Color,
    /// Second color
    pub color2: Color,
    /// Duration in milliseconds for the first color
    pub color1_ms: u64,
    /// Duration in milliseconds for the second color
    pub color2_ms: u64,
}

impl Default for BlinkParams<Srgb> {
    fn default() -> Self {
        Self {
            color1: Srgb::new(1.0, 0.0, 0.0), // Red
            color2: Srgb::new(0.0, 0.0, 1.0), // Blue
            color1_ms: 1000,
            color2_ms: 1000,
        }
    }
}

/// Blink pattern implementation.
///
/// Alternates between two specified colors at defined time intervals on the LED layout.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Blink<Color> {
    /// Configuration parameters
    params: BlinkParams<Color>,
}

/// 1D implementation
impl<Layout, Color> Pattern<Dim1d, Layout> for Blink<Color>
where
    Color: Copy,
    Layout: Layout1d,
{
    type Params = BlinkParams<Color>;
    type Color = Color;

    /// Creates a new Blink pattern with the specified parameters.
    fn new(params: Self::Params) -> Self {
        Self { params }
    }

    /// Generates colors for a 1D layout.
    /// All LEDs on the layout will be the same color at any given time.
    fn tick(&self, time_in_ms: u64) -> impl Iterator<Item = Self::Color> {
        let Self { params } = self;
        let BlinkParams {
            color1,
            color2,
            color1_ms,
            color2_ms,
        } = *params;

        let total_ms = color1_ms + color2_ms;
        let elapsed = if total_ms > 0 {
            time_in_ms % total_ms
        } else {
            0
        };
        let current_color = if elapsed < color1_ms { color1 } else { color2 };

        Layout::points().map(move |_| current_color)
    }
}

/// 2D implementation
impl<Layout, Color> Pattern<Dim2d, Layout> for Blink<Color>
where
    Color: Copy,
    Layout: Layout2d,
{
    type Params = BlinkParams<Color>;
    type Color = Color;

    /// Creates a new Blink pattern with the specified parameters.
    fn new(params: Self::Params) -> Self {
        Self { params }
    }

    /// Generates colors for a 2D layout.
    /// All LEDs on the layout will be the same color at any given time.
    fn tick(&self, time_in_ms: u64) -> impl Iterator<Item = Self::Color> {
        let Self { params } = self;
        let BlinkParams {
            color1,
            color2,
            color1_ms,
            color2_ms,
        } = *params;

        let total_ms = color1_ms + color2_ms;
        let elapsed = if total_ms > 0 {
            time_in_ms % total_ms
        } else {
            0
        };
        let current_color = if elapsed < color1_ms { color1 } else { color2 };

        Layout::points().map(move |_| current_color)
    }
}

/// 3D implementation
impl<Layout, Color> Pattern<Dim3d, Layout> for Blink<Color>
where
    Color: Copy,
    Layout: Layout3d,
{
    type Params = BlinkParams<Color>;
    type Color = Color;

    /// Creates a new Blink pattern with the specified parameters.
    fn new(params: Self::Params) -> Self {
        Self { params }
    }

    /// Generates colors for a 3D layout.
    /// All LEDs on the layout will be the same color at any given time.
    fn tick(&self, time_in_ms: u64) -> impl Iterator<Item = Self::Color> {
        let Self { params } = self;
        let BlinkParams {
            color1,
            color2,
            color1_ms,
            color2_ms,
        } = *params;

        let total_ms = color1_ms + color2_ms;
        let elapsed = if total_ms > 0 {
            time_in_ms % total_ms
        } else {
            0
        };
        let current_color = if elapsed < color1_ms { color1 } else { color2 };

        Layout::points().map(move |_| current_color)
    }
}
