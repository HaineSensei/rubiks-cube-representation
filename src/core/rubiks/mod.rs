pub mod moves;
pub mod tiles;

use std::ops::Index;

use crate::core::rubiks::tiles::TilePos;
use crate::core::Colour;
use super::cube::geometry::{Face, FACES};
use super::cube::schemes::{ColourScheme, ColourPerm};
use super::cube::rotations::{CubeRotation, X, X3, Y, Y3};

/// Represents the color state of a single cube face as a DIM×DIM grid.
///
/// This stores the colors of all tiles on one face of the cube. The grid uses
/// a standard orientation where `vals[0][0]` represents the top-left corner when
/// viewing the face directly. The top-left corner position is defined by that face's
/// [`principal_corner`](super::cube::geometry::Face::principal_corner).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FaceState<const DIM: usize> {
    /// 2D array of colors representing the face's tiles
    pub vals: [[Colour;DIM];DIM]
}

impl<const DIM: usize> FaceState<DIM> {
    /// Creates a face state where all tiles have the same color.
    ///
    /// This is used to create solved face states where every tile
    /// on the face shows the same color.
    fn flat(colour: Colour) -> Self {
        Self { vals: [[colour;DIM];DIM] }
    }
}

/// Complete state of a DIM×DIM×DIM Rubik's cube.
///
/// This represents the color state of all six faces of a cube. The faces are
/// oriented according to a standard net layout:
///
/// ```text
/// UP (TOP): U, DOWN (BOTTOM): D, LEFT: L, RIGHT: R, FRONT: F, BACK: B
/// with orientations as if in this net
///  U
/// LFR
///  D
///  B
/// ```
///
/// Each face is stored as a [`FaceState<DIM>`] containing the colors of all
/// tiles on that face. This representation supports cubes of any size
/// through the const generic `DIM` parameter.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RubiksState<const DIM: usize> {
    /// The up face (top of the cube)
    pub up: FaceState<DIM>,
    /// The down face (bottom of the cube)
    pub down: FaceState<DIM>,
    /// The left face
    pub left: FaceState<DIM>,
    /// The right face
    pub right: FaceState<DIM>,
    /// The front face
    pub front: FaceState<DIM>,
    /// The back face
    pub back: FaceState<DIM>
}

impl<const DIM: usize> RubiksState<DIM> {
    /// Returns a reference to the state of the specified face.
    ///
    /// This provides a convenient way to access face data by [`Face`] enum value
    /// rather than accessing the struct fields directly.
    pub fn face_state(&self, face: Face) -> &FaceState<DIM> {
        use Face::*;
        match face {
            Up => &self.up,
            Down => &self.down,
            Left => &self.left,
            Right => &self.right,
            Front => &self.front,
            Back => &self.back,
        }
    }

    /// Checks if the cube is solved in the given color scheme, allowing for any rotation.
    ///
    /// This implements a rotation-invariant solving algorithm that can detect when a cube
    /// is solved regardless of its physical orientation. This is essential for cube analysis
    /// where the cube might be rotated from its standard position.
    ///
    /// # Algorithm: Two-Step Rotation Detection
    ///
    /// 1. **Orient top color**: Find which face in the scheme has the same color as the cube's
    ///    current top face, then rotate the scheme to align the colors.
    /// 2. **Orient front color**: With the top aligned, find which remaining face matches
    ///    the cube's front color, then rotate around the vertical axis to align.
    /// 3. **Check solved state**: Verify that the fully oriented cube matches the scheme.
    ///
    /// This two-step approach can handle any of the 24 possible cube orientations by
    /// systematically reducing the problem to a standard orientation check.
    ///
    /// # Returns
    ///
    /// `true` if the cube is solved in the given scheme (possibly after rotation),
    /// `false` otherwise.
    pub fn is_solved_up_to_rotation_in<Scheme: ColourScheme>(&self, scheme: Scheme) -> bool {
        if DIM == 0 {
            return true
        }
        let top_colour = self.up.vals[0][0];
        println!("Debug: top_colour = {:?}", top_colour);

        if let Ok(scheme_side) = scheme.get_face(top_colour) {
            println!("Debug: found top_colour on scheme face: {:?}", scheme_side);

            let first_edit_scheme = match scheme_side {
                Face::Up => scheme.rotated(CubeRotation::ID),
                Face::Down => scheme.rotated(X*X),
                Face::Left => scheme.rotated(super::cube::rotations::Z),
                Face::Right => scheme.rotated(super::cube::rotations::Z3),
                Face::Front => scheme.rotated(X),
                Face::Back => scheme.rotated(X3),
            };

            println!("Debug: after first rotation, scheme up={:?}, front={:?}",
                     first_edit_scheme.up(), first_edit_scheme.front());

            let front_colour = self.front.vals[0][0];
            println!("Debug: front_colour = {:?}", front_colour);

            let edited_scheme = match first_edit_scheme.get_face(front_colour) {
                Ok(face) => {
                    println!("Debug: found front_colour on face: {:?}", face);
                    match face {
                        Face::Front => first_edit_scheme,
                        Face::Back => first_edit_scheme.rotated(Y*Y),
                        Face::Left => first_edit_scheme.rotated(Y3),
                        Face::Right => first_edit_scheme.rotated(Y),
                        _ => {
                            println!("Debug: unexpected face for front_colour: {:?}", face);
                            return false;
                        }
                    }
                },
                Err(e) => {
                    println!("Debug: front_colour not found in scheme: {}", e);
                    return false;
                }
            };

            println!("Debug: final scheme up={:?}, front={:?}",
                     edited_scheme.up(), edited_scheme.front());

            let result = self.is_solved_in(edited_scheme);
            println!("Debug: is_solved_in result = {}", result);
            result
        } else {
            println!("Debug: top_colour not found in scheme");
            false
        }
    }

