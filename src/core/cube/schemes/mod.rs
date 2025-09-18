//! Color schemes for cube representation.
//!
//! This module provides the interface between the abstract rotation system and concrete
//! color representations. It defines how colors are assigned to cube faces and how these
//! assignments change under rotations.
//!
//! # Core Concepts
//!
//! - **Color Schemes**: Abstract mappings from faces to colors ([`ColourScheme`] trait)
//! - **Color Permutations**: Concrete color assignments ([`ColourPerm`] struct)
//! - **Scheme Rotation**: How color schemes transform under cube rotations
//!
//! # Standard Schemes
//!
//! The module provides two standard color schemes:
//! - [`Western`]: The standard Western color arrangement (White-Yellow opposite, etc.)
//! - [`Japanese`]: The Japanese color arrangement (differs in Yellow-Blue placement)
//!
//! # Key Algorithm
//!
//! The [`ColourScheme::rotated`] method bridges between the rotation system and color schemes
//! by using inverse rotations to determine where each face's color comes from after rotation.
//! This enables rotation-invariant cube analysis and solving.

use std::collections::HashMap;
use crate::core::Colour;
use super::geometry::{Face, FACES};
use super::rotations::{CubeRotation, FacePerm};

/// A specific assignment of colors to the six cube faces.
///
/// This struct represents a concrete color scheme where each face has a specific color.
/// Unlike the trait-based schemes ([`Western`], [`Japanese`]), this stores the actual
/// color values and can represent any possible color arrangement, including rotated schemes.
#[derive(Clone, Copy, Debug)]
pub struct ColourPerm {
    /// Color assigned to the up face
    pub up: Colour,
    /// Color assigned to the down face
    pub down: Colour,
    /// Color assigned to the left face
    pub left: Colour,
    /// Color assigned to the right face
    pub right: Colour,
    /// Color assigned to the front face
    pub front: Colour,
    /// Color assigned to the back face
    pub back: Colour,
}

impl From<&HashMap<Face,Colour>> for ColourPerm {
    /// Creates a color permutation from a face-to-color mapping.
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
    /// Creates a color permutation from an owned face-to-color mapping.
    fn from(value: HashMap<Face,Colour>) -> Self {
        ColourPerm::from(&value)
    }
}

impl From<ColourPerm> for HashMap<Face,Colour> {
    /// Converts a color permutation back to a face-to-color mapping.
    fn from(value: ColourPerm) -> Self {
        FACES.iter().map(|&f|(f,value.from_face(f))).collect()
    }
}

/// Marker type for the Western color scheme.
///
/// The Western scheme uses the standard color arrangement common in Western countries:
/// White-Yellow opposite, Red-Orange opposite, Green-Blue opposite.
#[derive(Clone, Copy, Debug)]
pub struct Western;

/// Marker type for the Japanese color scheme.
///
/// The Japanese scheme differs from Western in the Yellow-Blue swap:
/// White-Blue opposite, Red-Orange opposite, Green-Yellow opposite.
#[derive(Clone, Copy, Debug)]
pub struct Japanese;

/// Trait for color schemes that assign colors to cube faces.
///
/// This trait provides the interface between the abstract rotation system and concrete
/// color representations. It allows different color schemes (Western, Japanese, custom)
/// to be used interchangeably while providing common operations like rotation and lookup.
pub trait ColourScheme {
    /// Returns the color assigned to the up face in this scheme.
    fn up(&self) -> Colour;

    /// Returns the color assigned to the down face in this scheme.
    fn down(&self) -> Colour;

    /// Returns the color assigned to the left face in this scheme.
    fn left(&self) -> Colour;

    /// Returns the color assigned to the right face in this scheme.
    fn right(&self) -> Colour;

    /// Returns the color assigned to the front face in this scheme.
    fn front(&self) -> Colour;

    /// Returns the color assigned to the back face in this scheme.
    fn back(&self) -> Colour;

    /// Gets the color for a specific face.
    ///
    /// This is a convenience method that maps from [`Face`] enum values to colors.
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

    /// Returns a new color scheme representing this scheme after applying a rotation.
    ///
    /// This is the key method that bridges between the rotation system and color schemes.
    /// It uses the inverse rotation to determine where each face's color should come from
    /// after the rotation is applied.
    ///
    /// # Algorithm
    ///
    /// 1. Compute the inverse of the given rotation
    /// 2. Convert to a face permutation
    /// 3. For each face, find where its color comes from in the original scheme
    /// 4. Return a [`ColourPerm`] with the resulting color arrangement
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

    /// Finds which face has the specified color in this scheme.
    ///
    /// Returns an error if the color is not present in the scheme.
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
    /// White up face - standard in Western cubing
    fn up(&self) -> Colour { Colour::White }
    /// Yellow down face - opposite of white
    fn down(&self) -> Colour { Colour::Yellow }
    /// Orange left face - opposite of red
    fn left(&self) -> Colour { Colour::Orange }
    /// Red right face - opposite of orange
    fn right(&self) -> Colour { Colour::Red }
    /// Green front face - opposite of blue
    fn front(&self) -> Colour { Colour::Green }
    /// Blue back face - opposite of green
    fn back(&self) -> Colour { Colour::Blue }
}

impl ColourScheme for Japanese {
    /// White up face - same as Western
    fn up(&self) -> Colour { Colour::White }
    /// Blue down face - swapped with back compared to Western
    fn down(&self) -> Colour { Colour::Blue }
    /// Orange left face - same as Western
    fn left(&self) -> Colour { Colour::Orange }
    /// Red right face - same as Western
    fn right(&self) -> Colour { Colour::Red }
    /// Green front face - same as Western
    fn front(&self) -> Colour { Colour::Green }
    /// Yellow back face - swapped with down compared to Western
    fn back(&self) -> Colour { Colour::Yellow }
}

impl ColourScheme for ColourPerm {
    /// Returns the stored color for the up face
    fn up(&self) -> Colour { self.up }
    /// Returns the stored color for the down face
    fn down(&self) -> Colour { self.down }
    /// Returns the stored color for the left face
    fn left(&self) -> Colour { self.left }
    /// Returns the stored color for the right face
    fn right(&self) -> Colour { self.right }
    /// Returns the stored color for the front face
    fn front(&self) -> Colour { self.front }
    /// Returns the stored color for the back face
    fn back(&self) -> Colour { self.back }
}

#[cfg(test)]
mod tests;