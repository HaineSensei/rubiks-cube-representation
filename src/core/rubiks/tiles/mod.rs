//! Tile-level permutations for complete cube state manipulation.
//!
//! This module extends the permutation concept from faces and diagonals to individual tiles,
//! providing representations for the positions and permutations of all N²×6 tiles on a cube.
//! This enables fine-grained analysis of cube state transformations beyond face-level operations.
//!
//! # Module Organization
//!
//! - [`restrictions`]: Tile subset analysis framework using the [`Restriction`](restrictions::Restriction) trait
//!   - Defines geometric subsets like slices and ranges
//!   - Iterator-based design for on-demand position generation
//!   - Used in partial permutation construction
//!
//! - [`partial`]: Sparse permutations and move construction primitives
//!   - [`PartialTilePerm`](partial::PartialTilePerm) for efficient sparse permutations
//!   - Helper functions [`rotate_face_only`](partial::rotate_face_only) and [`rotate_outside_of_slice`](partial::rotate_outside_of_slice)
//!   - Compositional building blocks for all move types
//!
//! - `implementations`: Conversions from operations to tile permutations (private module)
//!   - `From` implementations for all move types and cube rotations
//!   - Contains the geometric algorithms for permutation construction
//!   - Provides the bridge from abstract operations to concrete state transformations

use std::{array::from_fn, ops::{Index, Mul}};

use crate::{core::rubiks::{moves::{BasicMove, MiddleMove, RangeMove, SliceMove, WideMove}, tiles::{partial::PartialTilePerm, restrictions::Restriction}}, CubeRotation, Face, RubiksState};

mod implementations;
pub mod restrictions;
pub mod partial;

#[cfg(test)]
mod tests;

/// Position of a single tile on an N×N×N Rubik's cube.
///
/// This structure uniquely identifies one of the N²×6 tiles on a cube by specifying
/// which face it's on and its row/column coordinates within that face. It serves as
/// the fundamental position type throughout the tile permutation system.
///
/// # Coordinate System
///
/// Each face uses a 2D coordinate system where:
/// - **Row 0**: The top row when viewing the face in canonical orientation
/// - **Column 0**: The leftmost column when viewing the face in canonical orientation
/// - Both indices range from 0 to N-1
///
/// The top-left position of each face corresponds to that face's
/// [`principal_corner`](crate::core::cube::geometry::Face::principal_corner).
///
/// # Usage
///
/// [`TilePos`] is used throughout the system for:
/// - Indexing into [`RubiksState`](crate::core::rubiks::RubiksState) to get tile colors
/// - Defining tile permutations via [`TilePerm`]
/// - Specifying positions in restriction iterators
/// - Calculating tile movements during rotations and moves
///
/// # Invariants
///
/// For a valid tile position on an N×N×N cube:
/// - `row < N`
/// - `col < N`
/// - `face` is one of the six cube faces
///
/// These invariants are not enforced at the type level but must be maintained
/// by construction to ensure correct behavior.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TilePos {
    /// The face this tile is located on
    pub face: Face,
    /// Row index within the face (0 to N-1)
    pub row: usize,
    /// Column index within the face (0 to N-1)
    pub col: usize
}

/// Grid of tile positions representing how one face transforms under an operation.
///
/// A `TileGrid<N>` is an N×N array where each entry specifies where the tile at
/// that position moves to after applying a cube operation. This represents the
/// permutation of tiles for a single face.
///
/// # Structure
///
/// The grid is stored as `[[TilePos; N]; N]` where:
/// - `vals[row][col]` is the destination position for the tile at `(row, col)`
/// - Each [`TilePos`] can point to any face, not just the current face
/// - The grid preserves the 2D structure of face manipulations
///
/// # Usage
///
/// `TileGrid` is primarily used as a component of [`TilePerm`], with one grid
/// per face storing that face's tile permutation. It is rarely used in isolation.
///
/// # Design Note
///
/// This type intentionally does not implement `Copy` despite being semantically
/// copyable, as large cube dimensions (e.g., 10×10 or larger) would make implicit
/// copying expensive. Users should explicitly clone when needed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TileGrid<const N: usize> {
    /// 2D array mapping source positions to destination positions
    pub vals: [[TilePos;N];N]
}