    /// Checks if the cube is solved in the given color scheme with exact orientation.
    ///
    /// This verifies that every tile on the cube matches the expected color for that
    /// position according to the given color scheme. Unlike [`is_solved_up_to_rotation_in`],
    /// this requires the cube to be in the exact orientation specified by the scheme.
    ///
    /// # Implementation
    ///
    /// Iterates through every tile position `(face, row, col)` and verifies that
    /// the actual color matches the scheme's expected color for that face.
    pub fn is_solved_in<Scheme: ColourScheme>(&self, scheme: Scheme) -> bool{
        FACES
        .iter()
        .flat_map(|&f|
            (0..DIM)
            .flat_map( move |i|
                (0..DIM).map( move |j| (f,i,j))
            )
        )
        .all(|(f,i,j)| self.face_state(f).vals[i][j]==scheme.from_face(f))
    }

    /// Checks if the cube is solved using its current color configuration.
    ///
    /// This determines if the cube is in a solved state by creating a color scheme
    /// based on the current corner tiles and checking if the entire cube matches
    /// that scheme. This is more flexible than checking against a fixed color
    /// scheme since it adapts to whatever color configuration the cube currently has.
    ///
    /// # Algorithm
    ///
    /// 1. Extract the corner color from each face (using the `[0][0]` position)
    /// 2. Create a [`ColourPerm`] scheme using these colors
    /// 3. Check if the entire cube state matches this derived scheme
    ///
    /// This approach automatically handles cubes that may have been scrambled
    /// and solved in a different color orientation than the standard scheme.
    ///
    /// # Edge Case
    ///
    /// For zero-dimensional cubes (`DIM = 0`), returns `true` since there
    /// are no tiles to be out of place.
    pub fn is_solved(&self) -> bool {
        if DIM == 0 {
            true
        } else {
            let custom_scheme: ColourPerm = ColourPerm {
                up: self.up.vals[0][0],
                down: self.down.vals[0][0],
                left: self.left.vals[0][0],
                right: self.right.vals[0][0],
                front: self.front.vals[0][0],
                back: self.back.vals[0][0]
            };
            self.is_solved_in(custom_scheme)
        }
    }

    /// Creates a solved cube state using the specified color scheme.
    ///
    /// This constructor creates a `RubiksState<DIM>` where every tile on each face
    /// shows the color specified by the scheme for that face. This represents the
    /// canonical solved state for the given color scheme.
    ///
    /// # Parameters
    ///
    /// * `scheme` - A color scheme implementing [`ColourScheme`] that defines
    ///   which color should appear on each face in the solved state
    ///
    /// # Returns
    ///
    /// A new `RubiksState<DIM>` where:
    /// - All tiles on the up face show `scheme.up()`
    /// - All tiles on the down face show `scheme.down()`
    /// - All tiles on the left face show `scheme.left()`
    /// - All tiles on the right face show `scheme.right()`
    /// - All tiles on the front face show `scheme.front()`
    /// - All tiles on the back face show `scheme.back()`
    ///
    /// # Usage
    ///
    /// This is useful for:
    /// - Creating reference states for comparison
    /// - Initializing cubes for testing algorithms
    /// - Generating target states for solving algorithms
    pub fn solved_in<Scheme: ColourScheme>(scheme: Scheme) -> Self {
        Self {
            up: FaceState::flat(scheme.up()),
            down: FaceState::flat(scheme.down()),
            left: FaceState::flat(scheme.left()),
            right: FaceState::flat(scheme.right()),
            front: FaceState::flat(scheme.front()),
            back: FaceState::flat(scheme.back()),
        }
    }
}

impl<const N: usize> Index<TilePos> for RubiksState<N> {
    type Output = Colour;

    /// Returns the color of the tile at the specified position.
    ///
    /// This implementation enables convenient tile color lookup using array indexing syntax:
    /// ```ignore
    /// let pos = TilePos { face: Face::Up, row: 1, col: 2 };
    /// let color: &Colour = &cube[pos];
    /// ```
    ///
    /// # Usage
    ///
    /// This indexing is used throughout the tile permutation system when applying
    /// operations to cube state. The [`CubeOperation::on`](tiles::CubeOperation::on)
    /// method uses this to look up source colors when constructing the transformed state.
    ///
    /// # Panics
    ///
    /// This will panic if `row >= N` or `col >= N`, as those would be out-of-bounds
    /// array accesses. Proper usage requires [`TilePos`] values to satisfy the invariant
    /// that row and column indices are less than the cube dimension.
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

#[cfg(test)]
mod tests;