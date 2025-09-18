use std::collections::HashMap;
use crate::core::Colour;
use super::geometry::{Face, FACES};
use super::rotations::{CubeRotation, FacePerm};

#[derive(Clone, Copy, Debug)]
pub struct ColourPerm {
    pub up: Colour,
    pub down: Colour,
    pub left: Colour,
    pub right: Colour,
    pub front: Colour,
    pub back: Colour,
}

impl From<&HashMap<Face,Colour>> for ColourPerm {
    fn from(value: &HashMap<Face,Colour>) -> Self {
        use Face::*;
        Self {
            up: value[&Up],
            down: value[&Down],
            left: value[&Left],
            right: value[&Right],
            front: value[&Front],
            back: value[&Back],
        }
    }
}

impl From<HashMap<Face,Colour>> for ColourPerm {
    fn from(value: HashMap<Face,Colour>) -> Self {
        ColourPerm::from(&value)
    }
}

impl From<ColourPerm> for HashMap<Face,Colour> {
    fn from(value: ColourPerm) -> Self {
        FACES.iter().map(|&f|(f,value.from_face(f))).collect()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Western;
#[derive(Clone, Copy, Debug)]
pub struct Japanese;

pub trait ColourScheme {
    fn up(&self) -> Colour;
    fn down(&self) -> Colour;
    fn left(&self) -> Colour;
    fn right(&self) -> Colour;
    fn front(&self) -> Colour;
    fn back(&self) -> Colour;

    fn from_face(&self, face: Face) -> Colour {
        match face {
            Face::Up => self.up(),
            Face::Front => self.front(),
            Face::Left => self.left(),
            Face::Right => self.right(),
            Face::Down => self.down(),
            Face::Back => self.back(),
        }
    }

    fn rotated(&self, rotation: CubeRotation) -> ColourPerm {
        let inv_rotation = rotation.inverse();
        let face_perm: FacePerm = inv_rotation.into();
        use Face::*;
        ColourPerm {
            up: self.from_face(face_perm[Up]),
            down: self.from_face(face_perm[Down]),
            left: self.from_face(face_perm[Left]),
            right: self.from_face(face_perm[Right]),
            front: self.from_face(face_perm[Front]),
            back: self.from_face(face_perm[Back])
        }
    }

    fn get_face(&self, colour: Colour) -> Result<Face,String> {
        for face in FACES {
            if self.from_face(face) == colour {
                return Ok(face);
            }
        }
        Err(format!("Colour {colour:?} not in scheme."))
    }
}

impl ColourScheme for Western {
    fn up(&self) -> Colour { Colour::White }
    fn down(&self) -> Colour { Colour::Yellow }
    fn left(&self) -> Colour { Colour::Orange }
    fn right(&self) -> Colour { Colour::Red }
    fn front(&self) -> Colour { Colour::Green }
    fn back(&self) -> Colour { Colour::Blue }
}

impl ColourScheme for Japanese {
    fn up(&self) -> Colour { Colour::White }
    fn down(&self) -> Colour { Colour::Blue }
    fn left(&self) -> Colour { Colour::Orange }
    fn right(&self) -> Colour { Colour::Red }
    fn front(&self) -> Colour { Colour::Green }
    fn back(&self) -> Colour { Colour::Yellow }
}

impl ColourScheme for ColourPerm {
    fn up(&self) -> Colour { self.up }
    fn down(&self) -> Colour { self.down }
    fn left(&self) -> Colour { self.left }
    fn right(&self) -> Colour { self.right }
    fn front(&self) -> Colour { self.front }
    fn back(&self) -> Colour { self.back }
}

#[cfg(test)]
mod tests;