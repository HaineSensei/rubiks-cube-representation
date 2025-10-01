//! Cube rotation group implementation using diagonal permutations.
//!
//! This module implements the octahedral group (rotational symmetries of a cube) by representing
//! rotations as permutations of the four main diagonals. This approach provides a clean mathematical
//! foundation for the cube's rotation system.
//!
//! # Mathematical Foundation
//!
//! The cube has 24 rotational symmetries forming the octahedral group O. Rather than representing
//! rotations as 3D matrices or face permutations, this implementation uses the key insight that
//! cube rotations can be uniquely represented as permutations of the four main diagonals.
//!
//! # Core Types
//!
//! - [`CubeRotation`]: A rotation represented as a permutation of main diagonals
//! - [`FacePerm`]: A permutation of the six faces, converted from diagonal permutations
//!
//! # Standard Rotations
//!
//! The module provides constants for the basic rotations:
//! - `X`, `X2`, `X3`: 90°, 180°, and 270° rotations around the X-axis
//! - `Y`, `Y2`, `Y3`: 90°, 180°, and 270° rotations around the Y-axis
//! - `Z`, `Z2`, `Z3`: 90°, 180°, and 270° rotations around the Z-axis
//!
//! These generate the full group of 24 rotations through composition.
//!
//! # Key Algorithm
//!
//! The conversion from [`CubeRotation`] to [`FacePerm`] uses the geometric relationships
//! defined in the [`geometry`](super::geometry) module to translate between diagonal-based
//! and face-based representations. This enables the rotation system to interface with
//! face-oriented operations while maintaining the mathematical elegance of diagonal permutations.
//!
//! # Composition Convention
//!
//! The multiplication operator follows cubing notation where `a * b` means "apply rotation a,
//! then apply rotation b". This is the reverse of standard mathematical function composition.

use std::{array::from_fn, ops::{Index, Mul}};
use crate::core::rubiks::tiles::TilePerm;

use super::geometry::{CubeDiag, Face};

/// A cube rotation represented as a permutation of the four main diagonals.
///
/// This struct encodes rotations of the cube by specifying how the four main diagonals
/// (URF, ULF, URB, ULB) are permuted. The internal array maps each diagonal to its
/// destination: `rotation[i]` is where diagonal `i` moves under this rotation.
///
/// # Representation
///
/// The rotation is stored as `[CubeDiag; 4]` where the index corresponds to:
/// - `0` → URF destination
/// - `1` → ULF destination
/// - `2` → URB destination
/// - `3` → ULB destination
///
/// This diagonal-based representation provides a mathematically clean way to encode
/// the cube's 24 rotational symmetries as elements of the symmetric group S₄.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CubeRotation([CubeDiag;4]);

use CubeDiag::*;

/// 90° rotation around the X-axis
/// URF -> URB, ULF -> ULB, URB -> ULF, ULB -> URF
pub const X: CubeRotation = CubeRotation([URB,ULB,ULF,URF]);

/// 180° rotation around the X-axis, equivalent to X²
pub const X2: CubeRotation = CubeRotation([ULF,URF,ULB,URB]);

/// 270° rotation around the X-axis, equivalent to X³
pub const X3: CubeRotation = CubeRotation([ULB,URB,URF,ULF]);

/// 90° rotation around the Y-axis
/// URF -> ULF, ULF -> ULB, URB -> URF, ULB -> URB
pub const Y: CubeRotation = CubeRotation([ULF,ULB,URF,URB]);

/// 180° rotation around the Y-axis, equivalent to Y²
pub const Y2: CubeRotation = CubeRotation([ULB,URB,ULF,URF]);

/// 270° rotation around the Y-axis, equivalent to Y³
pub const Y3: CubeRotation = CubeRotation([URB,URF,ULB,ULF]);

/// 90° rotation around the Z-axis
/// URF -> ULB, ULF -> URF, URB -> ULF, ULB -> URB
pub const Z: CubeRotation = CubeRotation([ULB,URF,ULF,URB]);

/// 180° rotation around the Z-axis, equivalent to Z²
pub const Z2: CubeRotation = CubeRotation([URB,ULB,URF,ULF]);

/// 270° rotation around the Z-axis, equivalent to Z³
pub const Z3: CubeRotation = CubeRotation([ULF,URB,ULB,URF]);

impl Mul for CubeRotation {
    type Output = CubeRotation;

    /// Composes two rotations to create a new rotation.
    ///
    /// This implements the group operation for the cube's rotation group.
    /// The composition follows cubing notation conventions where `a * b` means
    /// "first apply rotation `a`, then apply rotation `b`".
    ///
    /// **Note**: This is the reverse of standard mathematical function composition,
    /// where `f ∘ g` typically means "apply g first, then f".
    ///
    /// # Implementation
    ///
    /// The composition is computed by composing the underlying permutations:
    /// `(perm2 ∘ perm1)[i] = perm2[perm1[i]]`
    fn mul(self, rhs: Self) -> Self::Output {
        let Self(perm1) = self;
        let Self(perm2) = rhs;
        Self(from_fn(|i| perm2[perm1[i] as usize]))
    }
}

impl Index<CubeDiag> for CubeRotation {
    type Output = CubeDiag;

    /// Returns where the specified diagonal moves under this rotation.
    ///
    /// Allows using `rotation[diagonal]` syntax to query the permutation.
    fn index(&self, index: CubeDiag) -> &Self::Output {
        let Self(perm) = self;
        &perm[index as usize]
    }
}

