pub mod moves;

use crate::core::Colour;
use super::cube::geometry::{Face, FACES};
use super::cube::schemes::{ColourScheme, ColourPerm};
use super::cube::rotations::{CubeRotation, X, X3, Y, Y3};

pub struct FaceState<const DIM: usize> {
    pub vals: [[Colour;DIM];DIM]
}

impl<const DIM: usize> FaceState<DIM> {
    fn flat(colour: Colour) -> Self {
        Self { vals: [[colour;DIM];DIM] }
    }
}

/// ```text
/// TOP (UP): U, FRONT: F, BOTTOM (DOWN): D, BACK: B, LEFT: L, RIGHT: R
/// with orientations as if in this net
///  U
/// LFR
///  D
///  B
/// ```
pub struct RubiksState<const DIM: usize> {
    pub up: FaceState<DIM>,
    pub down: FaceState<DIM>,
    pub left: FaceState<DIM>,
    pub right: FaceState<DIM>,
    pub front: FaceState<DIM>,
    pub back: FaceState<DIM>
}

impl<const DIM: usize> RubiksState<DIM> {
    pub fn face_state<'a>(&'a self, face: Face) -> &'a FaceState<DIM> {
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

#[cfg(test)]
mod tests;