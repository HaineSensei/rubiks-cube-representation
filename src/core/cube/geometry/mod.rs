
//! Geometric primitives and spatial relationships for cube representation.
//!
//! This module defines the fundamental geometric concepts used throughout the cube rotation system:
//! - **Corners**: Represented as boolean coordinates in 3D space
//! - **Diagonals**: The four main diagonals connecting opposite vertices
//! - **Faces**: The six faces of the cube with standard orientation
//! - **Face Adjacencies**: Mapping between faces and their neighboring faces with directional edges
//!
//! # Core Design
//!
//! The module centers around three key geometric relationships:
//!
//! 1. **Corner-to-Diagonal Mapping**: Any corner can be mapped to the main diagonal it lies on,
//!    with all diagonals represented by their upper corners for consistency.
//!
//! 2. **Face-to-Corner Ordering**: Each face has canonical corner orderings that support
//!    the conversion between diagonal-based rotations and face-based operations.
//!
//! 3. **Face-to-Face Adjacencies**: Each face maps to its four neighboring faces with
//!    precise edge specifications, enabling tile permutation calculations.
//!
//! # Key Functions
//!
//! - [`Face::diag_orientation_following_ulf`]: Maps face orientations to diagonal orderings
//! - [`Face::principal_diag`]: Identifies the principal (upper-left) corner of each face
//! - [`Face::adjacencies`]: Returns all four neighboring faces with their edge relationships
//! - [`Face::adjacent`]: Gets the specific neighboring face along a given edge
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

/// Cardinal directions for specifying edges of cube faces.
///
/// This enum provides an intrinsic coordinate system for each face, independent
/// of the observer's viewing angle or the cube's orientation in space. Each face
/// has its own local coordinate system with well-defined cardinal directions.
///
/// # Design Philosophy
///
/// Using cardinal directions (North, East, South, West) avoids the ambiguity
/// inherent in relative directions like "up," "down," "left," "right" which
/// depend on the observer's perspective. Cardinal directions are intrinsic
/// to each face's geometry and remain consistent regardless of how the cube
/// is oriented or viewed.
///
/// # Coordinate System
///
/// Each face defines its cardinal directions according to a consistent
/// convention that aligns with the cube's overall geometric structure:
/// - **North**: The "top" edge when viewing the face in its canonical orientation
/// - **East**: The "right" edge when viewing the face in its canonical orientation
/// - **South**: The "bottom" edge when viewing the face in its canonical orientation
/// - **West**: The "left" edge when viewing the face in its canonical orientation
///
/// # Usage in Adjacency System
///
/// Face sides are used in conjunction with [`AdjacentFace`] to specify precise
/// edge relationships between neighboring faces. This enables accurate tile
/// permutation calculations during face rotations.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FaceSide {
    /// The north edge of a face (top when viewing in canonical orientation)
    North = 0,
    /// The east edge of a face (right when viewing in canonical orientation)
    East = 1,
    /// The south edge of a face (bottom when viewing in canonical orientation)
    South = 2,
    /// The west edge of a face (left when viewing in canonical orientation)
    West = 3
}

/// Array containing all four face sides in enum declaration order.
///
/// Useful for iteration over all sides or indexed access by the side's discriminant value.
pub const FACE_SIDES: [FaceSide; 4] = [FaceSide::North, FaceSide::East, FaceSide::South, FaceSide::West];

/// Represents an adjacent face and the specific edge where adjacency occurs.
///
/// This struct captures the relationship between two neighboring faces on a cube,
/// specifying not just which face is adjacent, but exactly which edge of that
/// face forms the shared boundary. This precision is essential for calculating
/// how tiles move between faces during rotations.
///
/// # Fields
///
/// - **face**: The neighboring [`Face`] that shares an edge
/// - **side**: The specific [`FaceSide`] of the neighboring face where the adjacency occurs
///
/// # Geometric Interpretation
///
/// When face A's north edge is adjacent to face B's south edge, this relationship
/// is represented as `AdjacentFace { face: B, side: South }` from A's perspective.
/// The bidirectional nature ensures that from B's perspective, A would be
/// represented as `AdjacentFace { face: A, side: North }`.
///
/// # Usage in Tile Permutations
///
/// This structure provides the precise edge information needed to calculate
/// which tiles move between faces during rotations. When rotating a face,
/// tiles along its edges must be mapped to the corresponding positions on
/// adjacent faces, and the `side` field specifies exactly which edge receives
/// the tiles.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AdjacentFace {
    /// The neighboring face that shares an edge
    pub face: Face,
    /// The specific edge of the neighboring face where adjacency occurs
    pub side: FaceSide
}

