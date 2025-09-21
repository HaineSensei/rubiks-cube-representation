//! Core mathematical foundations for Rubik's cube representation and analysis.
//!
//! This module provides the fundamental building blocks for cube representation,
//! mathematical analysis, and solving algorithms. It establishes a clean separation
//! between abstract mathematical concepts and concrete cube implementations.
//!
//! # Module Organization
//!
//! - [`cube`]: Abstract mathematical cube theory (geometry, rotations, color schemes)
//! - [`rubiks`]: Concrete cube state implementation for NxNxN cubes
//! - [`Colour`]: Standard cube colors used throughout the system
//!
//! # Design Philosophy
//!
//! The core module emphasizes mathematical elegance and type safety:
//! - **Abstract foundations first**: Mathematical concepts (group theory, geometry)
//!   are separated from concrete implementations
//! - **Generic design**: Support for cubes of any size through const generics
//! - **Rotation-invariant analysis**: Algorithms that work regardless of cube orientation
//! - **Type-driven correctness**: Leverage Rust's type system to encode mathematical constraints
//!
//! # Color System
//!
//! The [`Colour`] enum defines the six standard cube colors, providing a foundation
//! for color scheme implementations and cube state representation.

use std::ops::Add;

pub mod cube;
pub mod rubiks;

#[cfg(test)]
mod tests;

/// Standard colors used on Rubik's cube faces.
///
/// This enum represents the six colors commonly found on standard Rubik's cubes.
/// These colors correspond to the Western color scheme convention and provide
/// the foundation for all color scheme implementations in the system.
///
/// # Color Conventions
///
/// The color selection follows the most common Western Rubik's cube color scheme:
/// - **White** and **Yellow**: Opposite faces (typically top/bottom)
/// - **Red** and **Orange**: Opposite faces (typically front/back)
/// - **Blue** and **Green**: Opposite faces (typically left/right)
///
/// # Usage
///
/// These colors are used throughout the system for:
/// - Defining color schemes via the [`ColourScheme`](cube::schemes::ColourScheme) trait
/// - Representing tile colors in cube state structures
/// - Creating solved and scrambled cube configurations
/// - Algorithm analysis and pattern recognition
///
/// # Design Notes
///
/// The enum derives standard traits for:
/// - **Copy/Clone**: Efficient passing and storage
/// - **Debug**: Development and testing support
/// - **PartialEq/Eq**: Color comparison operations
/// - **Hash**: Use in hash-based collections and algorithms
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Colour {
    /// Pure white color, typically used for the top face in Western schemes
    White,
    /// Bright yellow color, typically opposite to white
    Yellow,
    /// Bright red color, typically used for one of the side faces
    Red,
    /// Bright orange color, typically opposite to red
    Orange,
    /// Bright blue color, typically used for one of the remaining side faces
    Blue,
    /// Bright green color, typically opposite to blue
    Green,
}

/// Array containing all six standard cube colors.
///
/// This constant provides convenient access to all [`Colour`] variants for
/// iteration, validation, and algorithmic operations. The colors are listed
/// in enum declaration order.
///
/// # Usage Examples
///
/// ```
/// use rubiks_cube_representation::core::{Colour, COLOURS};
///
/// // Iterate over all colors
/// for color in COLOURS {
///     println!("Color: {:?}", color);
/// }
///
/// // Check if a color is valid
/// fn is_valid_color(color: Colour) -> bool {
///     COLOURS.contains(&color)
/// }
/// ```
pub const COLOURS: [Colour;6] = [Colour::White, Colour::Yellow, Colour::Red, Colour::Orange, Colour::Blue, Colour::Green];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Angle {
    Zero,
    CWQuarter,
    Half,
    ACWQuarter,
}

// no I don't know why I wrote it like this and not just as ints 0..3 with addition mod 4. Okay, yes I do, and it's for readability.
impl Add for Angle {
    type Output = Angle;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Angle::Zero,x) => x,
            (x, Angle::Zero) => x,
            (Angle::CWQuarter, Angle::CWQuarter) => Angle::Half,
            (Angle::CWQuarter, Angle::Half) => Angle::ACWQuarter,
            (Angle::CWQuarter, Angle::ACWQuarter) => Angle::Zero,
            (Angle::Half, Angle::CWQuarter) => Angle::ACWQuarter,
            (Angle::Half, Angle::Half) => Angle::Zero,
            (Angle::Half, Angle::ACWQuarter) => Angle::CWQuarter,
            (Angle::ACWQuarter, Angle::CWQuarter) => Angle::Zero,
            (Angle::ACWQuarter, Angle::Half) => Angle::CWQuarter,
            (Angle::ACWQuarter, Angle::ACWQuarter) => Angle::Half,
        }
    }
}