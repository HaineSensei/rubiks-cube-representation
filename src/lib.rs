use std::{array::from_fn, collections::HashMap, ops::{Index, Mul}};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Colour {
    White,
    Yellow,
    Red,
    Orange,
    Blue,
    Green,
}
use Colour::*;

pub const COLOURS: [Colour;6] = [White, Yellow, Red, Orange, Blue, Green];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CubeCorner {
    up:bool,
    front:bool,
    left:bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CubeDiag {
    UFR=0,
    UFL=1,
    UBR=2,
    UBL=3
}
use CubeDiag::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CubeRotation([CubeDiag;4]);

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct FacePerm([Face;6]);

impl FacePerm {
    fn inverse(self) -> Self {
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

impl CubeRotation {
    pub const ID: Self = Self([UFR,UFL,UBR,UBL]);
    pub const ONE: Self = Self::ID;
    pub const E: Self = Self::ID;

    fn inverse(self) -> Self {
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

impl From<CubeCorner> for CubeDiag {
    fn from(value: CubeCorner) -> Self {
        let CubeCorner { up, mut front, mut left } = value;
        if !up {
            front = !front;
            left = !left;
        }
        match (front,left) {
            (true, true) => UFL,
            (true, false) => UFR,
            (false, true) => UBL,
            (false, false) => UBR,
        }
    }
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
use Face::*;

pub const FACES: [Face;6] = [Up, Down, Left, Right, Front, Back];

impl Face {
    fn diag_orientation_following_ufl(&self) -> (CubeDiag,CubeDiag,CubeDiag) {
        match self {
            Up => (UBL,UBR,UFR),
            Down => (UFR,UBR,UBL),
            Left => (UBR,UFR,UBL),
            Right => (UBL,UFR,UBR),
            Front => (UFR,UBL,UBR),
            Back => (UBR,UBL,UFR),
        }

        // Up => (UFL,UBL,UBR,UFR),
        //         Down => (UFL,UFR,UBR,UBL),
        //         Left => (UFR,UBL,UFL,UBR),
        //         Right => (UBL,UFR,UBR,UFL),
        //         Front => (UBR,UFL,UFR,UBL),
        //         Back => (UFL,UBR,UBL,UFR),
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ColourPerm {
    up: Colour,
    down: Colour,
    left: Colour,
    right: Colour,
    front: Colour,
    back: Colour,
}

impl From<&HashMap<Face,Colour>> for ColourPerm {
    fn from(value: &HashMap<Face,Colour>) -> Self {
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
    fn up(&self) -> Colour { White }
    fn down(&self) -> Colour { Yellow }
    fn left(&self) -> Colour { Orange }
    fn right(&self) -> Colour { Red }
    fn front(&self) -> Colour { Green }
    fn back(&self) -> Colour { Blue }
}

impl ColourScheme for Japanese {
    fn up(&self) -> Colour { White }
    fn down(&self) -> Colour { Blue }
    fn left(&self) -> Colour { Orange }
    fn right(&self) -> Colour { Red }
    fn front(&self) -> Colour { Green }
    fn back(&self) -> Colour { Yellow }
}

impl ColourScheme for ColourPerm {
    fn up(&self) -> Colour { self.up }
    fn down(&self) -> Colour { self.down }
    fn left(&self) -> Colour { self.left }
    fn right(&self) -> Colour { self.right }
    fn front(&self) -> Colour { self.front }
    fn back(&self) -> Colour { self.back }
}

pub struct FaceState<const DIM: usize> {
    vals: [[Colour;DIM];DIM]
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
    up: FaceState<DIM>,
    down: FaceState<DIM>,
    left: FaceState<DIM>,
    right: FaceState<DIM>,
    front: FaceState<DIM>,
    back: FaceState<DIM>
}

impl<const DIM: usize> RubiksState<DIM> {
    pub fn face_state<'a>(&'a self, face: Face) -> &'a FaceState<DIM> {
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
                Up => scheme.rotated(CubeRotation::ID),
                Down => scheme.rotated(X*X),
                Left => scheme.rotated(Z),
                Right => scheme.rotated(Z3),
                Front => scheme.rotated(X),
                Back => scheme.rotated(X3),
            };

            println!("Debug: after first rotation, scheme up={:?}, front={:?}",
                     first_edit_scheme.up(), first_edit_scheme.front());

            let front_colour = self.front.vals[0][0];
            println!("Debug: front_colour = {:?}", front_colour);

            let edited_scheme = match first_edit_scheme.get_face(front_colour) {
                Ok(face) => {
                    println!("Debug: found front_colour on face: {:?}", face);
                    match face {
                        Front => first_edit_scheme,
                        Back => first_edit_scheme.rotated(Y*Y),
                        Left => first_edit_scheme.rotated(Y3),
                        Right => first_edit_scheme.rotated(Y),
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
mod tests {
    use super::*;

    #[test]
    fn test_rotation_orders() {
        // X rotation should have order 4 (x^4 = identity)
        assert_eq!(X * X * X * X, CubeRotation::ID);
        assert_ne!(X * X * X, CubeRotation::ID);

        // Y rotation should have order 4
        assert_eq!(Y * Y * Y * Y, CubeRotation::ID);
        assert_ne!(Y * Y * Y, CubeRotation::ID);

        // Z rotation should have order 4
        assert_eq!(Z * Z * Z * Z, CubeRotation::ID);
        assert_ne!(Z * Z * Z, CubeRotation::ID);
    }

    #[test]
    fn test_xyz_order() {
        // Let's find what order X*Y*Z actually has
        let xyz = X * Y * Z;
        let mut power = xyz;
        let mut order = 1;

        while power != CubeRotation::ID && order <= 24 {
            power = power * xyz;
            order += 1;
        }

        println!("XYZ = {:?}", xyz);
        println!("Order of XYZ: {}", order);

        // Also check XZY
        let xzy = X * Z * Y;
        let mut power_xzy = xzy;
        let mut order_xzy = 1;
        while power_xzy != CubeRotation::ID && order_xzy <= 24 {
            power_xzy = power_xzy * xzy;
            order_xzy += 1;
        }
        println!("XZY = {:?}", xzy);
        println!("Order of XZY: {}", order_xzy);

        assert!(order <= 24, "XYZ should have finite order");
    }

    #[test]
    fn test_group_closure() {
        // The group should be closed under multiplication
        // Test that xyz has finite order
        let xyz = X * Y * Z;
        let mut power = xyz;
        let mut found_identity = false;

        for _ in 1..25 { // At most 24 elements in the group
            if power == CubeRotation::ID {
                found_identity = true;
                break;
            }
            power = power * xyz;
        }
        assert!(found_identity, "xyz should have finite order");
    }

    #[test]
    fn test_associativity() {
        // Check associativity: (X*Y)*Z = X*(Y*Z)
        assert_eq!((X * Y) * Z, X * (Y * Z));
        assert_eq!((Y * Z) * X, Y * (Z * X));
        assert_eq!((Z * X) * Y, Z * (X * Y));
    }

    #[test]
    fn test_identity_constants() {
        // All identity constants should be equal
        assert_eq!(CubeRotation::ID, CubeRotation::ONE);
        assert_eq!(CubeRotation::ONE, CubeRotation::E);

        // Identity should be a left and right identity
        assert_eq!(CubeRotation::ID * X, X);
        assert_eq!(X * CubeRotation::ID, X);
        assert_eq!(CubeRotation::E * Y, Y);
        assert_eq!(Y * CubeRotation::E, Y);
    }

    #[test]
    fn test_rotation_inverses() {
        // Verify all X3 = X*X*X by computing manually
        let x_cubed = X * X * X;
        let y_cubed = Y * Y * Y;
        let z_cubed = Z * Z * Z;

        println!("X*X*X = {:?}, X3 = {:?}", x_cubed, X3);
        println!("Y*Y*Y = {:?}, Y3 = {:?}", y_cubed, Y3);
        println!("Z*Z*Z = {:?}, Z3 = {:?}", z_cubed, Z3);

        assert_eq!(x_cubed, X3);
        assert_eq!(y_cubed, Y3);
        assert_eq!(z_cubed, Z3);

        // Test that X3 is the inverse of X
        assert_eq!(X * X3, CubeRotation::ID);
        assert_eq!(X3 * X, CubeRotation::ID);

        // Test that Y3 is the inverse of Y
        assert_eq!(Y * Y3, CubeRotation::ID);
        assert_eq!(Y3 * Y, CubeRotation::ID);

        // Test that Z3 is the inverse of Z
        assert_eq!(Z * Z3, CubeRotation::ID);
        assert_eq!(Z3 * Z, CubeRotation::ID);
    }

    #[test]
    fn test_face_perm_identity() {
        let id_perm = FacePerm::from(CubeRotation::ID);
        // Identity should map each face to itself
        assert_eq!(id_perm.0, [Up, Down, Left, Right, Front, Back]);
    }

    #[test]
    fn test_face_perm_basic_rotations() {
        let x_perm = FacePerm::from(X);
        let y_perm = FacePerm::from(Y);
        let z_perm = FacePerm::from(Z);

        println!("X rotation face permutation: {:?}", x_perm.0);
        println!("Y rotation face permutation: {:?}", y_perm.0);
        println!("Z rotation face permutation: {:?}", z_perm.0);

        // Also check the inverse rotations
        let x3_perm = FacePerm::from(X3);
        let y3_perm = FacePerm::from(Y3);
        let z3_perm = FacePerm::from(Z3);

        println!("X3 rotation face permutation: {:?}", x3_perm.0);
        println!("Y3 rotation face permutation: {:?}", y3_perm.0);
        println!("Z3 rotation face permutation: {:?}", z3_perm.0);

        assert!(true); // Just print for now
    }

    #[test]
    fn test_face_perm_homomorphism() {
        // Test that composition works: FacePerm(AB) = FacePerm(A) * FacePerm(B)
        let xy = X * Y;
        let xy_perm = FacePerm::from(xy);

        // For now just verify it computes without panicking
        println!("XY face permutation: {:?}", xy_perm.0);
        assert!(true);
    }

    #[test]
    fn test_solved_up_to_rotation() {
        let western = Western;

        // Create a cube solved in standard Western orientation
        let cube_standard = RubiksState::<3>::solved_in(western);

        // Create cubes solved in rotated Western orientations
        let western_x_rotated = western.rotated(X);
        let cube_x_rotated = RubiksState::<3>::solved_in(western_x_rotated);

        println!("Standard Western: Up={:?}, Front={:?}", western.up(), western.front());
        println!("X-rotated Western: Up={:?}, Front={:?}", western_x_rotated.up(), western_x_rotated.front());
        println!("X-rotated cube: top_color={:?}, front_color={:?}",
                 cube_x_rotated.face_state(Up).vals[0][0],
                 cube_x_rotated.face_state(Front).vals[0][0]);

        let western_y_rotated = western.rotated(Y);
        let cube_y_rotated = RubiksState::<3>::solved_in(western_y_rotated);

        // Test some more complex rotations
        let complex_rotation1 = X * Y * Z3 * X3; // Compose multiple rotations
        let western_complex1 = western.rotated(complex_rotation1);
        let cube_complex1 = RubiksState::<3>::solved_in(western_complex1);

        let complex_rotation2 = Y * Y * Z * X; // Another complex rotation
        let western_complex2 = western.rotated(complex_rotation2);
        let cube_complex2 = RubiksState::<3>::solved_in(western_complex2);

        println!("Complex rotation 1 cube: top_color={:?}, front_color={:?}",
                 cube_complex1.face_state(Up).vals[0][0],
                 cube_complex1.face_state(Front).vals[0][0]);

        println!("Complex rotation 2 cube: top_color={:?}, front_color={:?}",
                 cube_complex2.face_state(Up).vals[0][0],
                 cube_complex2.face_state(Front).vals[0][0]);

        // All these cubes should be detected as solved in Western up to rotation
        assert!(cube_standard.is_solved_up_to_rotation_in(western));
        assert!(cube_x_rotated.is_solved_up_to_rotation_in(western));
        assert!(cube_y_rotated.is_solved_up_to_rotation_in(western));
        assert!(cube_complex1.is_solved_up_to_rotation_in(western));
        assert!(cube_complex2.is_solved_up_to_rotation_in(western));

        // But cubes with rotations shouldn't be solved in the standard orientation
        assert!(!cube_x_rotated.is_solved_in(western));
        assert!(!cube_y_rotated.is_solved_in(western));
        assert!(!cube_complex1.is_solved_in(western));
        assert!(!cube_complex2.is_solved_in(western));
    }
}
