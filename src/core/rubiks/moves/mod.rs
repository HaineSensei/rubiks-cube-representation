//! Comprehensive move notation system for Rubik's cube face turns.
//!
//! This module provides a complete representation of all standard cube move types,
//! from basic single-layer turns to complex multi-layer operations. Each move type
//! is paired with an internal representation that captures the essential parameters
//! needed for actual cube manipulation.
//!
//! # Move Type Hierarchy
//!
//! The module defines five categories of moves, each addressing different manipulation needs:
//!
//! ## Basic Moves ([`BasicMove`])
//! Standard single-layer face turns using traditional notation (U, D, L, R, F, B).
//! These form the foundation of all cube solving algorithms and provide the atomic
//! operations for most manipulation sequences.
//!
//! ## Wide Moves ([`WideMove`])
//! Multi-layer turns affecting consecutive layers from a face inward. Essential for
//! larger cubes (4x4x4+) where algorithms often require simultaneous manipulation
//! of multiple layers. Notation: `Uw(2)`, `Rw3(3)`.
//!
//! ## Slice Moves ([`SliceMove`])
//! Individual layer turns targeting specific internal layers by number. Provides
//! precise control over single layers without affecting adjacent ones.
//! Notation: `Us(2)`, `Fs3(4)`.
//!
//! ## Range Moves ([`RangeMove`])
//! Turns affecting specified ranges of consecutive layers. Offers fine-grained
//! control for complex manipulations targeting specific internal regions.
//! Notation: `Ur(2,4)`, `Lr3(1,3)`.
//!
//! ## Middle Moves ([`MiddleMove`])
//! Traditional middle slice notation (M, E, S) for the central layers of
//! odd-dimensioned cubes. Maintains compatibility with classical solving algorithms.
//!
//! # Internal Representations
//!
//! Each move type converts to an internal struct extracting essential geometric information:
//! - [`BasicMoveInternal`]: Face + rotation angle
//! - [`WideMoveInternal`]: Face + rotation + depth
//! - [`SliceMoveInternal`]: Face + rotation + layer number
//! - [`RangeMoveInternal`]: Face + rotation + layer range
//! - [`MiddleMoveInternal`]: Face + rotation (mapped from slice type)
//!
//! This separation enables clean notation while providing the mathematical essence
//! needed for cube operations and algorithm implementation.
//!
//! # Design Philosophy
//!
//! - **Completeness**: Captures all mathematically distinct move types
//! - **Consistency**: Uniform notation patterns across all move categories
//! - **Extensibility**: Designed for arbitrary cube dimensions through parameterization
//! - **Mathematical foundation**: Preserves group-theoretic properties for algorithm analysis

#[cfg(test)]
mod tests;
mod multiplication;

/// Standard single-layer face turns using traditional Rubik's cube notation.
///
/// This enum represents the fundamental moves in cube manipulation - single-layer
/// face turns that affect only the outermost layer of each face. These form the
/// foundation of all cube solving algorithms and standard notation systems.
///
/// # Notation Convention
///
/// Each face is identified by a single letter representing its position when
/// viewing the cube in standard orientation:
/// - **U** (Up): Top face of the cube
/// - **D** (Down): Bottom face of the cube
/// - **L** (Left): Left side face
/// - **R** (Right): Right side face
/// - **F** (Front): Face closest to the viewer
/// - **B** (Back): Face away from the viewer
///
/// # Rotation Amounts
///
/// Each face can be rotated by three different amounts:
/// - **Base move** (e.g., `U`): 90° clockwise rotation when viewing the face directly
/// - **Double move** (e.g., `U2`): 180° rotation (clockwise and counterclockwise equivalent)
/// - **Prime move** (e.g., `U3`): 90° counterclockwise rotation (equivalent to U')
///
/// # Mathematical Properties
///
/// Basic moves follow standard group theory properties:
/// - **Identity**: Four identical moves return to original state (U⁴ = identity)
/// - **Inverse**: Prime moves are inverses of base moves (U · U³ = identity)
/// - **Commutativity**: Opposite face moves commute (U · D = D · U)
/// - **Non-commutativity**: Adjacent face moves generally do not commute
///
/// # Usage in Algorithms
///
/// Basic moves form the atomic operations for:
/// - Beginner solving methods (layer-by-layer approaches)
/// - Advanced algorithms (F2L, OLL, PLL in CFOP method)
/// - Mathematical analysis of cube group structure
/// - Algorithm optimization and move count analysis
#[derive(Clone, Copy, Debug)]
pub enum BasicMove<const DIM: usize> {
    /// Up face 90° clockwise rotation
    U,
    /// Up face 180° rotation
    U2,
    /// Up face 90° counterclockwise rotation (U prime)
    U3,
    /// Down face 90° clockwise rotation
    D,
    /// Down face 180° rotation
    D2,
    /// Down face 90° counterclockwise rotation (D prime)
    D3,
    /// Left face 90° clockwise rotation
    L,
    /// Left face 180° rotation
    L2,
    /// Left face 90° counterclockwise rotation (L prime)
    L3,
    /// Right face 90° clockwise rotation
    R,
    /// Right face 180° rotation
    R2,
    /// Right face 90° counterclockwise rotation (R prime)
    R3,
    /// Front face 90° clockwise rotation
    F,
    /// Front face 180° rotation
    F2,
    /// Front face 90° counterclockwise rotation (F prime)
    F3,
    /// Back face 90° clockwise rotation
    B,
    /// Back face 180° rotation
    B2,
    /// Back face 90° counterclockwise rotation (B prime)
    B3
}
pub use BasicMove::*;