/// Complete adjacency information for a cube face, mapping each edge to its neighbor.
///
/// This struct provides the full adjacency context for a face by specifying
/// which face is adjacent along each of the four cardinal directions. It serves
/// as a comprehensive reference for all edge relationships from a single face's
/// perspective.
///
/// # Fields
///
/// Each field corresponds to one of the four edges of the face:
/// - **north**: The face adjacent to the north edge and its corresponding edge
/// - **east**: The face adjacent to the east edge and its corresponding edge
/// - **south**: The face adjacent to the south edge and its corresponding edge
/// - **west**: The face adjacent to the west edge and its corresponding edge
///
/// # Geometric Consistency
///
/// The adjacency relationships maintain geometric consistency across the entire
/// cube. If face A's north edge connects to face B's south edge, then face B's
/// adjacency information will show face A connected to its south edge.
///
/// # Usage Pattern
///
/// This struct is typically created by [`Face::adjacencies`] and provides
/// convenient access to all neighboring relationships at once, rather than
/// requiring separate queries for each edge direction.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Adjacencies {
    /// The face adjacent to the north edge
    pub north: AdjacentFace,
    /// The face adjacent to the east edge
    pub east: AdjacentFace,
    /// The face adjacent to the south edge
    pub south: AdjacentFace,
    /// The face adjacent to the west edge
    pub west: AdjacentFace
}

impl Face {
    /// Returns complete adjacency information for this face.
    ///
    /// This method provides the full adjacency context by returning an [`Adjacencies`]
    /// struct that maps each of the four cardinal directions to the neighboring face
    /// and the specific edge where the connection occurs.
    ///
    /// # Returns
    ///
    /// An [`Adjacencies`] struct containing [`AdjacentFace`] information for all
    /// four cardinal directions (north, east, south, west).
    ///
    /// # Geometric Foundation
    ///
    /// The adjacency mappings are based on a consistent spatial orientation system
    /// where each face has an intrinsic coordinate system. The returned adjacencies
    /// respect the geometric constraints of a cube, ensuring that shared edges are
    /// correctly identified from both faces' perspectives.
    ///
    /// # Usage
    ///
    /// This method is primarily used when comprehensive edge information is needed,
    /// such as during tile permutation calculations that must consider all edges
    /// of a face simultaneously.
    ///
    /// ```
    /// use rubiks_cube_representation::core::cube::geometry::Face;
    ///
    /// let up_adjacencies = Face::Up.adjacencies();
    /// // Access specific adjacencies
    /// let north_neighbor = up_adjacencies.north;
    /// let east_neighbor = up_adjacencies.east;
    /// ```
    pub fn adjacencies(self) -> Adjacencies {
        use Face::*;
        use FaceSide::*;
        match self {
            Up => Adjacencies {
                north: AdjacentFace { face: Back, side: South }, 
                east: AdjacentFace { face: Right, side: North },
                south: AdjacentFace { face: Front, side: North },
                west: AdjacentFace { face: Left, side: North}
            },
            Down => Adjacencies { 
                north: AdjacentFace { face: Front, side: South }, 
                east: AdjacentFace { face: Right, side: South }, 
                south: AdjacentFace { face: Back, side: North }, 
                west: AdjacentFace { face: Left, side: South }
            },
            Left => Adjacencies { 
                north: AdjacentFace { face: Up, side: West }, 
                east: AdjacentFace { face: Front, side: West }, 
                south: AdjacentFace { face: Down, side: West }, 
                west: AdjacentFace { face: Back, side: West } 
            },
            Right => Adjacencies {
                north: AdjacentFace { face: Up, side: East },
                east: AdjacentFace { face: Back, side: East },
                south: AdjacentFace { face: Down, side: East }, 
                west: AdjacentFace { face: Front, side: East }
            },
            Front => Adjacencies {
                north: AdjacentFace { face: Up, side: South },
                east: AdjacentFace { face: Right, side: West },
                south: AdjacentFace { face: Down, side: North }, 
                west: AdjacentFace { face: Left, side: East }
            },
            Back => Adjacencies {
                north: AdjacentFace { face: Down, side: South },
                east: AdjacentFace { face: Right, side: East },
                south: AdjacentFace { face: Up, side: North }, 
                west: AdjacentFace { face: Left, side: West }
            },
        }
    }

    /// Returns the adjacent face along a specific edge direction.
    ///
    /// This method provides targeted access to adjacency information by returning
    /// the [`AdjacentFace`] for a single specified cardinal direction. It offers
    /// a convenient interface when only one edge relationship is needed.
    ///
    /// # Parameters
    ///
    /// - `side`: The [`FaceSide`] (cardinal direction) for which to retrieve adjacency information
    ///
    /// # Returns
    ///
    /// An [`AdjacentFace`] struct specifying which face is adjacent along the given
    /// edge and the corresponding edge on that neighboring face.
    ///
    /// # Geometric Properties
    ///
    /// The returned adjacency maintains the bidirectional property: if face A's
    /// north edge connects to face B's south edge, then calling `B.adjacent(South)`
    /// will return information pointing back to face A's north edge.
    ///
    /// # Usage
    ///
    /// This method is commonly used in algorithms that need to traverse face
    /// relationships or when calculating tile movements along specific edges.
    ///
    /// ```
    /// use rubiks_cube_representation::core::cube::geometry::{Face, FaceSide};
    ///
    /// let north_adjacent = Face::Up.adjacent(FaceSide::North);
    /// // north_adjacent contains the face north of Up and the corresponding edge
    /// ```
    pub fn adjacent(self, side: FaceSide) -> AdjacentFace {
        let adjacencies = self.adjacencies();
        match side {
            FaceSide::North => adjacencies.north,
            FaceSide::East => adjacencies.east,
            FaceSide::South => adjacencies.south,
            FaceSide::West => adjacencies.west,
        }
    }
}

#[cfg(test)]
mod tests;