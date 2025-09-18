use super::*;
use crate::core::cube::schemes::Western;
use crate::core::cube::rotations::{X, X3, Y, Y3, Z, Z3};
use crate::core::cube::geometry::Face;

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
             cube_x_rotated.face_state(Face::Up).vals[0][0],
             cube_x_rotated.face_state(Face::Front).vals[0][0]);

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
             cube_complex1.face_state(Face::Up).vals[0][0],
             cube_complex1.face_state(Face::Front).vals[0][0]);

    println!("Complex rotation 2 cube: top_color={:?}, front_color={:?}",
             cube_complex2.face_state(Face::Up).vals[0][0],
             cube_complex2.face_state(Face::Front).vals[0][0]);

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