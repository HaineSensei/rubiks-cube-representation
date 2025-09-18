
//! Geometric primitives and spatial relationships for cube representation.
//!
//! This module defines the fundamental geometric concepts used throughout the cube rotation system:
//! - **Corners**: Represented as boolean coordinates in 3D space
//! - **Diagonals**: The four main diagonals connecting opposite vertices
//! - **Faces**: The six faces of the cube with standard orientation
//!
//! # Core Design
//!
//! The module centers around two key geometric relationships:
//!
//! 1. **Corner-to-Diagonal Mapping**: Any corner can be mapped to the main diagonal it lies on,
//!    with all diagonals represented by their upper corners for consistency.
//!
//! 2. **Face-to-Corner Ordering**: Each face has canonical corner orderings that support
//!    the conversion between diagonal-based rotations and face-based operations.
//!
//! # Key Functions
//!
//! - [`Face::diag_orientation_following_ulf`]: Maps face orientations to diagonal orderings
//! - [`Face::principal_diag`]: Identifies the principal (upper-left) corner of each face
//! - [`From<CubeCorner> for CubeDiag`]: Converts corners to their containing diagonals
//!
//! These functions work together to enable the rotation system's core algorithm for converting
//! between different representation formats.

/// Represents a cube corner using three boolean coordinates.
///
/// Each corner is uniquely identified by its position relative to the cube's three primary axes:
/// - `up`: Whether the corner is in the upper half of the cube
/// - `left`: Whether the corner is in the left half of the cube
/// - `front`: Whether the corner is in the front half of the cube
///
/// The naming convention follows standard cube orientation where the cube is viewed
/// with a specific face as "front" and "up" pointing towards the viewer's up direction.
/// The field order follows the standard face ordering (Up, Left, Front).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CubeCorner {
    /// True if the corner is in the upper half of the cube
    pub up: bool,
    /// True if the corner is in the left half of the cube
    pub left: bool,
    /// True if the corner is in the front half of the cube
    pub front: bool,
}

impl CubeCorner {
    /// Returns whether this corner touches the specified face.
    ///
    /// A corner touches a face if it lies on that face's side of the cube.
    /// For example, a corner with `up = true` touches the Up face, while
    /// a corner with `up = false` touches the Down face.
    ///
    /// This is useful for validating geometric relationships and ensuring
    /// that corners are positioned correctly relative to faces.
    pub fn touching(self, face: Face) -> bool {
        match face {
            Face::Up => self.up,
            Face::Down => !self.up,
            Face::Left => self.left,
            Face::Right => !self.left,
            Face::Front => self.front,
            Face::Back => !self.front,
        }
    }
}

/// The four main diagonals of the cube, represented by their upper corners.
///
/// Each diagonal connects two opposite vertices of the cube. This enum represents
/// each diagonal by naming its upper corner (the corner in the upper half of the cube).
/// Alternative constants are provided to reference the same diagonals by their lower corners
/// (DFR, DFL, DBR, DBL).
///
/// The naming convention follows standard Rubik's cube notation:
/// - `U` = Up, `D` = Down
/// - `F` = Front, `B` = Back
/// - `L` = Left, `R` = Right
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CubeDiag {
    /// Main diagonal represented by its Up-Right-Front corner
    URF=0,
    /// Main diagonal represented by its Up-Left-Front corner
    ULF=1,
    /// Main diagonal represented by its Up-Right-Back corner
    URB=2,
    /// Main diagonal represented by its Up-Left-Back corner
    ULB=3
}

impl CubeDiag {
    /// Alternative reference to the same diagonal using its Down-Right-Front (lower) corner
    pub const DRF : Self = Self::ULB;
    /// Alternative reference to the same diagonal using its Down-Left-Front (lower) corner
    pub const DLF : Self = Self::URB;
    /// Alternative reference to the same diagonal using its Down-Right-Back (lower) corner
    pub const DRB : Self = Self::ULF;
    /// Alternative reference to the same diagonal using its Down-Left-Back (lower) corner
    pub const DLB : Self = Self::URF;
}