/// Complete permutation of all tiles on an N×N×N Rubik's cube.
///
/// A `TilePerm<N>` represents a bijective mapping from tile positions to tile positions,
/// describing where each of the N²×6 tiles on the cube moves under a cube operation.
/// This is the fundamental transformation type in the tile permutation system.
///
/// # Mathematical Interpretation
///
/// `TilePerm<N>` represents an element of the symmetric group S_{6N²}, though not all
/// elements of S_{6N²} are realizable as cube operations (only those preserving the
/// cube's geometric constraints). Operations like rotations, moves, and their compositions
/// all convert to `TilePerm<N>` for application to cube state.
///
/// # Structure
///
/// The permutation is stored as six [`TileGrid<N>`] values, one per face:
/// - `up`, `down`, `left`, `right`, `front`, `back`
///
/// Each grid specifies where tiles on that face move to. Looking up `perm.up.vals[r][c]`
/// gives the destination of the tile at row `r`, column `c` on the up face.
///
/// # Indexing
///
/// `TilePerm<N>` implements `Index<TilePos>`, allowing direct lookup:
/// ```ignore
/// let dest: TilePos = perm[source_pos];
/// ```
/// This returns where the tile at `source_pos` moves to under this permutation.
///
/// # Operations
///
/// - **Composition**: Permutations can be multiplied via `*` operator (see [`Mul`](std::ops::Mul) implementations)
/// - **Inverse**: [`TilePerm::inverse`] computes the inverse permutation
/// - **Application**: Use [`CubeOperation::on`] to apply to [`RubiksState`](crate::core::rubiks::RubiksState)
///
/// # Conversion
///
/// Many types convert to `TilePerm<N>` via [`From`]/[`Into`]:
/// - [`CubeRotation`](crate::core::cube::rotations::CubeRotation)
/// - All move types ([`BasicMove`](crate::core::rubiks::moves::BasicMove), etc.)
/// - [`PartialTilePerm<N>`](partial::PartialTilePerm)
///
/// This allows uniform handling of all cube operations through the permutation abstraction.
///
/// # Design Note
///
/// Like [`TileGrid`], this type does not implement `Copy` to avoid expensive implicit
/// copies for large cubes. Most operations work with references (`&TilePerm<N>`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TilePerm<const N: usize> {
    /// Permutation grid for the up face
    pub up: TileGrid<N>,
    /// Permutation grid for the down face
    pub down: TileGrid<N>,
    /// Permutation grid for the left face
    pub left: TileGrid<N>,
    /// Permutation grid for the right face
    pub right: TileGrid<N>,
    /// Permutation grid for the front face
    pub front: TileGrid<N>,
    /// Permutation grid for the back face
    pub back: TileGrid<N>
}

impl<const N: usize> Index<TilePos> for TilePerm<N> {
    type Output = TilePos;

    fn index(&self, index: TilePos) -> &Self::Output {
        let TilePos { face, row, col } = index;
        match face {
            Face::Up => &self.up.vals[row][col],
            Face::Down => &self.down.vals[row][col],
            Face::Left => &self.left.vals[row][col],
            Face::Right => &self.right.vals[row][col],
            Face::Front => &self.front.vals[row][col],
            Face::Back => &self.back.vals[row][col],
        }
    }
}

impl<'a, 'b, const N: usize> Mul<&'b TilePerm<N>> for &'a TilePerm<N> {
    type Output = TilePerm<N>;

