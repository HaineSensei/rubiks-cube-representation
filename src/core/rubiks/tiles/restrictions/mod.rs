//! Tile subset analysis through restriction abstractions.
//!
//! This module provides a framework for defining and working with subsets of tile positions
//! on a cube. The core abstraction is the [`Restriction`] trait, which represents any subset
//! of the N²×6 tiles through an iterator interface.
//!
//! # Core Concept
//!
//! Rather than explicitly storing sets of tile positions, restrictions provide iterators
//! that generate positions on demand. This keeps the representation lightweight while
//! supporting arbitrary geometric tile subsets like slices, layers, and other meaningful
//! tile collections.
//!
//! # Primary Use Cases
//!
//! 1. **Slice-based cube analysis**: Identifying which tiles belong to specific slices
//! 2. **Partial permutation construction**: Generating sparse permutations that only
//!    affect certain tiles (see [`partial`](super::partial) module)
//! 3. **Move implementation**: Defining which tiles are affected by slice-based operations
//!
//! # Key Types
//!
//! - [`Restriction`]: Trait for defining tile subsets via iterators
//! - [`Slice`]: A horizontal slice through the cube at a specified depth from a reference face
//! - [`SliceIter`]: Iterator handling end slices (with face) vs middle slices (edges only)
//! - [`SliceRange`]: Multiple consecutive slices as a single restriction
//! - [`SliceRangeIter`]: Iterator chaining multiple slice iterators together
//!
//! # Design Philosophy
//!
//! The iterator-based design keeps restrictions abstract and composable:
//! - Restrictions don't need to materialize full position sets in memory
//! - Multiple restrictions can be combined through iterator chaining
//! - The same restriction can generate positions multiple times without state
//! - Integration with Rust's iterator ecosystem (map, filter, collect, etc.)
//!
//! # Slice Geometry
//!
//! For an N×N×N cube, a [`Slice`] at depth `d` from face `F` contains:
//!
//! **End slices** (`d == 0` or `d == N-1`):
//! - All N² tiles on the associated face
//! - All N tiles on each of four adjacent edges
//! - Total: N² + 4N tiles
//!
//! **Middle slices** (`0 < d < N-1`):
//! - N tiles on each of four adjacent edges (forming a "ring")
//! - No face tiles
//! - Total: 4N tiles
//!
//! A [`SliceRange`] represents multiple consecutive slices, iterating through all tiles
//! in each slice sequentially.
//!
//! # Example Usage
//!
//! ```
//! use rubiks_cube_representation::core::rubiks::tiles::restrictions::Restriction;
//! use rubiks_cube_representation::core::rubiks::tiles::restrictions::Slice;
//! use rubiks_cube_representation::Face;
//!
//! // Define a slice - type annotation helps inference
//! let slice: Slice = Slice { face: Face::Up, slice_index: 1 };
//!
//! // Iterate over all tiles in the slice (on a 3x3x3 cube)
//! for pos in <Slice as Restriction<3>>::restricted_positions(&slice) {
//!     // Process each tile position
//! }
//!
//! // Count tiles in the slice
//! let tile_count = <Slice as Restriction<3>>::restricted_positions(&slice).count();
//! ```
//!
//! # Warning: 1×1×1 Cube Edge Case
//!
//! Restrictions on 1×1×1 cubes may behave inaccurately since there is only one slice
//! across any axis in that case. For N=1, slice index 0 is simultaneously both the first
//! and last slice (since N-1 = 0), which can lead to unexpected behavior in iteration
//! logic that assumes distinct end slices.
//!
//! # Internal Implementation Notes
//!
//! The slice iteration algorithm is carefully designed to:
//! - Visit each tile exactly once in a consistent order
//! - Handle the two geometrically distinct cases (end vs middle slices)
//! - Ensure proper termination through state machine progression
//! - Use `Box<dyn Iterator>` in `SliceRangeIter` to work around type inference limitations

use crate::{core::{cube::geometry::{Adjacencies, FaceSide}, rubiks::tiles::TilePos}, Face};

#[cfg(test)]
mod tests;