/// The six faces of a cube.
///
/// The naming convention follows standard Rubik's cube notation, representing
/// the faces as they appear when viewing the cube in standard orientation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Face {
    /// The top face of the cube
    Up=0,
    /// The bottom face of the cube
    Down=1,
    /// The left face of the cube
    Left=2,
    /// The right face of the cube
    Right=3,
    /// The front face of the cube
    Front=4,
    /// The back face of the cube
    Back=5
}

/// Array containing all six faces in enum declaration order.
///
/// Useful for iteration over all faces or indexed access by the face's discriminant value.
pub const FACES: [Face;6] = [Face::Up, Face::Down, Face::Left, Face::Right, Face::Front, Face::Back];

impl From<CubeCorner> for CubeDiag {
    /// Converts a cube corner to its corresponding main diagonal.
    ///
    /// This conversion maps any corner of the cube to the main diagonal it lies on.
    /// Since each main diagonal connects two opposite corners, this function always
    /// returns the diagonal identified by its upper corner, regardless of whether
    /// the input corner is the upper or lower corner of that diagonal.
    ///
    /// # Algorithm
    ///
    /// 1. If the corner is already in the upper half (`up = true`), use its front/left coordinates directly
    /// 2. If the corner is in the lower half (`up = false`), flip the front/left coordinates
    ///    to get the corresponding upper corner on the same diagonal
    /// 3. Map the resulting front/left combination to the appropriate diagonal
    ///
    /// This ensures that both corners on the same diagonal map to the same [`CubeDiag`] variant.
    fn from(value: CubeCorner) -> Self {
        let CubeCorner { up, mut left, mut front } = value;
        if !up {
            left = !left;
            front = !front;
        }
        match (front, left) {
            (true, true) => CubeDiag::ULF,
            (true, false) => CubeDiag::URF,
            (false, true) => CubeDiag::ULB,
            (false, false) => CubeDiag::URB,
        }
    }
}

impl Face {
    /// Returns the canonical ordering of the remaining three main diagonals when viewed from this face,
    /// with ULF designated as the first diagonal.
    ///
    /// This function encodes the geometric relationship between face orientations and diagonal orderings,
    /// which is fundamental to converting between diagonal-based cube rotations and face-based operations.
    ///
    /// # Mathematical Foundation
    ///
    /// Each face has a natural way of ordering the four main diagonals (URF, ULF, URB, ULB) when
    /// viewed from that face's perspective. This function assumes ULF is the first diagonal and
    /// returns the 2nd, 3rd, and 4th diagonals in their canonical order for this face.
    ///
    /// The canonical ordering corresponds to the order the diagonals appear in the corners when
    /// viewing that face as the front and tracing the corners in clockwise order.
    ///
    /// # Usage in Rotation System
    ///
    /// This function is critical for the `From<CubeRotation> for FacePerm` conversion:
    /// 1. For each face, get its diagonal ordering via this function
    /// 2. Apply the cube rotation's diagonal permutation to this ordering
    /// 3. Normalize the result so ULF appears first (handling cyclic permutations)
    /// 4. Match the resulting triplet pattern to determine the destination face
    ///
    /// # Returns
    ///
    /// A tuple `(d2, d3, d4)` where:
    /// - `d2`: The second diagonal in this face's canonical ordering
    /// - `d3`: The third diagonal in this face's canonical ordering
    /// - `d4`: The fourth diagonal in this face's canonical ordering
    ///
    /// # Examples
    ///
    /// ```
    /// use rubiks_cube_representation::core::cube::geometry::{Face, CubeDiag};
    ///
    /// // When viewing the Up face, after ULF the diagonals proceed: ULB → URB → URF
    /// assert_eq!(Face::Up.diag_orientation_following_ulf(),
    ///            (CubeDiag::ULB, CubeDiag::URB, CubeDiag::URF));
    ///
    /// // When viewing the Front face, after ULF: URF → ULB → URB
    /// assert_eq!(Face::Front.diag_orientation_following_ulf(),
    ///            (CubeDiag::URF, CubeDiag::ULB, CubeDiag::URB));
    /// ```
    pub fn diag_orientation_following_ulf(&self) -> (CubeDiag,CubeDiag,CubeDiag) {
        use CubeDiag::*;
        match self {
            Face::Up => (ULB,URB,URF),
            Face::Down => (URF,URB,ULB),
            Face::Left => (URB,URF,ULB),
            Face::Right => (ULB,URF,URB),
            Face::Front => (URF,ULB,URB),
            Face::Back => (URB,ULB,URF),
        }
    }

