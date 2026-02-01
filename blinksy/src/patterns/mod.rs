//! # Pattern Implementations
//!
//! This is the library of built-in patterns.
//!
//! - [`rainbow`][]: A basic scrolling rainbow.
//! - [`noise`]: A flow through random noise functions.
//! - [`blink`]: A simple on/off blink pattern.
//!
//! If you want help to port a pattern from FastLED / WLED to Rust, [make an issue](https://github.com/ahdinosaur/blinksy/issues)!

pub mod blink;
pub mod noise;
pub mod rainbow;
