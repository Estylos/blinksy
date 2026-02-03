//! # Solid Pattern
//!
//! The solid pattern displays a single constant color across all LEDs.
//!
//! Solid pattern accepts any color space chosen by the user ([`Srgb`], [`Hsv`], [`Okhsv`]...).
//!
//! ## Example
//!
//! ```rust,ignore
//! use blinksy::{
//!     ControlBuilder,
//!     color::{Srgb, Okhsv},
//!     layout::Layout1d,
//!     layout1d,
//!     patterns::solid::{Solid, SolidParams}
//! };
//!
//! // Define a 1D layout
//! layout1d!(Layout, 60);
//!
//! // Create a Solid pattern with Srgb color
//! let control = ControlBuilder::new_1d()
//!     .with_layout::<Layout, { Layout::PIXEL_COUNT }>()
//!     .with_pattern::<Solid<Srgb>>(SolidParams {
//!         color: Srgb::new(1.0, 0.0, 0.0),  // Red
//!     })
//!     .with_driver(/* Your driver */)
//!     .with_frame_buffer_size::</* Length of frame buffer */>()
//!     .build();
//!
//! // Or use Okhsv for perceptually uniform colors
//! let control = ControlBuilder::new_1d()
//!     .with_layout::<Layout, { Layout::PIXEL_COUNT }>()
//!     .with_pattern::<Solid<Okhsv>>(SolidParams {
//!         color: Okhsv::new(29./360., 1.0, 0.57),   // Red
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

/// Configuration parameters for the Solid pattern.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SolidParams<Color> {
    /// The constant color
    pub color: Color,
}

impl Default for SolidParams<Srgb> {
    fn default() -> Self {
        Self {
            color: Srgb::new(1.0, 1.0, 1.0), // White
        }
    }
}

/// Solid pattern implementation.
///
/// Displays a single constant color across all LEDs on the layout.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Solid<Color> {
    /// Configuration parameters
    params: SolidParams<Color>,
}

/// 1D implementation
impl<Layout, Color> Pattern<Dim1d, Layout> for Solid<Color>
where
    Color: Copy,
    Layout: Layout1d,
{
    type Params = SolidParams<Color>;
    type Color = Color;

    /// Creates a new Solid pattern with the specified parameters.
    fn new(params: Self::Params) -> Self {
        Self { params }
    }

    /// Generates colors for a 1D layout.
    /// All LEDs on the layout will display the same constant color.
    fn tick(&self, _time_in_ms: u64) -> impl Iterator<Item = Self::Color> {
        Layout::points().map(move |_| self.params.color)
    }
}

/// 2D implementation
impl<Layout, Color> Pattern<Dim2d, Layout> for Solid<Color>
where
    Color: Copy,
    Layout: Layout2d,
{
    type Params = SolidParams<Color>;
    type Color = Color;

    /// Creates a new Solid pattern with the specified parameters.
    fn new(params: Self::Params) -> Self {
        Self { params }
    }

    /// Generates colors for a 2D layout.
    /// All LEDs on the layout will display the same constant color.
    fn tick(&self, _time_in_ms: u64) -> impl Iterator<Item = Self::Color> {
        Layout::points().map(move |_| self.params.color)
    }
}

/// 3D implementation
impl<Layout, Color> Pattern<Dim3d, Layout> for Solid<Color>
where
    Color: Copy,
    Layout: Layout3d,
{
    type Params = SolidParams<Color>;
    type Color = Color;

    /// Creates a new Solid pattern with the specified parameters.
    fn new(params: Self::Params) -> Self {
        Self { params }
    }

    /// Generates colors for a 3D layout.
    /// All LEDs on the layout will display the same constant color.
    fn tick(&self, _time_in_ms: u64) -> impl Iterator<Item = Self::Color> {
        Layout::points().map(move |_| self.params.color)
    }
}
