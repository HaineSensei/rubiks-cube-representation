
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CubeCorner {
    pub up: bool,
    pub front: bool,
    pub left: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CubeDiag {
    UFR=0,
    UFL=1,
    UBR=2,
    UBL=3
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Face {
    Up=0,
    Down=1,
    Left=2,
    Right=3,
    Front=4,
    Back=5
}

pub const FACES: [Face;6] = [Face::Up, Face::Down, Face::Left, Face::Right, Face::Front, Face::Back];

impl From<CubeCorner> for CubeDiag {
    fn from(value: CubeCorner) -> Self {
        let CubeCorner { up, mut front, mut left } = value;
        if !up {
            front = !front;
            left = !left;
        }
        match (front,left) {
            (true, true) => CubeDiag::UFL,
            (true, false) => CubeDiag::UFR,
            (false, true) => CubeDiag::UBL,
            (false, false) => CubeDiag::UBR,
        }
    }
}

impl Face {
    pub fn diag_orientation_following_ufl(&self) -> (CubeDiag,CubeDiag,CubeDiag) {
        use CubeDiag::*;
        match self {
            Face::Up => (UBL,UBR,UFR),
            Face::Down => (UFR,UBR,UBL),
            Face::Left => (UBR,UFR,UBL),
            Face::Right => (UBL,UFR,UBR),
            Face::Front => (UFR,UBL,UBR),
            Face::Back => (UBR,UBL,UFR),
        }
    }
}

#[cfg(test)]
mod tests;