/// Trait for defining subsets of tile positions on a cube.
///
/// A restriction represents a subset of the N²×6 tiles on a cube, specified by
/// an iterator over [`TilePos`]. This abstraction enables analysis of cube substructures
/// like slices, layers, and other geometrically meaningful tile collections.
///
/// # Core Concept
///
/// Rather than explicitly storing sets of positions, restrictions provide an iterator
/// that generates positions on demand. This keeps the representation lightweight while
/// supporting arbitrary position subsets.
///
/// # Primary Use Cases
///
/// 1. **Slice-based cube analysis**: Identifying which tiles belong to specific slices
/// 2. **Partial permutation construction**: Generating sparse permutations that only
///    affect certain tiles (see [`PartialTilePerm`](super::partial::PartialTilePerm))
/// 3. **Algorithm development**: Defining tile subsets for move generation and analysis
///
/// # Key Implementors
///
/// - [`Slice`]: A single horizontal slice through the cube at a given depth
/// - `SliceRange`: Multiple consecutive slices (internal type)
/// - `CombinedRestriction`: Union of two restrictions (internal type)
///
/// # Iterator Requirements
///
/// The iterator must:
/// - Yield each position at most once (no duplicates)
/// - Eventually terminate (finite restriction)
/// - Yield only valid positions for the cube dimension `N`
///
/// These requirements are not enforced at the type level but must be maintained
/// by implementations.
///
/// # Example Usage
///
/// ```ignore
/// let slice = Slice { face: Face::Up, slice_index: 1 };
/// for pos in slice.restricted_positions() {
///     // Process each tile in the slice
/// }
/// ```
pub trait Restriction<const N: usize> {
    /// Iterator type yielding tile positions in this restriction.
    type Iter: Iterator<Item = TilePos>;

    /// Returns an iterator over all tile positions in this restriction.
    ///
    /// Each position should be yielded exactly once, and the iterator must
    /// terminate after yielding all positions in the restriction.
    fn restricted_positions(&self) -> Self::Iter;
}



pub struct CombinedRestriction<'a, 'b, const N: usize, First:Restriction<N>, Second:Restriction<N>> {
    first: &'a First,
    second: &'b Second,
}

impl<const N: usize, First: Restriction<N>, Second:Restriction<N>> Restriction<N> for CombinedRestriction<'_, '_, N, First, Second> {
    type Iter = std::iter::Chain<<First as Restriction<N>>::Iter,<Second as Restriction<N>>::Iter>;

    fn restricted_positions(&self) -> Self::Iter {
        self
        .first
        .restricted_positions()
        .chain(self
            .second
            .restricted_positions()
        )
    }
}

/// A horizontal slice through the cube parallel to a specified face.
///
/// A slice represents all tiles at a specific depth from a reference face, forming
/// a "layer" that can be rotated independently. This is the fundamental geometric
/// primitive for implementing cube moves.
///
/// # Structure
///
/// A slice is defined by:
/// - **face**: The reference face from which depth is measured
/// - **slice_index**: Distance from the reference face (0-indexed, 0 = the face itself)
///
/// # Slice Types
///
/// For an N×N×N cube, there are three types of slices:
///
/// ## End Slices (index 0 or N-1)
/// - Include the tiles of one face plus the edges of adjacent faces
/// - Example: For `{face: Up, slice_index: 0}`, includes Up face and the top edges of Front/Right/Back/Left
/// - Special case: `slice_index == N-1` is equivalent to `{face: face.opposite(), slice_index: 0}`
///
/// ## Middle Slices (1 ≤ index < N-1)
/// - Include only edge tiles from the four adjacent faces, no face tiles
/// - Form a "ring" of tiles at the specified depth
///
/// # Usage
///
/// Slices are primarily used to:
/// 1. Implement the [`Restriction`] trait for tile subset iteration
/// 2. Calculate tile permutations in move implementations
/// 3. Define which tiles are affected by slice-based moves
///
/// Used extensively in `rotate_outside_of_slice` to generate move permutations.
///
/// # Examples
///
/// ```
/// use rubiks_cube_representation::core::rubiks::tiles::restrictions::Slice;
/// use rubiks_cube_representation::Face;
///
/// // The top face and its adjacent edges
/// let top_slice = Slice { face: Face::Up, slice_index: 0 };
///
/// // The first internal layer below the top
/// let second_layer = Slice { face: Face::Up, slice_index: 1 };
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Slice {
    /// The reference face from which depth is measured
    pub face: Face,
    /// Distance from the reference face (0 = face itself, N-1 = opposite face)
    pub slice_index: usize
}

