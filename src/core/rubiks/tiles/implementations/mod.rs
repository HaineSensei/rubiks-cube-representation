//! Conversions from cube operations to tile permutations.
//!
//! This module implements the conversion logic that transforms all cube operations
//! (rotations and moves) into their [`TilePerm<N>`](super::TilePerm) representations.
//! These conversions enable the uniform application of any operation to cube state
//! through the tile permutation abstraction.
//!
//! # Core Conversions
//!
//! The module provides `From` implementations for:
//! - [`CubeRotation`] → [`TilePerm<N>`](super::TilePerm) (3D rotations)
//! - All five move types → [`TilePerm<N>`](super::TilePerm) (face turns)
//!   - [`BasicMove<N>`](crate::core::rubiks::moves::BasicMove)
//!   - [`WideMove<N>`](crate::core::rubiks::moves::WideMove)
//!   - [`SliceMove<N>`](crate::core::rubiks::moves::SliceMove)
//!   - [`RangeMove<N>`](crate::core::rubiks::moves::RangeMove)
//!   - [`MiddleMove<N>`](crate::core::rubiks::moves::MiddleMove)
//!
//! # Move Construction Pattern
//!
//! All move types follow the same compositional pattern using partial permutations:
//!
//! 1. **Convert to internal representation**: Extract `(face, amount, [depth/layer/range])`
//! 2. **Generate face permutation(s)**: Use [`rotate_face_only`] for affected faces
//!    - Slice 0: Rotate the reference face
//!    - Slice N-1: Rotate the opposite face with inverted angle
//!    - Other slices: No face rotation (empty partial permutation)
//! 3. **Generate edge permutation(s)**: Use [`rotate_outside_of_slice`] for affected slices
//! 4. **Compose permutations**: Multiply face and edge partials together
//! 5. **Convert to full permutation**: Transform [`PartialTilePerm`] → [`TilePerm<N>`](super::TilePerm)
//!
//! # Special Cases
//!
//! ## Opposite Face Handling
//!
//! When a slice at depth `N-1` is rotated, it affects the opposite face. The opposite
//! face must rotate in the inverted direction:
//! ```ignore
//! if slice_index == N-1 {
//!     rotate_face_only::<N>(face.opposite(), Angle::Zero - amount)
//! }
//! ```
//!
//! ## Middle Moves
//!
//! Middle moves (M, E, S) only affect internal slices and never rotate faces:
//! ```ignore
//! let middle_index = N / 2;
//! let edge_perm = rotate_outside_of_slice::<N>(
//!     Slice { face, slice_index: middle_index },
//!     amount
//! );
//! // No face rotation needed
//! ```
//!
//! # Cube Rotation Algorithm
//!
//! Cube rotations use a more complex diagonal-based algorithm via [`grid_from_face`]:
//!
//! 1. **Face permutation**: Determine where each face goes using [`FacePerm`](crate::core::cube::rotations::FacePerm)
//! 2. **Diagonal tracking**: Track the principal diagonal to determine orientation
//! 3. **Orientation computation**: Compare diagonal positions to compute rotation angle
//! 4. **Apply rotation**: Use [`Angle::rotate_indices`](crate::core::Angle::rotate_indices) to transform each tile position
//!
//! This algorithm leverages the mathematical insight that cube rotations can be understood
//! through diagonal permutations, with face orientations derived from where diagonals map.
//!
//! # Layer Indexing Convention
//!
//! **User notation** (1-indexed): Layers numbered from 1 at the face
//! - Layer 1 = the face itself
//! - Layer 2 = first internal layer
//! - Layer N = opposite face
//!
//! **Internal representation** (0-indexed): Slice indices from 0
//! - Slice 0 = the face itself
//! - Slice 1 = first internal layer
//! - Slice N-1 = opposite face
//!
//! Conversion: `slice_index = layer - 1`
//!
//! # Implementation Details
//!
//! The module is organized into:
//! - Direct `From` implementations for each move type
//! - Helper module `cube_rotation_tools` containing [`grid_from_face`]
//! - Extensive inline documentation on the geometric algorithms
//!
//! Both owned and borrowed variants are implemented for ergonomics, with borrowed
//! variants containing the actual logic and owned variants delegating.

use crate::core::rubiks::moves::{BasicMove, WideMove, SliceMove, RangeMove, MiddleMove, BasicMoveInternal, WideMoveInternal, SliceMoveInternal, RangeMoveInternal, MiddleMoveInternal};
use crate::core::cube::rotations::CubeRotation;
use crate::core::Angle;
use crate::Face;
use super::{TilePerm, partial::{rotate_face_only, rotate_outside_of_slice, PartialTilePerm}, restrictions::Slice};