    /// Composes two tile permutations.
    ///
    /// This implements the group operation for tile permutations, composing two
    /// permutations to create a new permutation. Following standard cubing notation,
    /// `perm1 * perm2` means "apply `perm1` first, then apply `perm2`".
    ///
    /// # Mathematical Foundation
    ///
    /// The composition is defined by permutation composition:
    /// ```text
    /// (perm2 ∘ perm1)[tile] = perm2[perm1[tile]]
    /// ```
    ///
    /// For each tile position, we:
    /// 1. Look up where `perm1` (self) sends that tile
    /// 2. Look up where `perm2` (rhs) sends that intermediate position
    /// 3. Store the final destination in the result
    ///
    /// # Notation Convention
    ///
    /// This follows the reverse of standard mathematical function composition to
    /// match cubing notation where operations read left-to-right. The expression
    /// `cube * rotation * move` applies rotation first, then move, matching the
    /// natural reading order.
    ///
    /// # Performance
    ///
    /// Composition is O(N²) since it must compute the destination for all 6N² tiles.
    /// For large cubes, consider whether composition is necessary or if operations
    /// can be applied sequentially instead.
    fn mul(self, rhs: &TilePerm<N>) -> Self::Output {
        TilePerm {
            up: TileGrid { vals: from_fn(|row| from_fn(|col| {
                let tile = TilePos { face: Face::Up, row, col };
                rhs[self[tile]]
            })) },
            down: TileGrid { vals: from_fn(|row| from_fn(|col| {
                let tile = TilePos { face: Face::Down, row, col };
                rhs[self[tile]]
            })) },
            left: TileGrid { vals: from_fn(|row| from_fn(|col| {
                let tile = TilePos { face: Face::Left, row, col };
                rhs[self[tile]]
            })) },
            right: TileGrid { vals: from_fn(|row| from_fn(|col| {
                let tile = TilePos { face: Face::Right, row, col };
                rhs[self[tile]]
            })) },
            front: TileGrid { vals: from_fn(|row| from_fn(|col| {
                let tile = TilePos { face: Face::Front, row, col };
                rhs[self[tile]]
            })) },
            back: TileGrid { vals: from_fn(|row| from_fn(|col| {
                let tile = TilePos { face: Face::Back, row, col };
                rhs[self[tile]]
            })) },
        }
    }
}

impl<'a, const N: usize> Mul<&'a Self> for TilePerm<N> {
    type Output = Self;

    /// Convenience multiplication: owned permutation with borrowed permutation.
    ///
    /// Delegates to the core `&TilePerm * &TilePerm` implementation.
    fn mul(self, rhs: &'a Self) -> Self::Output {
        &self * rhs
    }
}

impl<const N: usize> Mul<TilePerm<N>> for &'_ TilePerm<N> {
    type Output = TilePerm<N>;

    /// Convenience multiplication: borrowed permutation with owned permutation.
    ///
    /// Delegates to the core `&TilePerm * &TilePerm` implementation.
    fn mul(self, rhs: TilePerm<N>) -> Self::Output {
        self * &rhs
    }
}

impl<const N: usize> Mul for TilePerm<N> {
    type Output = TilePerm<N>;

    /// Convenience multiplication: both permutations owned.
    ///
    /// Delegates to the core `&TilePerm * &TilePerm` implementation.
    fn mul(self, rhs: Self) -> Self::Output {
        &self * &rhs
    }
}