    /// Returns the principal diagonal of this face, which corresponds to the upper-left corner
    /// in the face's internal representation.
    ///
    /// This function provides the starting corner for face-based operations and complements
    /// [`Face::diag_orientation_following_ulf`] to give complete corner ordering information for any face.
    ///
    /// # Usage
    ///
    /// The principal diagonal serves as a reference point for:
    /// - Applying rotations to cube state (not just color schemes)
    /// - Establishing consistent corner indexing across different face orientations
    /// - Converting between face-based and diagonal-based representations
    ///
    /// Together with [`Face::diag_orientation_following_ulf`], this provides the complete corner
    /// ordering for any face: the principal diagonal plus the three remaining diagonals
    /// in their canonical clockwise order.
    ///
    /// # Returns
    ///
    /// The [`CubeDiag`] corresponding to the upper-left corner of this face's internal representation.
    ///
    /// # Examples
    ///
    /// ```
    /// use rubiks_cube_representation::core::cube::geometry::{Face, CubeDiag};
    ///
    /// // The Front face's upper-left corner is ULF
    /// assert_eq!(Face::Front.principal_diag(), CubeDiag::ULF);
    ///
    /// // The Up face's upper-left corner is ULB
    /// assert_eq!(Face::Up.principal_diag(), CubeDiag::ULB);
    /// ```
    pub fn principal_diag(&self) -> CubeDiag {
        use CubeDiag::*;
        match self {
            Face::Up => ULB,
            Face::Down => URB,
            Face::Left => ULB,
            Face::Right => URF,
            Face::Front => ULF,
            Face::Back => URF,
        }
    }

    /// Returns the principal corner of this face, which corresponds to the upper-left corner
    /// in the face's internal representation.
    ///
    /// This function provides the actual corner coordinates for the principal corner,
    /// giving the exact 3D coordinates in the cube's coordinate system.
    ///
    /// # Relationship to Principal Diagonal
    ///
    /// The principal corner always lies on the principal diagonal. If the face name
    /// appears in the principal diagonal's name (e.g., Up face → ULB diagonal),
    /// the corner uses the upper diagonal corner directly. Otherwise, it uses the
    /// corresponding lower corner on the same diagonal.
    ///
    /// # Returns
    ///
    /// A [`CubeCorner`] representing the 3D coordinates of the upper-left corner
    /// of this face's internal representation.
    pub fn principal_corner(&self) -> CubeCorner {
        match self {
            Face::Up => CubeCorner { up: true, left: true, front: false },      // ULB matches (contains U)
            Face::Down => CubeCorner { up: false, left: true, front: true },    // DFL (URB doesn't contain D)
            Face::Left => CubeCorner { up: true, left: true, front: false },    // ULB matches (contains L)
            Face::Right => CubeCorner { up: true, left: false, front: true },   // URF matches (contains R)
            Face::Front => CubeCorner { up: true, left: true, front: true },    // ULF matches (contains F)
            Face::Back => CubeCorner { up: false, left: true, front: false },   // DBL (URF doesn't contain B)
        }
    }
}

#[cfg(test)]
mod tests;