impl<const N: usize> From<&BasicMove<N>> for TilePerm<N> {
    /// Converts a basic move to its tile permutation representation.
    ///
    /// This implements the standard single-layer face turn, rotating the face itself
    /// and cycling the edge tiles on adjacent faces. It represents the fundamental
    /// atomic operation in cube solving algorithms.
    ///
    /// # Algorithm
    ///
    /// A basic move consists of two independent permutations that are composed:
    /// 1. **Face rotation**: `rotate_face_only` rotates the N² tiles on the specified face
    /// 2. **Edge cycling**: `rotate_outside_of_slice` cycles the N edge tiles on each adjacent face
    ///
    /// These are composed via multiplication to produce the complete move permutation.
    ///
    /// # Example
    ///
    /// For a U move (Up face clockwise):
    /// - The 9 tiles on the Up face rotate 90° clockwise
    /// - The top rows of Front, Right, Back, Left faces cycle: F→R→B→L→F
    ///
    /// # Implementation Pattern
    ///
    /// This pattern is used by all move types:
    /// 1. Convert move notation to internal representation ([`BasicMoveInternal`](crate::core::rubiks::moves::BasicMoveInternal))
    /// 2. Generate face permutation at slice_index 0
    /// 3. Generate edge permutation at slice_index 0
    /// 4. Compose the two permutations
    /// 5. Convert the partial permutation to a full tile permutation
    fn from(value: &BasicMove<N>) -> Self {
        let BasicMoveInternal { face, amount } = BasicMoveInternal::from(*value);
        let face_perm = rotate_face_only::<N>(face, amount);
        let edge_perm = rotate_outside_of_slice::<N>(Slice { face, slice_index: 0 }, amount);
        let combined = &face_perm * &edge_perm;
        TilePerm::from(&combined)
    }
}

impl<const N: usize> From<BasicMove<N>> for TilePerm<N> {
    fn from(value: BasicMove<N>) -> Self {
        Self::from(&value)
    }
}

impl<const N: usize> From<&WideMove<N>> for TilePerm<N> {
    /// Converts a wide move to its tile permutation representation.
    ///
    /// Wide moves rotate multiple consecutive layers from a face inward. This implementation
    /// composes the permutations for each affected slice, handling the special cases of
    /// end slices (which include face rotations) vs middle slices (which don't).
    ///
    /// See [`From<&BasicMove<N>>`](Self::from) for the core algorithm pattern.
    fn from(value: &WideMove<N>) -> Self {
        let WideMoveInternal { face, amount, depth } = WideMoveInternal::from(*value);
        // Wide move rotates multiple layers from 0 to depth-1
        let mut combined = PartialTilePerm(std::collections::HashMap::new());
        for slice_index in 0..depth {
            let face_perm = if slice_index == 0 {
                rotate_face_only::<N>(face, amount)
            } else if slice_index == N-1 {
                rotate_face_only::<N>(face.opposite(), Angle::Zero - amount)
            } else {
                PartialTilePerm(std::collections::HashMap::new())
            };
            let edge_perm = rotate_outside_of_slice::<N>(Slice { face, slice_index }, amount);
            combined = &combined * &(&face_perm * &edge_perm);
        }
        TilePerm::from(&combined)
    }
}

impl<const N: usize> From<WideMove<N>> for TilePerm<N> {
    fn from(value: WideMove<N>) -> Self {
        Self::from(&value)
    }
}

impl<const N: usize> From<&SliceMove<N>> for TilePerm<N> {
    /// Converts a slice move to its tile permutation representation.
    ///
    /// Slice moves rotate a single layer at a specified depth. The layer number is
    /// 1-indexed in user notation but converted to 0-indexed internally. End slices
    /// (layers 1 and N) include face rotations, middle slices don't.
    ///
    /// See [`From<&BasicMove<N>>`](Self::from) for the core algorithm pattern.
    fn from(value: &SliceMove<N>) -> Self {
        let SliceMoveInternal { face, amount, layer } = SliceMoveInternal::from(*value);
        // Slice move rotates only the specified layer (1-indexed in notation, 0-indexed internally)
        let slice_index = layer - 1;
        // Note: For N=1, slice 0 is both the first and last slice (since N-1 = 0).
        // This means both face rotations would apply, but since rotating a 1x1 grid
        // is always the identity permutation, the redundancy doesn't matter.
        let face_perm = if slice_index == 0 {
            rotate_face_only::<N>(face, amount)
        } else if slice_index == N-1 {
            rotate_face_only::<N>(face.opposite(), Angle::Zero - amount)
        } else {
            PartialTilePerm(std::collections::HashMap::new())
        };
        let edge_perm = rotate_outside_of_slice::<N>(Slice { face, slice_index }, amount);
        let combined = &face_perm * &edge_perm;
        TilePerm::from(&combined)
    }
}

