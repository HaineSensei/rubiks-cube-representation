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

use std::ops::{Add, Sub};

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

/// Rotation angles for face and tile manipulations.
///
/// This enum represents the four possible rotation angles when rotating a face or
/// layer of the cube. It forms a cyclic group isomorphic to ℤ₄ (integers modulo 4)
/// under addition, providing a mathematically clean representation of 2D rotations.
///
/// # Rotation Direction Convention
///
/// All rotations are defined relative to viewing a face directly (looking straight
/// at the face from outside the cube):
/// - **Clockwise (CW)**: 90° rotation in the direction of clock hands
/// - **Counterclockwise (ACW)**: 90° rotation opposite to clock hands
/// - **Half**: 180° rotation (clockwise and counterclockwise are equivalent)
///
/// # Mathematical Properties
///
/// The angle operations satisfy group properties:
/// - **Identity**: `Zero` is the additive identity
/// - **Inverse**: Each angle has an additive inverse (`CWQuarter` ↔ `ACWQuarter`)
/// - **Closure**: Adding or subtracting angles always produces another valid angle
/// - **Associativity**: `(a + b) + c = a + (b + c)`
///
/// # Usage
///
/// Angles are used throughout the system for:
/// - Defining move rotations (U = Up face by `CWQuarter`)
/// - Composing rotations algebraically
/// - Computing tile positions after face rotations
/// - Calculating geometric transformations in tile permutations
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Angle {
    /// No rotation (0°)
    Zero,
    /// Clockwise quarter turn (90° clockwise)
    CWQuarter,
    /// Half turn (180°)
    Half,
    /// Anti-clockwise quarter turn (90° counterclockwise, equivalent to 270° clockwise)
    ACWQuarter,
}

impl Add for Angle {
    type Output = Angle;

    /// Composes two rotation angles by addition (modulo 360°).
    ///
    /// This implements the group operation for angle composition, following the
    /// natural rule that rotating by angle `a` then by angle `b` is equivalent
    /// to rotating by `a + b` (with angles reduced modulo 360°).
    ///
    /// # Examples
    ///
    /// ```
    /// use rubiks_cube_representation::core::Angle;
    ///
    /// // Two quarter turns make a half turn
    /// assert_eq!(Angle::CWQuarter + Angle::CWQuarter, Angle::Half);
    ///
    /// // Quarter + three-quarters = full rotation = zero
    /// assert_eq!(Angle::CWQuarter + Angle::ACWQuarter, Angle::Zero);
    ///
    /// // Half + half = full rotation = zero
    /// assert_eq!(Angle::Half + Angle::Half, Angle::Zero);
    /// ```
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

impl Sub for Angle {
    type Output = Angle;

    /// Computes the angular difference between two rotation angles.
    ///
    /// This implements angle subtraction (modulo 360°), useful for computing
    /// relative rotations or finding the inverse of a composition.
    ///
    /// # Interpretation
    ///
    /// `a - b` gives the angle needed to go from `b` to `a`. Equivalently,
    /// it's the angle such that `b + (a - b) = a`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rubiks_cube_representation::core::Angle;
    ///
    /// // Subtracting an angle from itself gives zero
    /// assert_eq!(Angle::CWQuarter - Angle::CWQuarter, Angle::Zero);
    ///
    /// // Half - quarter = quarter
    /// assert_eq!(Angle::Half - Angle::CWQuarter, Angle::CWQuarter);
    ///
    /// // Subtracting from zero gives the inverse
    /// assert_eq!(Angle::Zero - Angle::CWQuarter, Angle::ACWQuarter);
    /// ```
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (x, Angle::Zero) => x,
            (Angle::CWQuarter, Angle::ACWQuarter) => Angle::Half,
            (Angle::CWQuarter, Angle::Half) => Angle::ACWQuarter,
            (Angle::CWQuarter, Angle::CWQuarter) => Angle::Zero,
            (Angle::Half, Angle::ACWQuarter) => Angle::ACWQuarter,
            (Angle::Half, Angle::Half) => Angle::Zero,
            (Angle::Half, Angle::CWQuarter) => Angle::CWQuarter,
            (Angle::ACWQuarter, Angle::ACWQuarter) => Angle::Zero,
            (Angle::ACWQuarter, Angle::Half) => Angle::CWQuarter,
            (Angle::ACWQuarter, Angle::CWQuarter) => Angle::Half,
            (Angle::Zero, Angle::CWQuarter) => Angle::ACWQuarter,
            (Angle::Zero, Angle::Half) => Angle::Half,
            (Angle::Zero, Angle::ACWQuarter) => Angle::CWQuarter,
        }
    }
}

impl Angle {
    /// Applies this rotation angle to 2D array indices.
    ///
    /// This method transforms a position `(row, col)` in an N×N grid by rotating
    /// the entire grid by this angle. The rotation is performed around the center
    /// of the grid, maintaining the square boundary.
    ///
    /// # Algorithm
    ///
    /// The transformation for each angle is:
    /// - **Zero**: `(row, col)` → `(row, col)` — no change
    /// - **CWQuarter**: `(row, col)` → `(col, N-1-row)` — 90° clockwise
    /// - **Half**: `(row, col)` → `(N-1-row, N-1-col)` — 180°
    /// - **ACWQuarter**: `(row, col)` → `(N-1-col, row)` — 90° counterclockwise
    ///
    /// # Visual Example (2×2 grid)
    ///
    /// ```text
    /// Original positions:     After CWQuarter:
    /// (0,0) (0,1)             (1,0) (0,0)
    /// (1,0) (1,1)             (1,1) (0,1)
    ///
    /// Or labeling positions A,B,C,D:
    /// A B  →  C A
    /// C D  →  D B
    /// ```
    ///
    /// # Usage
    ///
    /// This is the core transformation used throughout the tile permutation system
    /// to calculate where tiles move when faces are rotated. It's used by:
    /// - `rotate_face_only` to rotate tiles on a single face
    /// - Cube rotation tile permutation calculations
    /// - Face orientation calculations in tile permutations
    ///
    /// # Parameters
    ///
    /// - `N`: The dimension of the grid (compile-time constant)
    /// - `row`: The row index in the original grid (0 to N-1)
    /// - `col`: The column index in the original grid (0 to N-1)
    ///
    /// # Returns
    ///
    /// A tuple `(new_row, new_col)` representing the position after rotation.
    pub fn rotate_indices<const N: usize>(self, row:usize,col:usize) -> (usize,usize) {
        match self {
            Angle::Zero => (row,col),
            Angle::CWQuarter => (col,(N-1)-row),
            Angle::Half => (N-1-row,N-1-col),
            Angle::ACWQuarter => (N-1-col,row),
        }
    }
}