impl<const N: usize> TilePerm<N> {
    /// Computes the inverse of this tile permutation.
    ///
    /// The inverse permutation undoes the effect of this permutation. Applying
    /// a permutation followed by its inverse (in either order) returns all tiles
    /// to their original positions.
    ///
    /// # Mathematical Property
    ///
    /// For any permutation `p`:
    /// ```
    /// # use rubiks_cube_representation::core::rubiks::tiles::TilePerm;
    /// # use rubiks_cube_representation::core::cube::rotations::{CubeRotation, X};
    /// # let p = TilePerm::<3>::from(&X);
    /// # let identity = TilePerm::<3>::from(&CubeRotation::ID);
    /// // p * p.inverse() = identity
    /// assert_eq!(&p * &p.inverse(), identity);
    /// // p.inverse() * p = identity
    /// assert_eq!(&p.inverse() * &p, identity);
    /// ```
    ///
    /// # Algorithm
    ///
    /// The inverse is computed by reversing the mapping:
    /// - If `perm[source] = dest`, then `inverse[dest] = source`
    ///
    /// This is implemented by:
    /// 1. Creating an identity permutation as a starting point
    /// 2. For each tile position, looking up where it goes under `self`
    /// 3. Setting the inverse to map that destination back to the source
    ///
    /// # Usage
    ///
    /// Inverse permutations are primarily used internally by [`CubeOperation::on`]
    /// when applying operations to cube state. The inverse tells us where to pull
    /// colors from when populating the new state.
    ///
    /// # Performance
    ///
    /// Computing the inverse is O(N²), requiring a full traversal of all tiles.
    pub fn inverse(&self) -> Self {
        let mut result = TilePerm {
            up: TileGrid { vals: from_fn(|row| from_fn(|col| TilePos { face: Face::Up, row, col })) },
            down: TileGrid { vals: from_fn(|row| from_fn(|col| TilePos { face: Face::Down, row, col })) },
            left: TileGrid { vals: from_fn(|row| from_fn(|col| TilePos { face: Face::Left, row, col })) },
            right: TileGrid { vals: from_fn(|row| from_fn(|col| TilePos { face: Face::Right, row, col })) },
            front: TileGrid { vals: from_fn(|row| from_fn(|col| TilePos { face: Face::Front, row, col })) },
            back: TileGrid { vals: from_fn(|row| from_fn(|col| TilePos { face: Face::Back, row, col })) },
        };

        for face in [Face::Up, Face::Down, Face::Left, Face::Right, Face::Front, Face::Back] {
            for row in 0..N {
                for col in 0..N {
                    let source = TilePos { face, row, col };
                    let destination = self[source];
                    let dest_grid = match destination.face {
                        Face::Up => &mut result.up.vals,
                        Face::Down => &mut result.down.vals,
                        Face::Left => &mut result.left.vals,
                        Face::Right => &mut result.right.vals,
                        Face::Front => &mut result.front.vals,
                        Face::Back => &mut result.back.vals,
                    };
                    dest_grid[destination.row][destination.col] = source;
                }
            }
        }

        result
    }
}

/// Trait for operations that can be applied to a cube state.
///
/// This trait provides a uniform interface for applying any cube operation
/// (rotations, moves, or tile permutations) to a [`RubiksState<N>`]. All types
/// that can be converted to [`TilePerm<N>`] automatically implement this trait
/// via a blanket implementation.
///
/// # Core Method
///
/// The [`on`](CubeOperation::on) method takes an operation and a cube state,
/// returning the new state after applying the operation.
///
/// # Design
///
/// This trait is intentionally crate-private (`pub(crate)`) because the multiplication
/// syntax `cube * operation` provides the public API through operator overloading.
/// The trait exists to enable that syntax while keeping implementation details internal.
///
/// # Implementors
///
/// Any type implementing `Into<TilePerm<N>>` automatically implements `CubeOperation<N>`:
/// - [`TilePerm<N>`] itself
/// - [`CubeRotation`](crate::core::cube::rotations::CubeRotation)
/// - All move types ([`BasicMove<N>`](crate::core::rubiks::moves::BasicMove), etc.)
/// - References to the above types (via additional blanket impl)
///
/// # Usage
///
/// End users interact with this trait through multiplication syntax:
/// ```ignore
/// let rotated_cube = &cube * &rotation;
/// let moved_cube = &cube * &BasicMove::<3>::U;
/// ```
pub(crate) trait CubeOperation<const N: usize>: Into<TilePerm<N>> {
    /// Applies this operation to a cube state, returning the transformed state.
    ///
    /// This method converts the operation to a [`TilePerm<N>`] and applies it
    /// to the cube. The implementation uses the permutation's inverse to determine
    /// where to pull colors from when constructing the new state.
    fn on(self, cube: &RubiksState<N>) -> RubiksState<N>;
}

impl<'a, const N: usize, Operation> CubeOperation<N> for &'a Operation 
where
    &'a Operation: Into<TilePerm<N>>
{
    fn on(self, cube: &RubiksState<N>) -> RubiksState<N> {
        let tile_perm: TilePerm<N> = self.into();
        tile_perm.on(cube)
    }
}