impl<const N: usize> From<SliceMove<N>> for TilePerm<N> {
    fn from(value: SliceMove<N>) -> Self {
        Self::from(&value)
    }
}

impl<const N: usize> From<&RangeMove<N>> for TilePerm<N> {
    /// Converts a range move to its tile permutation representation.
    ///
    /// Range moves rotate multiple layers within a specified range (inclusive on both ends).
    /// This is similar to wide moves but allows specifying arbitrary start and end layers.
    /// Layer numbers are 1-indexed in user notation but converted to 0-indexed internally.
    ///
    /// See [`From<&BasicMove<N>>`](Self::from) for the core algorithm pattern.
    fn from(value: &RangeMove<N>) -> Self {
        let RangeMoveInternal { face, amount, start_layer, end_layer } = RangeMoveInternal::from(*value);
        // Range move rotates layers from start_layer to end_layer (1-indexed in notation, 0-indexed internally)
        let mut combined = PartialTilePerm(std::collections::HashMap::new());
        for slice_index in (start_layer - 1)..=(end_layer - 1) {
            let face_perm = if slice_index == 0 {
                rotate_face_only::<N>(face, amount)
            } else if slice_index == N-1 {
                rotate_face_only::<N>(face.opposite(), Angle::Zero - amount)
            } else {
                PartialTilePerm(std::collections::HashMap::new())
            };
            let edge_perm = rotate_outside_of_slice::<N>(Slice { face, slice_index }, amount);
            combined = &combined * &(&face_perm * &edge_perm);
        }
        TilePerm::from(&combined)
    }
}

impl<const N: usize> From<RangeMove<N>> for TilePerm<N> {
    fn from(value: RangeMove<N>) -> Self {
        Self::from(&value)
    }
}

impl<const N: usize> From<&MiddleMove<N>> for TilePerm<N> {
    /// Converts a middle move to its tile permutation representation.
    ///
    /// Middle moves (M, E, S) rotate the single center slice on odd-dimensioned cubes.
    /// The middle slice is at index N/2 (0-indexed). These moves only affect edge tiles,
    /// never face tiles, so only `rotate_outside_of_slice` is used (no `rotate_face_only`).
    ///
    /// See [`From<&BasicMove<N>>`](Self::from) for the core algorithm pattern.
    fn from(value: &MiddleMove<N>) -> Self {
        let MiddleMoveInternal { face, amount } = MiddleMoveInternal::from(*value);
        // Middle move rotates the center slice (only valid for odd N)
        // For odd N, the middle slice is at index N/2
        let middle_index = N / 2;
        let edge_perm = rotate_outside_of_slice::<N>(Slice { face, slice_index: middle_index }, amount);
        TilePerm::from(&edge_perm)
    }
}

impl<const N: usize> From<MiddleMove<N>> for TilePerm<N> {
    fn from(value: MiddleMove<N>) -> Self {
        Self::from(&value)
    }
}

mod cube_rotation_tools {
    use std::array::from_fn;

    use crate::{core::{cube::rotations::FacePerm, rubiks::tiles::{TileGrid, TilePos}, Angle}, CubeRotation, Face};