/// Internal representation of a basic move extracting essential geometric parameters.
///
/// This struct provides the mathematical essence of a [`BasicMove`] by separating
/// the face being rotated from the amount of rotation. It serves as an intermediate
/// representation for converting user-friendly notation into the geometric operations
/// needed for actual cube manipulation.
///
/// # Fields
///
/// - **face**: The [`Face`] being rotated (Up, Down, Left, Right, Front, Back)
/// - **amount**: The rotation [`Angle`] (CWQuarter, Half, ACWQuarter)
///
/// # Design Purpose
///
/// This separation allows the move system to:
/// - Convert from notation to mathematical operations cleanly
/// - Support generic cube operations that work with any face/angle combination
/// - Prepare move data for tile permutation calculations
/// - Enable move analysis and optimization algorithms
///
/// # Usage
///
/// `BasicMoveInternal` instances are typically created through [`From`] conversions
/// rather than direct construction, ensuring consistency with the notation system.
pub struct BasicMoveInternal<const DIM: usize> {
    /// The cube face being rotated
    face: Face,
    /// The amount of rotation to apply
    amount: Angle
}

impl<const DIM: usize> BasicMoveInternal<DIM> {
    fn new(face: Face, amount: Angle) -> Self {
        BasicMoveInternal { face, amount }
    }
}

impl<const N: usize> From<BasicMove<N>> for BasicMoveInternal<N> {
    fn from(value: BasicMove<N>) -> Self {
        use Face::*;
        use Angle::*;
        let new = BasicMoveInternal::new;
        match value {
            U => new(Up, CWQuarter),
            U2 => new(Up, Half),
            U3 => new(Up, ACWQuarter),
            D => new(Down, CWQuarter),
            D2 => new(Down, Half),
            D3 => new(Down, ACWQuarter),
            L => new(Left, CWQuarter),
            L2 => new(Left, Half),
            L3 => new(Left, ACWQuarter),
            R => new(Right, CWQuarter),
            R2 => new(Right, Half),
            R3 => new(Right, ACWQuarter),
            F => new(Front, CWQuarter),
            F2 => new(Front, Half),
            F3 => new(Front, ACWQuarter),
            B => new(Back, CWQuarter),
            B2 => new(Back, Half),
            B3 => new(Back, ACWQuarter),
        }
    }
}