impl<const N: usize> CubeOperation<N> for TilePerm<N> {
    fn on(self, cube: &RubiksState<N>) -> RubiksState<N> {
        let perm_inverse = self.inverse();
        RubiksState { 
            up: super::FaceState { 
                vals: 
                from_fn(|row| 
                    from_fn(|col| 
                        cube[
                            perm_inverse[
                                TilePos{face: Face::Up, row, col}
                            ]
                        ]
                    )
                ) 
            }, 
            down: super::FaceState {
                vals:
                from_fn(|row|
                    from_fn(|col|
                        cube[
                            perm_inverse[
                                TilePos{face: Face::Down, row, col}
                            ]
                        ]
                    )
                )
            }, 
            left: super::FaceState {
                vals:
                from_fn(|row|
                    from_fn(|col|
                        cube[
                            perm_inverse[
                                TilePos{face: Face::Left, row, col}
                            ]
                        ]
                    )
                )
            }, 
            right: super::FaceState {
                vals:
                from_fn(|row|
                    from_fn(|col|
                        cube[
                            perm_inverse[
                                TilePos{face: Face::Right, row, col}
                            ]
                        ]
                    )
                )
            }, 
            front: super::FaceState {
                vals:
                from_fn(|row|
                    from_fn(|col|
                        cube[
                            perm_inverse[
                                TilePos{face: Face::Front, row, col}
                            ]
                        ]
                    )
                )
            }, 
            back: super::FaceState {
                vals:
                from_fn(|row|
                    from_fn(|col|
                        cube[
                            perm_inverse[
                                TilePos{face: Face::Back, row, col}
                            ]
                        ]
                    )
                )
            },
        }
    }
}

// notation cube * <Ops multiplied in order> also works once we've implemented something else that won't work as a blanket impl for annoying reasons...
impl<const N: usize, T: CubeOperation<N>> Mul<T> for &RubiksState<N> {
    type Output = RubiksState<N>;

    fn mul(self, rhs: T) -> Self::Output {
        rhs.on(self)
    }
}
// also &cube * ops since might want cube not to be consumed
impl<const N: usize, T: CubeOperation<N>> Mul<T> for RubiksState<N> {
    type Output = RubiksState<N>;

    fn mul(self, rhs: T) -> Self::Output {
        rhs.on(&self)
    }
}

/// Marker trait for cube operations that are not [`TilePerm<N>`] itself.
///
/// This trait serves as a marker to distinguish between operations that need to
/// be converted to tile permutations (rotations and moves) versus tile permutations
/// themselves. It exists to work around Rust's trait coherence restrictions when
/// implementing multiplication operators.
///
/// # Purpose
///
/// Without this trait, implementing `Mul` for both `TilePerm * TilePerm` and
/// `TilePerm * Move` would create overlapping implementations due to the blanket
/// `Into<TilePerm<N>>` bounds. This marker allows the type system to distinguish
/// these cases.
///
/// # Implementors
///
/// This trait is implemented for:
/// - [`BasicMove<N>`](crate::core::rubiks::moves::BasicMove)
/// - [`WideMove<N>`](crate::core::rubiks::moves::WideMove)
/// - [`SliceMove<N>`](crate::core::rubiks::moves::SliceMove)
/// - [`RangeMove<N>`](crate::core::rubiks::moves::RangeMove)
/// - [`MiddleMove<N>`](crate::core::rubiks::moves::MiddleMove)
/// - [`CubeRotation`](crate::core::cube::rotations::CubeRotation)
///
/// Notably, [`TilePerm<N>`] itself does **not** implement this trait.
///
/// # Design Note
///
/// This is a workaround for Rust's coherence rules. In an ideal type system with
/// "disjoint" trait markers, this wouldn't be necessary. It's purely a technical
/// requirement to enable clean multiplication syntax across all operation types.
pub(crate) trait NonTilePermOperation<const N: usize>: Into<TilePerm<N>> {}

impl<const N: usize> NonTilePermOperation<N> for BasicMove<N> {}
impl<const N: usize> NonTilePermOperation<N> for WideMove<N> {}
impl<const N: usize> NonTilePermOperation<N> for SliceMove<N> {}
impl<const N: usize> NonTilePermOperation<N> for RangeMove<N> {}
impl<const N: usize> NonTilePermOperation<N> for MiddleMove<N> {}
impl<const N: usize> NonTilePermOperation<N> for CubeRotation {}