/// Iterator over tile positions in a slice.
///
/// This enum-based iterator handles the two geometrically distinct cases for slice
/// iteration: end slices (which include a face) and middle slices (which don't).
/// The iteration algorithm ensures all tiles in the slice are visited exactly once
/// in a consistent order.
///
/// # Variants
///
/// ## `End`
/// Iterates over an end slice (slice_index 0 or N-1), which includes:
/// 1. All N² tiles on the associated face
/// 2. All N tiles on each of the four adjacent edges (4N tiles total)
///
/// Total: N² + 4N tiles
///
/// ## `Mid`
/// Iterates over a middle slice (1 ≤ slice_index < N-1), which includes:
/// - N tiles on each of the four adjacent edges (4N tiles total)
/// - No face tiles
///
/// Total: 4N tiles
///
/// # Iteration Algorithm
///
/// For end slices:
/// 1. Iterate through all positions on the face (row-major order)
/// 2. Then iterate through edges: North → East → South → West
/// 3. Each edge yields N tiles in a consistent indexing direction
///
/// For middle slices:
/// 1. Iterate through edges only: North → East → South → West
/// 2. Each edge yields N tiles at the specified depth
///
/// # State Management
///
/// The iterator tracks:
/// - Current side being iterated (North/East/South/West or None when done)
/// - Position index along the current side (0 to N-1)
/// - For end slices: position within the face (or None when face is complete)
///
/// # Termination
///
/// The iterator terminates when `curr_side` becomes `None`, which happens after
/// processing all four edges. This ensures finite iteration with proper termination.
pub enum SliceIter<const N: usize> {
    /// Iterator for end slices (includes face tiles and adjacent edges)
    End {
        /// The face whose tiles are being iterated
        face: Face,
        /// Current position on the face, or None if face iteration is complete
        end_pos: Option<(usize, usize)>,
        /// Adjacency information for the four edges
        adjacents: Adjacencies,
        /// Current edge being iterated, or None if iteration is complete
        curr_side: Option<FaceSide>,
        /// Index along the current edge (0 to N-1)
        curr_side_pos: usize,
    },
    /// Iterator for middle slices (only adjacent edges, no face)
    Mid {
        /// Adjacency information for the four edges
        adjacents: Adjacencies,
        /// Depth of this slice from the reference face
        slice_index: usize,
        /// Current edge being iterated, or None if iteration is complete
        curr_side: Option<FaceSide>,
        /// Index along the current edge (0 to N-1)
        curr_side_pos: usize,
    }
}

fn increment_pos<const N: usize>(pos: &mut(usize, usize)) {
    if pos.0 < N-1 {
        pos.0 += 1;
    } else {
        pos.0 = 0;
        pos.1 += 1;
    }
}

impl<const N: usize> Iterator for SliceIter<N> {
    type Item = TilePos;

    fn next(&mut self) -> Option<Self::Item> {
        let out: Option<Self::Item>;
        match self {
            SliceIter::End { face, end_pos, adjacents, curr_side, curr_side_pos } => {
                match end_pos {
                    Some(pos) => {
                        out = Some(TilePos { face: *face, row: pos.0, col: pos.1 });
                        increment_pos::<N>(pos);
                        if pos.1 == N {
                            *end_pos = None;
                        }
                    },
                    None => {
                        match curr_side {
                            Some(side) => {
                                let adjacent_face = match side {
                                    FaceSide::North => adjacents.north,
                                    FaceSide::East => adjacents.east,
                                    FaceSide::South => adjacents.south,
                                    FaceSide::West => adjacents.west,
                                };
                                let (row, col) = match adjacent_face.side {
                                    FaceSide::North => (0, *curr_side_pos),
                                    FaceSide::East => (*curr_side_pos, N-1),
                                    FaceSide::South => (N-1, N-1- *curr_side_pos),
                                    FaceSide::West => (N-1- *curr_side_pos, 0),
                                };
                                out = Some(TilePos { face: adjacent_face.face, row, col });
                                if *curr_side_pos < N-1 {
                                    *curr_side_pos += 1;
                                } else {
                                    match side {
                                        FaceSide::North => {
                                            *side = FaceSide::East;
                                        },
                                        FaceSide::East => {
                                            *side = FaceSide::South;
                                        },
                                        FaceSide::South => {
                                            *side = FaceSide::West;
                                        },
                                        FaceSide::West => {
                                            *curr_side = None;
                                        },
                                    }
                                    *curr_side_pos = 0;
                                }
                            },
                            None => {
                                out = None;
                            },
                        }
                    },
                }
            },
            SliceIter::Mid { adjacents, slice_index, curr_side, curr_side_pos} => {
                match curr_side {
                    Some(side) => {
                        let adjacent_face = match side {
                            FaceSide::North => adjacents.north,
                            FaceSide::East => adjacents.east,
                            FaceSide::South => adjacents.south,
                            FaceSide::West => adjacents.west,
                        };
                        out = Some(adjacent_face.side_pos_at_depth::<N>(*curr_side_pos, *slice_index));
                        if *curr_side_pos < N-1 {
                            *curr_side_pos += 1;
                        } else {
                            match side {
                                FaceSide::North => {
                                    *side = FaceSide::East;
                                },
                                FaceSide::East => {
                                    *side = FaceSide::South;
                                },
                                FaceSide::South => {
                                    *side = FaceSide::West;
                                },
                                FaceSide::West => {
                                    *curr_side = None;
                                },
                            }
                            *curr_side_pos = 0;
                        }
                    },
                    None => {
                        out = None;
                    },
                }
            },
        }
        out
    }
}

