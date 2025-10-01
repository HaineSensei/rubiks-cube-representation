//! Sparse tile permutations and move construction primitives.
//!
//! This module provides the building blocks for constructing move permutations through
//! compositional assembly of partial permutations. The core abstraction is [`PartialTilePerm`],
//! which represents a permutation defined only on a subset of tile positions.
//!
//! # Core Concept
//!
//! Move permutations are built by composing two fundamental operations:
//! 1. **Face rotation**: Rotating tiles on a single face ([`rotate_face_only`])
//! 2. **Edge cycling**: Cycling tiles along the edges of adjacent faces ([`rotate_outside_of_slice`])
//!
//! These operations return [`PartialTilePerm`] instances that can be composed via multiplication
//! to build complete move permutations. Any tiles not explicitly mapped are implicitly fixed
//! (identity mapping).
//!
//! # Key Types and Functions
//!
//! - [`PartialTilePerm`]: Sparse permutation using HashMap representation
//! - [`rotate_face_only`]: Creates a partial permutation rotating one face's tiles
//! - [`rotate_outside_of_slice`]: Creates a partial permutation cycling edge tiles around a slice
//! - [`RestrictionToPartial`]: Trait for converting restrictions to identity partial permutations
//!
//! # Construction Pattern
//!
//! All move types follow the same construction pattern:
//!
//! ```ignore
//! // Example: Basic U move
//! let face_perm = rotate_face_only::<N>(Face::Up, Angle::CWQuarter);
//! let edge_perm = rotate_outside_of_slice::<N>(
//!     Slice { face: Face::Up, slice_index: 0 },
//!     Angle::CWQuarter
//! );
//! let u_move_partial = &face_perm * &edge_perm;
//! let u_move_full = TilePerm::from(&u_move_partial);
//! ```
//!
//! This pattern extends to all move types:
//! - **Basic moves**: One face + one slice edge cycle
//! - **Wide moves**: Multiple faces + multiple slice edge cycles
//! - **Slice moves**: Conditional face (if end slice) + one slice edge cycle
//! - **Range moves**: Multiple conditional faces + multiple slice edge cycles
//! - **Middle moves**: Only edge cycle (no face rotation)
//!
//! # Composition Through Multiplication
//!
//! Partial permutations compose via the `*` operator, following the standard cubing
//! convention where `p1 * p2` means "apply `p1` first, then apply `p2`". The composition
//! algorithm:
//!
//! 1. For each key in `p1`, follow it through `p2` (or keep if `p2` doesn't map it)
//! 2. Add any keys in `p2` that aren't in `p1`
//! 3. Result is a new partial permutation with the combined mapping
//!
//! # Conversion to Full Permutations
//!
//! [`PartialTilePerm`] implements `Into<TilePerm<N>>` by extending the partial mapping
//! with the identity for all unmapped positions. This conversion happens at the final
//! step after all partial permutations have been composed.
//!
//! # Design Rationale
//!
//! Using partial permutations keeps move construction clean and modular:
//! - Each operation (face rotation, edge cycling) is implemented independently
//! - Operations compose naturally through multiplication
//! - No need to manually track which tiles are affected
//! - HashMap provides efficient sparse representation
//!
//! The two helper functions ([`rotate_face_only`], [`rotate_outside_of_slice`]) provide
//! the only geometric logic needed. All move types are then simple compositions of these
//! primitives, with the complexity isolated to these two well-tested functions.

use std::{collections::{hash_set, HashMap, HashSet}, ops::Mul};

use crate::{core::{cube::geometry::FACE_SIDES, rubiks::tiles::{restrictions::{Restriction, Slice}, TilePos}, Angle}, Face};

#[cfg(test)]
mod tests;

