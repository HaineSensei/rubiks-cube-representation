use std::{array::from_fn, ops::{Index, Mul}};
use super::geometry::{CubeDiag, Face};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CubeRotation([CubeDiag;4]);

use CubeDiag::*;

pub const X: CubeRotation = CubeRotation([UBR,UBL,UFL,UFR]);
pub const X3: CubeRotation = CubeRotation([UBL,UBR,UFR,UFL]); // X*X*X
pub const Y: CubeRotation = CubeRotation([UFL,UBL,UFR,UBR]);
pub const Y3: CubeRotation = CubeRotation([UBR,UFR,UBL,UFL]); // Y*Y*Y (to be verified)
pub const Z: CubeRotation = CubeRotation([UBL,UFR,UFL,UBR]);
pub const Z3: CubeRotation = CubeRotation([UFL,UBR,UBL,UFR]); // Z*Z*Z

impl Mul for CubeRotation {
    type Output = CubeRotation;

    fn mul(self, rhs: Self) -> Self::Output {
        let Self(perm1) = self;
        let Self(perm2) = rhs;
        Self(from_fn(|i| perm2[perm1[i] as usize]))
    }
}

impl Index<CubeDiag> for CubeRotation {
    type Output = CubeDiag;

    fn index(&self, index: CubeDiag) -> &Self::Output {
        let Self(perm) = self;
        &perm[index as usize]
    }
}

impl CubeRotation {
    pub const ID: Self = Self([UFR,UFL,UBR,UBL]);
    pub const ONE: Self = Self::ID;
    pub const E: Self = Self::ID;

    pub fn inverse(self) -> Self {
        let mut result = [UFR; 4]; // Default array
        for (i, &diag) in self.0.iter().enumerate() {
            result[diag as usize] = match i {
                0 => UFR,
                1 => UFL,
                2 => UBR,
                _ => UBL,
            };
        }
        CubeRotation(result)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FacePerm(pub [Face;6]);

impl FacePerm {
    pub fn inverse(self) -> Self {
        use Face::*;
        let mut result = [Up; 6]; // Default array
        for (i, &face) in self.0.iter().enumerate() {
            result[face as usize] = match i {
                0 => Up,
                1 => Down,
                2 => Left,
                3 => Right,
                4 => Front,
                _ => Back,
            };
        }
        FacePerm(result)
    }
}

impl Index<Face> for FacePerm {
    type Output = Face;

    fn index(&self, index: Face) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl From<CubeRotation> for FacePerm {
    fn from(value: CubeRotation) -> Self {
        use Face::*;
        FacePerm(from_fn(|i| {
            let face = match i {
                0 => Up,
                1 => Down,
                2 => Left,
                3 => Right,
                4 => Front,
                _ => Back
            };
            let (d2_1,d3_1,d4_1) = face.diag_orientation_following_ufl();
            let mapped_orientation = (value[UFL],value[d2_1],value[d3_1],value[d4_1]);
            let (d2_2, d3_2, d4_2) = match mapped_orientation {
                (UFL,d2,d3,d4) => (d2,d3,d4),
                (d4,UFL,d2,d3) => (d2,d3,d4),
                (d3,d4,UFL,d2) => (d2,d3,d4),
                (d2,d3,d4,_) => (d2,d3,d4),
            };
            match (d2_2,d3_2,d4_2) {
                (UBL,UBR,UFR) => Up,
                (UFR,UBR,UBL) => Down,
                (UBR,UFR,UBL) => Left,
                (UBL,UFR,UBR) => Right,
                (UFR,UBL,UBR) => Front,
                (UBR,UBL,UFR) => Back,
                (_,_,_) => unreachable!("This triplet should be a permutation of these 3 terms")
            }
        }))
    }
}

#[cfg(test)]
mod tests;