impl<'a, const N: usize> From<&'a PartialTilePerm<N>> for TilePerm<N> {
    fn from(value: &'a PartialTilePerm<N>) -> Self {
        Self {
            up: TileGrid { vals: from_fn(|row| from_fn(|col| {
                let tile = TilePos { face: Face::Up, row, col };
                match value.0.get(&tile) {
                    Some(&x) => x,
                    None => tile,
                }
            }))},
            down: TileGrid { vals: from_fn(|row| from_fn(|col| {
                let tile = TilePos { face: Face::Down, row, col };
                match value.0.get(&tile) {
                    Some(&x) => x,
                    None => tile,
                }
            }))},
            left: TileGrid { vals: from_fn(|row| from_fn(|col| {
                let tile = TilePos { face: Face::Left, row, col };
                match value.0.get(&tile) {
                    Some(&x) => x,
                    None => tile,
                }
            }))},
            right: TileGrid { vals: from_fn(|row| from_fn(|col| {
                let tile = TilePos { face: Face::Right, row, col };
                match value.0.get(&tile) {
                    Some(&x) => x,
                    None => tile,
                }
            }))},
            front: TileGrid { vals: from_fn(|row| from_fn(|col| {
                let tile = TilePos { face: Face::Front, row, col };
                match value.0.get(&tile) {
                    Some(&x) => x,
                    None => tile,
                }
            }))},
            back: TileGrid { vals: from_fn(|row| from_fn(|col| {
                let tile = TilePos { face: Face::Back, row, col };
                match value.0.get(&tile) {
                    Some(&x) => x,
                    None => tile,
                }
            }))},
        }
    }
}

impl<const N: usize> TilePerm<N> {
    /// Checks if two tile permutations agree on all positions within a restriction.
    ///
    /// This method tests whether `self` and `other` map all positions in the given
    /// restriction to the same destinations. It's particularly useful for verifying
    /// that operations affect only their intended tile subsets.
    ///
    /// # Algorithm
    ///
    /// For each position `p` in the restriction:
    /// - Compare `self[p]` with `other[p]`
    /// - Return `true` only if all positions map to the same destination
    ///
    /// # Primary Use Case: Testing Move Implementations
    ///
    /// This method enables elegant verification of move correctness by checking:
    /// 1. The move agrees with the expected permutation on affected slices
    /// 2. The move agrees with identity on unaffected slices (tiles are fixed)
    ///
    /// # Example Testing Pattern
    ///
    /// ```
    /// use rubiks_cube_representation::core::rubiks::tiles::{TilePerm, restrictions::Slice};
    /// use rubiks_cube_representation::core::rubiks::moves::BasicMove;
    /// use rubiks_cube_representation::core::cube::rotations::CubeRotation;
    /// use rubiks_cube_representation::Face;
    ///
    /// // Test that U move affects only the top slice
    /// let u_move = TilePerm::<3>::from(&BasicMove::<3>::U);
    /// let identity = TilePerm::<3>::from(&CubeRotation::ID);
    /// let top_slice = Slice { face: Face::Up, slice_index: 0 };
    /// let second_slice = Slice { face: Face::Up, slice_index: 1 };
    ///
    /// // U move should NOT agree with identity on the top slice (it changes it)
    /// assert!(!u_move.agree_on(&identity, top_slice));
    ///
    /// // U move SHOULD agree with identity on other slices (they're fixed)
    /// assert!(u_move.agree_on(&identity, second_slice));
    /// ```
    ///
    /// # Performance
    ///
    /// The comparison short-circuits on the first disagreement, making it efficient
    /// even for large restrictions. Time complexity is O(k) where k is the number
    /// of positions in the restriction, with early termination on mismatch.
    ///
    /// # Parameters
    ///
    /// - `other`: The permutation to compare against
    /// - `restriction`: Any type implementing [`Restriction<N>`](restrictions::Restriction),
    ///   defining which positions to check
    ///
    /// # Returns
    ///
    /// `true` if both permutations map all restricted positions to the same destinations,
    /// `false` if any position disagrees.
    pub fn agree_on<T: Restriction<N>>(&self, other: &Self, restriction: T) -> bool {
        restriction.restricted_positions().all(|pos| self[pos] == other[pos])
    }
}