/// Multi-layer wide turns affecting multiple consecutive layers from a face.
///
/// Wide moves extend the concept of basic face turns to include multiple layers,
/// starting from the named face and moving inward by a specified depth. This
/// notation is essential for larger cubes (4x4x4 and beyond) where algorithms
/// often require simultaneous manipulation of multiple layers.
///
/// # Notation Convention
///
/// Wide moves use the format `[Face]w[Rotation]([Depth])`:
/// - **Face**: Same as basic moves (U, D, L, R, F, B)
/// - **w**: Indicates wide move (from "wide")
/// - **Rotation**: Optional rotation amount (base = 90°, 2 = 180°, 3 = 270°/prime)
/// - **Depth**: Number of layers from the face to include
///
/// # Examples
///
/// - `Uw(2)`: Rotate top 2 layers clockwise (U and second layer from top)
/// - `Rw2(3)`: Rotate right 3 layers 180° (R, second, and third layers from right)
/// - `Fw3(1)`: Rotate front 1 layer counterclockwise (equivalent to F3, but using wide notation)
///
/// # Layer Counting
///
/// Layers are numbered starting from 1 at the named face:
/// - **Layer 1**: The face itself (Uw(1) equivalent to U)
/// - **Layer 2**: First internal layer adjacent to the face
/// - **Layer n**: nth layer from the face toward the center
///
/// # Mathematical Properties
///
/// Wide moves maintain the same rotational properties as basic moves but affect
/// multiple layers simultaneously. They form a natural extension of the basic
/// move group for larger cube dimensions. Note that opposite wide moves can
/// affect overlapping slices, though they still commute.
#[derive(Clone, Copy, Debug)]
pub enum WideMove<const DIM: usize> {
    /// Up face wide turn, 90° clockwise, with specified depth
    Uw(usize),
    /// Up face wide turn, 180°, with specified depth
    Uw2(usize),
    /// Up face wide turn, 90° counterclockwise, with specified depth
    Uw3(usize),
    /// Down face wide turn, 90° clockwise, with specified depth
    Dw(usize),
    /// Down face wide turn, 180°, with specified depth
    Dw2(usize),
    /// Down face wide turn, 90° counterclockwise, with specified depth
    Dw3(usize),
    /// Left face wide turn, 90° clockwise, with specified depth
    Lw(usize),
    /// Left face wide turn, 180°, with specified depth
    Lw2(usize),
    /// Left face wide turn, 90° counterclockwise, with specified depth
    Lw3(usize),
    /// Right face wide turn, 90° clockwise, with specified depth
    Rw(usize),
    /// Right face wide turn, 180°, with specified depth
    Rw2(usize),
    /// Right face wide turn, 90° counterclockwise, with specified depth
    Rw3(usize),
    /// Front face wide turn, 90° clockwise, with specified depth
    Fw(usize),
    /// Front face wide turn, 180°, with specified depth
    Fw2(usize),
    /// Front face wide turn, 90° counterclockwise, with specified depth
    Fw3(usize),
    /// Back face wide turn, 90° clockwise, with specified depth
    Bw(usize),
    /// Back face wide turn, 180°, with specified depth
    Bw2(usize),
    /// Back face wide turn, 90° counterclockwise, with specified depth
    Bw3(usize)
}
pub use WideMove::*;

/// Internal representation of a wide move with face, rotation, and depth parameters.
///
/// Extracts the essential geometric information from a [`WideMove`] for use in
/// cube manipulation algorithms. The depth parameter specifies how many layers
/// from the named face should be rotated together.
pub struct WideMoveInternal<const DIM: usize> {
    /// The cube face being rotated
    face: Face,
    /// The amount of rotation to apply
    amount: Angle,
    /// Number of layers from the face to include in the rotation
    depth: usize
}

impl<const DIM: usize> WideMoveInternal<DIM> {
    fn new(face: Face, amount: Angle, depth: usize) -> Self {
        WideMoveInternal { face, amount, depth }
    }
}

impl<const N: usize> From<WideMove<N>> for WideMoveInternal<N> {
    fn from(value: WideMove<N>) -> Self {
        use Face::*;
        use Angle::*;
        let new = WideMoveInternal::new;
        match value {
            Uw(depth) => new(Up, CWQuarter, depth),
            Uw2(depth) => new(Up, Half, depth),
            Uw3(depth) => new(Up, ACWQuarter, depth),
            Dw(depth) => new(Down, CWQuarter, depth),
            Dw2(depth) => new(Down, Half, depth),
            Dw3(depth) => new(Down, ACWQuarter, depth),
            Lw(depth) => new(Left, CWQuarter, depth),
            Lw2(depth) => new(Left, Half, depth),
            Lw3(depth) => new(Left, ACWQuarter, depth),
            Rw(depth) => new(Right, CWQuarter, depth),
            Rw2(depth) => new(Right, Half, depth),
            Rw3(depth) => new(Right, ACWQuarter, depth),
            Fw(depth) => new(Front, CWQuarter, depth),
            Fw2(depth) => new(Front, Half, depth),
            Fw3(depth) => new(Front, ACWQuarter, depth),
            Bw(depth) => new(Back, CWQuarter, depth),
            Bw2(depth) => new(Back, Half, depth),
            Bw3(depth) => new(Back, ACWQuarter, depth),
        }
    }
}