/// Sparse tile permutation defined only on a subset of positions.
///
/// A `PartialTilePerm<N>` represents a permutation that is explicitly defined only
/// for certain tile positions, leaving all other positions implicitly mapped to
/// themselves (identity). This enables efficient composition of permutations that
/// only affect specific regions of the cube.
///
/// # Structure
///
/// Internally, this is a `HashMap<TilePos, TilePos>` where:
/// - Keys are the source positions where the permutation is explicitly defined
/// - Values are the corresponding destination positions
/// - Any position not in the map is implicitly fixed (maps to itself)
///
/// # Invariant
///
/// Although not enforced by the type system, all instances should represent actual
/// permutations on their domain. That is, the mapping is bijective on the set of
/// positions that appear as keys:
/// - **Injective**: Different keys map to different values
/// - **Surjective on domain**: Every key position is also mapped to by some key
///   (the permutation only rearranges positions within its domain)
///
/// This invariant is maintained by construction in all internal uses.
///
/// # Conversion to Full Permutation
///
/// `PartialTilePerm<N>` implements `Into<TilePerm<N>>` by extending the partial
/// permutation with the identity for all unmapped positions. This conversion is
/// used when applying the permutation to cube state.
///
/// # Usage
///
/// Partial permutations are used to build up complete move permutations through
/// composition:
/// 1. `rotate_face_only` creates a partial permutation rotating one face
/// 2. `rotate_outside_of_slice` creates a partial permutation for edge tiles
/// 3. These are composed via multiplication to build the complete move permutation
///
/// This compositional approach keeps move implementations clean and modular.
///
/// # Operations
///
/// - **Multiplication** (`*`): Composes partial permutations
/// - **Inverse** ([`inverse`](PartialTilePerm::inverse)): Computes the inverse partial permutation
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct PartialTilePerm<const N: usize>(pub HashMap<TilePos,TilePos>);

impl<const N: usize> PartialTilePerm<N> {
    pub fn inverse(&self) -> Self {
        Self(
            self
            .0
            .iter()
            .map(|(&x,&y)|(y,x))
            .collect()
        )
    }
}

impl<'a, 'b, const N: usize> Mul<&'b PartialTilePerm<N>> for &'a PartialTilePerm<N> {
    type Output = PartialTilePerm<N>;
    
    fn mul(self, rhs: &'b PartialTilePerm<N>) -> Self::Output {
        let mut out = HashMap::new();
        for (&key,value) in &self.0 {
            match rhs.0.get(value) {
                Some(&x) => {
                    out.insert(key,x);
                },
                None => {
                    out.insert(key,*value);
                },
            }
        }
        for (key,&value) in &rhs.0 {
            if !out.contains_key(key) {
                out.insert(*key,value);
            }
        }
        PartialTilePerm(out)
    }
}

pub(crate) trait RestrictionToPartial<const N: usize> : Restriction<N> {
    fn partial_identity(&self) -> PartialTilePerm<N> {
        PartialTilePerm(
            self
            .restricted_positions()
            .map(|x|(x,x))
            .collect()
        )
    }
}

impl<const N: usize> PartialTilePerm<N> {
    pub(crate) fn restriction_domain(&self) -> PartialTileSet<N> {
        PartialTileSet(self.0.keys().cloned().collect())
    }
}

pub(crate) struct PartialTileSet<const N: usize>(HashSet<TilePos>);

impl<const N: usize> Restriction<N> for PartialTileSet<N> 
{
    type Iter = hash_set::IntoIter<TilePos>;

    fn restricted_positions(&self) -> Self::Iter 
    {
        self
        .0
        .clone()
        .into_iter()
    }
}

impl<const N: usize, T: Restriction<N>> RestrictionToPartial<N> for T {}