impl CubeRotation {
    /// The identity rotation (no change to the cube).
    pub const ID: Self = Self([URF,ULF,URB,ULB]);

    /// Alias for the identity rotation using multiplicative notation.
    pub const ONE: Self = Self::ID;

    /// Computes the inverse of this rotation.
    ///
    /// The inverse rotation undoes the effect of this rotation:
    /// `rotation * rotation.inverse() == CubeRotation::ID`
    ///
    /// # Implementation
    ///
    /// For a permutation, the inverse maps each destination back to its source.
    /// If `rotation[i] = j`, then `inverse[j] = i`.
    pub fn inverse(self) -> Self {
        let mut result = [URF; 4]; // Default array
        for (i, &diag) in self.0.iter().enumerate() {
            result[diag as usize] = match i {
                0 => URF,
                1 => ULF,
                2 => URB,
                _ => ULB,
            };
        }
        CubeRotation(result)
    }
}

/// A permutation of the six cube faces.
///
/// This represents how the faces of the cube are rearranged under a rotation.
/// The internal array maps each face to its destination: `perm[i]` is where face `i` moves.
///
/// `FacePerm` is primarily used as the result of converting from [`CubeRotation`],
/// allowing the diagonal-based rotation system to interface with face-based operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FacePerm(pub [Face;6]);

impl FacePerm {
    /// Computes the inverse of this face permutation.
    ///
    /// The inverse permutation undoes the effect of this permutation:
    /// `perm.apply(perm.inverse().apply(face)) == face` for any face.
    pub fn inverse(self) -> Self {
        use Face::*;
        let mut result = [Up; 6]; // Default array
        for (i, &face) in self.0.iter().enumerate() {
            result[face as usize] = match i {
                0 => Up,
                1 => Down,
                2 => Left,
                3 => Right,
                4 => Front,
                _ => Back,
            };
        }
        FacePerm(result)
    }
}

impl Index<Face> for FacePerm {
    type Output = Face;

    /// Returns where the specified face moves under this permutation.
    ///
    /// Allows using `perm[face]` syntax to query the permutation.
    fn index(&self, index: Face) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl From<CubeRotation> for FacePerm {
    /// Converts a diagonal-based rotation to a face-based permutation.
    ///
    /// This is the core algorithm that bridges between the diagonal representation
    /// of rotations and face-based operations. It uses the geometric relationships
    /// defined in the [`geometry`](super::geometry) module.
    ///
    /// # Algorithm
    ///
    /// For each face:
    /// 1. Get the face's canonical diagonal ordering using [`Face::diag_orientation_following_ulf`]
    /// 2. Apply the rotation's diagonal permutation to this ordering
    /// 3. Normalize the result so ULF appears first (handling cyclic permutations)
    /// 4. Match the resulting diagonal triplet to determine the destination face
    ///
    /// This algorithm leverages the fact that each face has a unique diagonal ordering
    /// pattern that is preserved under rotation, allowing face identification through
    /// diagonal relationships.
    ///
    /// [`Face::diag_orientation_following_ulf`]: super::geometry::Face::diag_orientation_following_ulf
    fn from(value: CubeRotation) -> Self {
        use Face::*;
        FacePerm(from_fn(|i| {
            let face = match i {
                0 => Up,
                1 => Down,
                2 => Left,
                3 => Right,
                4 => Front,
                _ => Back
            };
            let (d2_1,d3_1,d4_1) = face.diag_orientation_following_ulf();
            let mapped_orientation = (value[ULF],value[d2_1],value[d3_1],value[d4_1]);
            let (d2_2, d3_2, d4_2) = match mapped_orientation {
                (ULF,d2,d3,d4) => (d2,d3,d4),
                (d4,ULF,d2,d3) => (d2,d3,d4),
                (d3,d4,ULF,d2) => (d2,d3,d4),
                (d2,d3,d4,_) => (d2,d3,d4),
            };
            match (d2_2,d3_2,d4_2) {
                (ULB,URB,URF) => Up,
                (URF,URB,ULB) => Down,
                (URB,URF,ULB) => Left,
                (ULB,URF,URB) => Right,
                (URF,ULB,URB) => Front,
                (URB,ULB,URF) => Back,
                (_,_,_) => unreachable!("This triplet should be a permutation of these 3 terms")
            }
        }))
    }
}

impl<'a, const N: usize> Mul<&'a TilePerm<N>> for &CubeRotation {
    type Output = TilePerm<N>;

    fn mul(self, rhs: &'a TilePerm<N>) -> Self::Output {
        TilePerm::from(self) * rhs
    }
}

impl<const N: usize> Mul<TilePerm<N>> for CubeRotation {
    type Output = TilePerm<N>;

    fn mul(self, rhs: TilePerm<N>) -> Self::Output {
        &self * &rhs
    }
}

impl<const N: usize> Mul<TilePerm<N>> for &CubeRotation {
    type Output = TilePerm<N>;

    fn mul(self, rhs: TilePerm<N>) -> Self::Output {
        self*&rhs
    }
}

impl<'a, const N: usize> Mul<&'a TilePerm<N>> for CubeRotation {
    type Output = TilePerm<N>;

    fn mul(self, rhs: &'a TilePerm<N>) -> Self::Output {
        &self * rhs
    }
}

#[cfg(test)]
mod tests;