/// Individual slice turns targeting specific internal layers by number.
///
/// Slice moves allow precise control over individual internal layers, numbered
/// by their distance from the named face. Unlike wide moves that affect multiple
/// consecutive layers, slice moves affect only the single specified layer.
///
/// # Notation: `[Face]s[Rotation]([Layer])`
///
/// Examples: `Us(2)` (slice 2 from Up), `Rs3(4)` (slice 4 from Right, counterclockwise)
#[derive(Clone, Copy, Debug)]
pub enum SliceMove<const DIM: usize> {
    Us(usize),
    Us2(usize),
    Us3(usize),
    Ds(usize),
    Ds2(usize),
    Ds3(usize),
    Ls(usize),
    Ls2(usize),
    Ls3(usize),
    Rs(usize),
    Rs2(usize),
    Rs3(usize),
    Fs(usize),
    Fs2(usize),
    Fs3(usize),
    Bs(usize),
    Bs2(usize),
    Bs3(usize)
}
pub use SliceMove::*;

/// Internal representation of a slice move with face, rotation, and layer number.
///
/// Extracts the essential parameters from a [`SliceMove`] for cube operations.
pub struct SliceMoveInternal<const DIM: usize> {
    /// The cube face defining the layer numbering reference
    face: Face,
    /// The amount of rotation to apply
    amount: Angle,
    /// The specific layer number to rotate
    layer: usize
}

impl<const DIM: usize> SliceMoveInternal<DIM> {
    fn new(face: Face, amount: Angle, layer: usize) -> Self {
        SliceMoveInternal { face, amount, layer }
    }
}

impl<const N: usize> From<SliceMove<N>> for SliceMoveInternal<N> {
    fn from(value: SliceMove<N>) -> Self {
        use Face::*;
        use Angle::*;
        let new = SliceMoveInternal::new;
        match value {
            Us(layer) => new(Up, CWQuarter, layer),
            Us2(layer) => new(Up, Half, layer),
            Us3(layer) => new(Up, ACWQuarter, layer),
            Ds(layer) => new(Down, CWQuarter, layer),
            Ds2(layer) => new(Down, Half, layer),
            Ds3(layer) => new(Down, ACWQuarter, layer),
            Ls(layer) => new(Left, CWQuarter, layer),
            Ls2(layer) => new(Left, Half, layer),
            Ls3(layer) => new(Left, ACWQuarter, layer),
            Rs(layer) => new(Right, CWQuarter, layer),
            Rs2(layer) => new(Right, Half, layer),
            Rs3(layer) => new(Right, ACWQuarter, layer),
            Fs(layer) => new(Front, CWQuarter, layer),
            Fs2(layer) => new(Front, Half, layer),
            Fs3(layer) => new(Front, ACWQuarter, layer),
            Bs(layer) => new(Back, CWQuarter, layer),
            Bs2(layer) => new(Back, Half, layer),
            Bs3(layer) => new(Back, ACWQuarter, layer),
        }
    }
}

/// Range-based turns affecting multiple consecutive layers within specified bounds.
///
/// Range moves provide fine-grained control over layer selection by specifying
/// both start and end layers. This allows for complex manipulations targeting
/// specific internal regions of larger cubes.
///
/// # Notation: `[Face]r[Rotation]([Start], [End])`
///
/// Examples: `Ur(2,4)` (layers 2-4 from Up), `Lr3(1,3)` (layers 1-3 from Left, counterclockwise)
#[derive(Clone, Copy, Debug)]
pub enum RangeMove<const DIM: usize> {
    Ur(usize,usize),
    Ur2(usize,usize),
    Ur3(usize,usize),
    Dr(usize,usize),
    Dr2(usize,usize),
    Dr3(usize,usize),
    Lr(usize,usize),
    Lr2(usize,usize),
    Lr3(usize,usize),
    Rr(usize,usize),
    Rr2(usize,usize),
    Rr3(usize,usize),
    Fr(usize,usize),
    Fr2(usize,usize),
    Fr3(usize,usize),
    Br(usize,usize),
    Br2(usize,usize),
    Br3(usize,usize)
}
pub use RangeMove::*;

/// Internal representation of a range move with face, rotation, and layer bounds.
///
/// Extracts the essential parameters from a [`RangeMove`] for cube operations.
pub struct RangeMoveInternal<const DIM: usize> {
    /// The cube face defining the layer numbering reference
    face: Face,
    /// The amount of rotation to apply
    amount: Angle,
    /// The first layer in the range (inclusive)
    start_layer: usize,
    /// The last layer in the range (inclusive)
    end_layer: usize
}

impl<const DIM: usize> RangeMoveInternal<DIM> {
    fn new(face: Face, amount: Angle, start_layer: usize, end_layer: usize) -> Self {
        RangeMoveInternal { face, amount, start_layer, end_layer }
    }
}