/// Creates a partial permutation that rotates edge tiles around a slice.
///
/// This function generates a permutation for the tiles on the edges of adjacent faces
/// at a specific slice depth. It handles the cyclic rotation of tiles around the four
/// edges surrounding the slice, implementing the edge-movement portion of slice-based moves.
///
/// # Algorithm
///
/// For a slice at depth `d` from a reference face:
/// 1. Get the four adjacent faces (North, East, South, West edges)
/// 2. For each edge and each position index `i` along that edge:
///    - Find source position at depth `d` on current edge
///    - Find destination position at depth `d` on the edge rotated by `angle`
///    - Map source → destination
/// 3. This creates a cyclic permutation: North → East → South → West → North (for CW)
///
/// The key insight is using `side * angle` to determine which edge receives tiles
/// from the current edge, and `adjacencies.on_side()` to get position information.
///
/// # Usage in Move Implementation
///
/// This is the second building block for move permutations (along with `rotate_face_only`).
/// It handles how edge tiles cycle around when a slice rotates. Every move type uses this:
/// - **Basic moves**: Slice 0 edges cycle around the face
/// - **Slice moves**: Internal slice edges cycle at specified depth
/// - **Wide/Range moves**: Multiple slices each get their edges cycled
///
/// # Parameters
///
/// - `N`: Cube dimension (compile-time constant)
/// - `slice`: The [`Slice`] defining which edges to rotate
/// - `angle`: Rotation angle determining the cyclic shift direction
///
/// # Returns
///
/// A [`PartialTilePerm<N>`] mapping edge tiles to their rotated positions.
pub(crate) fn rotate_outside_of_slice<const N: usize>(slice: Slice, angle: Angle) -> PartialTilePerm<N> {
    let Slice { face, slice_index } = slice;
    let adjacencies = face.adjacencies();
    let map = FACE_SIDES
        .iter()
        .flat_map(move |&side| {
            (0..N)
            .map(move |index| (adjacencies.on_side(side).side_pos_at_depth::<N>(index, slice_index),adjacencies.on_side(side*angle).side_pos_at_depth::<N>(index, slice_index)))
        })
        .collect();
    PartialTilePerm(map)
}

/// Creates a partial permutation that rotates tiles on a single face.
///
/// This function generates a permutation affecting only the N² tiles on the specified
/// face, rotating them by the given angle. All tiles not on this face are implicitly
/// fixed (identity mapping).
///
/// # Algorithm
///
/// For each tile at position `(row, col)` on the face:
/// 1. Apply [`Angle::rotate_indices`] to compute new position `(new_row, new_col)`
/// 2. Map `(face, row, col)` → `(face, new_row, new_col)`
///
/// This implements the geometric rotation of the face's 2D grid while keeping tiles
/// on the same face.
///
/// # Usage in Move Implementation
///
/// This is one of two building blocks for move permutations:
/// - `rotate_face_only`: Handles the face tiles themselves
/// - `rotate_outside_of_slice`: Handles the edge tiles on adjacent faces
///
/// These are composed via multiplication to build complete move permutations. For example,
/// a basic U move composes:
/// ```ignore
/// let face_perm = rotate_face_only::<N>(Face::Up, Angle::CWQuarter);
/// let edge_perm = rotate_outside_of_slice::<N>(
///     Slice { face: Face::Up, slice_index: 0 },
///     Angle::CWQuarter
/// );
/// let u_move = &face_perm * &edge_perm;
/// ```
///
/// # Parameters
///
/// - `N`: Cube dimension (compile-time constant)
/// - `face`: The face to rotate
/// - `angle`: Rotation angle to apply
///
/// # Returns
///
/// A [`PartialTilePerm<N>`] defining the rotation on the face tiles.
pub(crate) fn rotate_face_only<const N: usize>(face: Face, angle: Angle) -> PartialTilePerm<N> {
    let map = (0..N)
        .flat_map(|row|
            (0..N)
            .map(move |col| {
                let base = TilePos { face, row, col } ;
                let (new_row, new_col) = angle.rotate_indices::<N>(row, col);
                let new = TilePos {face, row: new_row, col: new_col };
                (base, new)
            })
        )
        .collect();
   PartialTilePerm(map)
}
