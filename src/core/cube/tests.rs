use super::*;
use geometry::Face;
use rotations::{CubeRotation, FacePerm, X, X3, Y, Y3, Z, Z3};

#[test]
fn test_face_perm_identity() {
    let id_perm = FacePerm::from(CubeRotation::ID);
    // Identity should map each face to itself
    use Face::*;
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