impl<const N: usize> Restriction<N> for Slice {
    type Iter = SliceIter<N>;
    
    fn restricted_positions(&self) -> Self::Iter {
        let Slice { face, slice_index } = self;
        match slice_index {
            0 => SliceIter::End {
                face: *face,
                end_pos: Some((0,0)),
                adjacents: face.adjacencies(),
                curr_side: Some(FaceSide::North),
                curr_side_pos: 0,
            },
            _ if *slice_index == N-1 => {
                let face = face.opposite();
                SliceIter::End {
                    face,
                    end_pos: Some((0,0)),
                    adjacents: face.adjacencies(),
                    curr_side: Some(FaceSide::North),
                    curr_side_pos: 0,
                }
            },
            _ => SliceIter::Mid {
                adjacents: face.adjacencies(),
                slice_index: *slice_index,
                curr_side: Some(FaceSide::North),
                curr_side_pos: 0
            },
        }
    }
}

/// A range of consecutive slices from a reference face.
///
/// This internal type represents multiple adjacent slices as a single restriction,
/// useful for implementing wide moves and range-based operations that affect
/// multiple layers simultaneously.
///
/// # Structure
///
/// - **face**: Reference face for slice numbering
/// - **start_slice_index**: First slice in the range (inclusive)
/// - **end_slice_index**: Last slice in the range (inclusive)
///
/// # Usage
///
/// `SliceRange` is used internally for range-based move implementations but is
/// not exposed in the public API. Wide moves and range moves create these
/// internally to generate the appropriate tile permutations.
pub struct SliceRange {
    pub face: Face,
    pub start_slice_index: usize,
    pub end_slice_index: usize
}

/// Iterator over tile positions in a range of slices.
///
/// This iterator chains together multiple [`SliceIter`] instances to iterate
/// over all tiles in a consecutive range of slices. It maintains the current
/// slice iterator and a queue of remaining slice iterators to process.
///
/// # Algorithm
///
/// 1. Start with the first slice in the range
/// 2. Iterate through all tiles in that slice
/// 3. When the current slice is exhausted, move to the next slice
/// 4. Repeat until all slices in the range are processed
///
/// # Implementation Note
///
/// The `remaining_iters` field uses `Box<dyn Iterator>` because the type of
/// the iterator chain becomes so complex that even the Rust compiler cannot
/// infer it. This is a pragmatic workaround for type inference limitations
/// when chaining multiple iterator adaptors with closures.
pub struct SliceRangeIter<const N: usize> {
    /// Current slice iterator being processed, or None if starting/finished
    curr_iter: Option<SliceIter<N>>,
    /// Queue of remaining slice iterators to process
    remaining_iters: Box<dyn Iterator<Item = SliceIter<N>>>
}

fn slice_range<const N: usize>(face: Face, first_slice_index: usize, second_slice_index: usize) -> SliceRangeIter<N> {
    let remaining_iters = Box::new((first_slice_index..=second_slice_index)
    .map(move |i| Slice { face, slice_index: i }.restricted_positions()));
    SliceRangeIter {
        curr_iter: None,
        remaining_iters
    }
}

impl<const N: usize> Iterator for SliceRangeIter<N> {
    type Item = TilePos;

    fn next(&mut self) -> Option<Self::Item> {
        let SliceRangeIter { curr_iter, remaining_iters } = self;
        let out;
        match curr_iter {
            Some(iter) => {
                match iter.next() {
                    Some(x) => {
                        out = Some(x);
                    },
                    None => {
                        match remaining_iters.next() {
                            Some(new_iter) => {
                                *curr_iter = Some(new_iter);
                                out = self.next()
                            },
                            None => {
                                *curr_iter = None;
                                out = None;
                            }
                        }
                    },
                }
            },
            None => {
                match remaining_iters.next() {
                    Some(new_iter) => {
                        *curr_iter = Some(new_iter);
                        out = self.next()
                    },
                    None => {
                        out = None;
                    }
                }
            },
        }
        out
    }
}

impl<const N: usize> Restriction<N> for SliceRange {
    type Iter = SliceRangeIter<N>;

    fn restricted_positions(&self) -> Self::Iter {
        slice_range(self.face,self.start_slice_index,self.end_slice_index)
    }
}