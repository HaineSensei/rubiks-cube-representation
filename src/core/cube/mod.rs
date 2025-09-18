//! Abstract cube mathematical foundations.
//!
//! This module provides the mathematical abstractions for cube representation and manipulation.
//! It contains three core components that work together to create a complete cube rotation system:
//!
//! # Module Organization
//!
//! - [`geometry`]: Fundamental geometric primitives (corners, diagonals, faces)
//! - [`rotations`]: The octahedral group implementation using diagonal permutations
//! - [`schemes`]: Color scheme abstraction and rotation interface
//!
//! # Mathematical Approach
//!
//! The cube module implements a clean separation between:
//! 1. **Geometric Structure** - How cube elements relate spatially
//! 2. **Group Theory** - How rotations compose and interact
//! 3. **Color Representation** - How abstract rotations map to concrete color schemes
//!
//! This separation enables rotation-invariant analysis and provides a mathematically
//! elegant foundation for cube solving algorithms.

pub mod geometry;
pub mod rotations;
pub mod schemes;

#[cfg(test)]
mod tests;