impl<const N: usize> From<RangeMove<N>> for RangeMoveInternal<N> {
    fn from(value: RangeMove<N>) -> Self {
        use Face::*;
        use Angle::*;
        let new = RangeMoveInternal::new;
        match value {
            Ur(start, end) => new(Up, CWQuarter, start, end),
            Ur2(start, end) => new(Up, Half, start, end),
            Ur3(start, end) => new(Up, ACWQuarter, start, end),
            Dr(start, end) => new(Down, CWQuarter, start, end),
            Dr2(start, end) => new(Down, Half, start, end),
            Dr3(start, end) => new(Down, ACWQuarter, start, end),
            Lr(start, end) => new(Left, CWQuarter, start, end),
            Lr2(start, end) => new(Left, Half, start, end),
            Lr3(start, end) => new(Left, ACWQuarter, start, end),
            Rr(start, end) => new(Right, CWQuarter, start, end),
            Rr2(start, end) => new(Right, Half, start, end),
            Rr3(start, end) => new(Right, ACWQuarter, start, end),
            Fr(start, end) => new(Front, CWQuarter, start, end),
            Fr2(start, end) => new(Front, Half, start, end),
            Fr3(start, end) => new(Front, ACWQuarter, start, end),
            Br(start, end) => new(Back, CWQuarter, start, end),
            Br2(start, end) => new(Back, Half, start, end),
            Br3(start, end) => new(Back, ACWQuarter, start, end),
        }
    }
}

/// Traditional middle slice moves for the central layers of odd-dimensioned cubes.
///
/// These moves represent the classic middle slice notation used in standard
/// cube solving algorithms. Each move targets the single middle layer along
/// one of the three primary axes, following traditional direction conventions.
///
/// # Slice Definitions
///
/// - **M (Middle)**: The layer between L and R faces, rotates like an L turn
/// - **E (Equator)**: The layer between U and D faces, rotates like a D turn
/// - **S (Standing)**: The layer between F and B faces, rotates like an F turn
///
/// # Mathematical Properties
///
/// Middle moves form an important subset of slice operations, particularly
/// useful in algorithms that manipulate cube parity and orientation states.
/// They maintain the same rotational algebra as their corresponding face moves.
#[derive(Clone, Copy, Debug)]
pub enum MiddleMove<const DIM: usize> {
    /// Middle slice 90° clockwise (like L)
    M,
    /// Middle slice 180°
    M2,
    /// Middle slice 90° counterclockwise (like L3)
    M3,
    /// Equator slice 90° clockwise (like D)
    E,
    /// Equator slice 180°
    E2,
    /// Equator slice 90° counterclockwise (like D3)
    E3,
    /// Standing slice 90° clockwise (like F)
    S,
    /// Standing slice 180°
    S2,
    /// Standing slice 90° counterclockwise (like F3)
    S3,
}
pub use MiddleMove::*;

/// Internal representation of a middle move with corresponding face and rotation.
///
/// Maps traditional middle slice notation to the equivalent face operations
/// according to standard cube solving conventions.
pub struct MiddleMoveInternal<const DIM: usize> {
    /// The equivalent face that defines the rotation direction
    face: Face,
    /// The amount of rotation to apply
    amount: Angle
}

impl<const DIM: usize> MiddleMoveInternal<DIM> {
    fn new(face: Face, amount: Angle) -> Self {
        MiddleMoveInternal { face, amount }
    }
}

impl<const N: usize> From<MiddleMove<N>> for MiddleMoveInternal<N> {
    fn from(value: MiddleMove<N>) -> Self {
        use Face::*;
        use Angle::*;
        let new = MiddleMoveInternal::new;
        match value {
            M => new(Left, CWQuarter),
            M2 => new(Left, Half),
            M3 => new(Left, ACWQuarter),
            E => new(Down, CWQuarter),
            E2 => new(Down, Half),
            E3 => new(Down, ACWQuarter),
            S => new(Front, CWQuarter),
            S2 => new(Front, Half),
            S3 => new(Front, ACWQuarter),
        }
    }
}

use crate::{core::{rubiks::tiles::TilePerm, Angle}, Face};

pub(crate) trait Move<const N: usize> : Into<TilePerm<N>> {}

impl<const N: usize> Move<N> for BasicMove<N> {}

impl<const N: usize> Move<N> for WideMove<N> {}

impl<const N: usize> Move<N> for SliceMove<N> {}

impl<const N: usize> Move<N> for RangeMove<N> {}

impl<const N: usize> Move<N> for MiddleMove<N> {}