    /// Computes the tile permutation grid for a face under a cube rotation.
    ///
    /// This function calculates where each tile on a given face moves to when the entire
    /// cube undergoes a 3D rotation. It handles both the face permutation (which face the
    /// tiles end up on) and the face orientation (how the tiles are arranged on that face).
    ///
    /// # Algorithm
    ///
    /// The algorithm uses principal diagonals to determine face orientation:
    ///
    /// 1. **Determine destination face**: Use [`FacePerm`](crate::core::cube::rotations::FacePerm) to find where `face` goes
    /// 2. **Get diagonal orientations**:
    ///    - Find the principal diagonal of the original face
    ///    - Find where that diagonal maps to under the rotation
    ///    - Use `diag_orientation_following_ulf` to determine the orientation ordering on the destination face
    /// 3. **Compute orientation angle**:
    ///    - Find the position of the destination face's principal diagonal in its orientation ordering
    ///    - Find the position of where the source's principal diagonal mapped to
    ///    - The difference gives the rotation angle needed
    /// 4. **Apply rotation**: Use [`Angle::rotate_indices`] to transform each `(row, col)` position
    ///
    /// # Diagonal-Based Orientation
    ///
    /// The key insight is that cube rotations can be understood through how they permute
    /// the four principal diagonals (URF, ULF, URB, ULB). Each face has a principal diagonal,
    /// and tracking where it goes tells us the face's orientation after rotation.
    ///
    /// The `diag_orientation_following_ulf` method provides a canonical ordering of diagonals
    /// touching each face, which serves as a reference for measuring rotation angles.
    ///
    /// # Parameters
    ///
    /// - `N`: Cube dimension (compile-time constant)
    /// - `face`: The source face whose tiles we're tracking
    /// - `cube_rotation`: The rotation being applied
    ///
    /// # Returns
    ///
    /// A [`TileGrid<N>`] mapping each position on the source face to its destination after rotation.
    pub fn grid_from_face<const N: usize>(face: Face, cube_rotation: &CubeRotation) -> TileGrid<N> {
        TileGrid { vals: from_fn(|row| from_fn(|col| {
            let new_face = FacePerm::from(*cube_rotation)[face];
            let old_principal = face.principal_diag();
            let new_principal = new_face.principal_diag();
            let (d2, d3, d4) = new_face.diag_orientation_following_ulf();
            let new_principal_pos = match new_principal {
                _ if new_principal == d2 => Angle::CWQuarter,
                _ if new_principal == d3 => Angle::Half,
                _ if new_principal == d4 => Angle::ACWQuarter,
                _ => Angle::Zero
            };
            let mapped_old_principal = cube_rotation[old_principal];
            let mapped_old_principal_pos = match mapped_old_principal {
                _ if mapped_old_principal == d2 => Angle::CWQuarter,
                _ if mapped_old_principal == d3 => Angle::Half,
                _ if mapped_old_principal == d4 => Angle::ACWQuarter,
                _ => Angle::Zero
            };

            let difference = mapped_old_principal_pos - new_principal_pos;

            let (new_row, new_col) = difference.rotate_indices::<N>(row, col);
            TilePos { face: new_face, row: new_row, col: new_col }
        })) }
    }
}


impl<const N: usize> From<&CubeRotation> for TilePerm<N> {
    /// Converts a cube rotation to its tile permutation representation.
    ///
    /// This conversion implements the action of a cube rotation on tile positions,
    /// computing where each tile moves when the entire cube is rotated in 3D space.
    ///
    /// # Algorithm Overview
    ///
    /// For each face on the cube:
    /// 1. Determine which face it moves to under the rotation (via [`FacePerm`](crate::core::cube::rotations::FacePerm))
    /// 2. Determine the orientation change using diagonal analysis
    /// 3. Apply the orientation change to compute new tile positions
    ///
    /// The complex geometric calculation is delegated to `grid_from_face` which handles
    /// the diagonal-based orientation computation.
    ///
    /// # Mathematical Foundation
    ///
    /// Cube rotations are elements of the octahedral group acting on the cube. This
    /// conversion computes the induced action on individual tiles by combining:
    /// - Face permutation (which face goes where)
    /// - Face orientation (how the face is rotated when it arrives)
    ///
    /// The orientation is computed using principal diagonals and the
    /// `diag_orientation_following_ulf` algorithm.
    ///
    /// # Usage
    ///
    /// This conversion enables applying rotations to cube state:
    /// ```ignore
    /// let rotation = CubeRotation::X;
    /// let perm = TilePerm::<3>::from(&rotation);
    /// let rotated_cube = perm.on(&cube);
    /// ```
    fn from(value: &CubeRotation) -> Self {
        use cube_rotation_tools::*;
        use Face::*;
        TilePerm {
            up: grid_from_face(Up, value),
            down: grid_from_face(Down, value),
            left: grid_from_face(Left, value),
            right: grid_from_face(Right, value),
            front: grid_from_face(Front, value),
            back: grid_from_face(Back, value),
        }
    }
}

impl<const N: usize> From<CubeRotation> for TilePerm<N> {
    fn from(value: CubeRotation) -> Self {
        Self::from(&value)
    }
}

#[cfg(test)]
mod tests;