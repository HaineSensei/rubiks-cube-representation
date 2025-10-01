use super::*;
use crate::core::cube::schemes::{Western, ColourPerm};
use crate::core::cube::rotations::{X, X3, Y, Y3, Z, Z3, CubeRotation};
use crate::core::cube::geometry::Face;
use crate::core::rubiks::moves::{BasicMove, SliceMove, RangeMove, WideMove, MiddleMove};
use crate::core::rubiks::tiles::{TilePerm, restrictions::Slice};
use crate::core::Colour;

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

#[test]
fn test_1x1_cube_rotations() {
    // Create a simple 1x1 cube with distinct colors on each face
    let original_scheme = ColourPerm {
        up: Colour::White,
        down: Colour::Yellow,
        left: Colour::Orange,
        right: Colour::Red,
        front: Colour::Green,
        back: Colour::Blue,
    };

    let cube = RubiksState::<1>::solved_in(original_scheme);

    // Test X rotation: Up→Back, Down→Front, Front→Up, Back→Down, Left→Left, Right→Right
    let rotated_cube = &cube * &X;

    let expected_after_x = ColourPerm {
        up: Colour::Green,    // Front went to Up
        down: Colour::Blue,   // Back went to Down
        left: Colour::Orange, // Left stayed Left
        right: Colour::Red,   // Right stayed Right
        front: Colour::Yellow, // Down went to Front
        back: Colour::White,  // Up went to Back
    };

    let expected_cube = RubiksState::<1>::solved_in(expected_after_x);
    assert_eq!(rotated_cube, expected_cube, "X rotation should move faces correctly");

    // Test Y rotation on original: Front→Left, Left→Back, Back→Right, Right→Front, Up→Up, Down→Down
    let y_rotated_cube = &cube * &Y;

    let expected_after_y = ColourPerm {
        up: Colour::White,    // Up stayed Up
        down: Colour::Yellow, // Down stayed Down
        left: Colour::Green,  // Front went to Left
        right: Colour::Blue,  // Back went to Right
        front: Colour::Red,   // Right went to Front
        back: Colour::Orange, // Left went to Back
    };

    let expected_y_cube = RubiksState::<1>::solved_in(expected_after_y);
    assert_eq!(y_rotated_cube, expected_y_cube, "Y rotation should move faces correctly");

    // Test Z rotation on original: Up→Right, Right→Down, Down→Left, Left→Up, Front→Front, Back→Back
    let z_rotated_cube = &cube * &Z;

    let expected_after_z = ColourPerm {
        up: Colour::Orange,   // Left went to Up
        down: Colour::Red,    // Right went to Down
        left: Colour::Yellow, // Down went to Left
        right: Colour::White, // Up went to Right
        front: Colour::Green, // Front stayed Front
        back: Colour::Blue,   // Back stayed Back
    };

    let expected_z_cube = RubiksState::<1>::solved_in(expected_after_z);
    assert_eq!(z_rotated_cube, expected_z_cube, "Z rotation should move faces correctly");
}

#[test]
fn test_2x2_cube_x_rotation() {
    use Colour::*;
    let initial = RubiksState {
        up: FaceState { vals: [[Orange, Red], [Yellow, White]] },
        down: FaceState { vals: [[Blue, Green], [Red, Orange]] },
        left: FaceState { vals: [[White, Yellow], [Blue, Green]] },
        right: FaceState { vals: [[Green, Blue], [Orange, Red]] },
        front: FaceState { vals: [[Yellow, White], [Green, Blue]] },
        back: FaceState { vals: [[Red, Orange], [White, Yellow]] },
    };
    
    let expected_after_x = RubiksState {
        up: FaceState { vals: [[Yellow, White], [Green, Blue]] },
        down: FaceState { vals: [[Red, Orange], [White, Yellow]] },
        left: FaceState { vals: [[Yellow, Green], [White, Blue]] },
        right: FaceState { vals: [[Orange, Green], [Red, Blue]] },
        front: FaceState { vals: [[Blue, Green], [Red, Orange]] },
        back: FaceState { vals: [[Orange, Red], [Yellow, White]] },
    };

    assert_eq!(&initial * &X, expected_after_x);
}

#[test]
fn test_2x2_cube_y_rotation() {
    use Colour::*;
    let initial = RubiksState {
        up: FaceState { vals: [[Orange, Red], [Yellow, White]] },
        down: FaceState { vals: [[Blue, Green], [Red, Orange]] },
        left: FaceState { vals: [[White, Yellow], [Blue, Green]] },
        right: FaceState { vals: [[Green, Blue], [Orange, Red]] },
        front: FaceState { vals: [[Yellow, White], [Green, Blue]] },
        back: FaceState { vals: [[Red, Orange], [White, Yellow]] },
    };
    
    let expected_after_y = RubiksState {
        up: FaceState { vals: [[Yellow, Orange], [White, Red]] },
        down: FaceState { vals: [[Green, Orange], [Blue, Red]] },
        left: FaceState { vals: [[Yellow, White], [Green, Blue]] },
        right: FaceState { vals: [[Yellow, White], [Orange, Red]] },
        front: FaceState { vals: [[Green, Blue], [Orange, Red]] },
        back: FaceState { vals: [[Green, Blue], [Yellow, White]] },
    };

    assert_eq!(&initial * &Y, expected_after_y);
}

#[test]
fn test_2x2_cube_z_rotation() {
    use Colour::*;
    let initial = RubiksState {
        up: FaceState { vals: [[Orange, Red], [Yellow, White]] },
        down: FaceState { vals: [[Blue, Green], [Red, Orange]] },
        left: FaceState { vals: [[White, Yellow], [Blue, Green]] },
        right: FaceState { vals: [[Green, Blue], [Orange, Red]] },
        front: FaceState { vals: [[Yellow, White], [Green, Blue]] },
        back: FaceState { vals: [[Red, Orange], [White, Yellow]] },
    };
    
    let expected_after_z = RubiksState {
        up: FaceState { vals: [[Blue, White], [Green, Yellow]] },
        down: FaceState { vals: [[Orange, Green], [Red, Blue]] },
        left: FaceState { vals: [[Red, Blue], [Orange, Green]] },
        right: FaceState { vals: [[Yellow, Orange], [White, Red]] },
        front: FaceState { vals: [[Green, Yellow], [Blue, White]] },
        back: FaceState { vals: [[Orange, Yellow], [Red, White]] },
    };

    assert_eq!(&initial * &Z, expected_after_z);
}

#[test]
fn test_basic_moves_agree_with_rotations_on_slices() {
    // Each move should agree with the corresponding rotation on its slice,
    // and agree with identity on all other slices

    let identity = TilePerm::<3>::from(&CubeRotation::ID);

    let test_cases = [
        (BasicMove::<3>::U, Y, Face::Up),     // U rotates like Y around Up face
        (BasicMove::<3>::D, Y3, Face::Down),  // D rotates like Y' around Down face
        (BasicMove::<3>::L, X3, Face::Left),  // L rotates like X' around Left face
        (BasicMove::<3>::R, X, Face::Right),  // R rotates like X around Right face
        (BasicMove::<3>::F, Z, Face::Front),  // F rotates like Z around Front face
        (BasicMove::<3>::B, Z3, Face::Back),  // B rotates like Z' around Back face
    ];

    for (mov, rotation, face) in test_cases {
        let move_perm = TilePerm::<3>::from(&mov);
        let rotation_perm = TilePerm::<3>::from(&rotation);

        let move_slice = Slice { face, slice_index: 0 };

        // The move should agree with the rotation on its slice
        assert!(move_perm.agree_on(&rotation_perm, move_slice),
                "{:?} should agree with {:?} on {:?} slice 0", mov, rotation, face);

        // The move should agree with identity on other slices of the same face
        for slice_idx in 1..3 {
            let other_slice = Slice { face, slice_index: slice_idx };
            assert!(move_perm.agree_on(&identity, other_slice),
                    "{:?} should agree with identity on {:?} slice {}", mov, face, slice_idx);
        }
    }
}

#[test]
fn test_slice_moves_agree_with_rotations_on_slices() {
    // SliceMove should agree with rotation on the specific slice it affects
    let identity = TilePerm::<3>::from(&CubeRotation::ID);

    let test_cases = [
        (SliceMove::<3>::Us(2), Y, Face::Up, 1),      // Layer 2 = slice index 1
        (SliceMove::<3>::Ds(2), Y3, Face::Down, 1),
        (SliceMove::<3>::Ls(2), X3, Face::Left, 1),
        (SliceMove::<3>::Rs(2), X, Face::Right, 1),
        (SliceMove::<3>::Fs(2), Z, Face::Front, 1),
        (SliceMove::<3>::Bs(2), Z3, Face::Back, 1),
    ];

    for (mov, rotation, face, slice_index) in test_cases {
        let move_perm = TilePerm::<3>::from(&mov);
        let rotation_perm = TilePerm::<3>::from(&rotation);

        let target_slice = Slice { face, slice_index };

        // Should agree with rotation on the target slice
        assert!(move_perm.agree_on(&rotation_perm, target_slice),
                "{:?} should agree with {:?} on {:?} slice {}", mov, rotation, face, slice_index);

        // Should agree with identity on all other slices of that face
        for other_idx in 0..3 {
            if other_idx != slice_index {
                let other_slice = Slice { face, slice_index: other_idx };
                assert!(move_perm.agree_on(&identity, other_slice),
                        "{:?} should agree with identity on {:?} slice {}", mov, face, other_idx);
            }
        }
    }
}

#[test]
fn test_wide_moves_agree_with_rotations_on_slices() {
    // WideMove should agree with rotation on slices 0..depth
    let identity = TilePerm::<3>::from(&CubeRotation::ID);

    let test_cases = [
        (WideMove::<3>::Uw(2), Y, Face::Up),
        (WideMove::<3>::Dw(2), Y3, Face::Down),
        (WideMove::<3>::Lw(2), X3, Face::Left),
        (WideMove::<3>::Rw(2), X, Face::Right),
        (WideMove::<3>::Fw(2), Z, Face::Front),
        (WideMove::<3>::Bw(2), Z3, Face::Back),
    ];

    for (mov, rotation, face) in test_cases {
        let move_perm = TilePerm::<3>::from(&mov);
        let rotation_perm = TilePerm::<3>::from(&rotation);

        // Should agree with rotation on slices 0 and 1 (depth=2)
        for slice_idx in 0..2 {
            let affected_slice = Slice { face, slice_index: slice_idx };
            assert!(move_perm.agree_on(&rotation_perm, affected_slice),
                    "{:?} should agree with {:?} on {:?} slice {}", mov, rotation, face, slice_idx);
        }

        // Should agree with identity on slice 2
        let unaffected_slice = Slice { face, slice_index: 2 };
        assert!(move_perm.agree_on(&identity, unaffected_slice),
                "{:?} should agree with identity on {:?} slice 2", mov, face);
    }
}

#[test]
fn test_range_moves_agree_with_rotations_on_slices() {
    // RangeMove should agree with rotation on slices in the specified range
    let identity = TilePerm::<3>::from(&CubeRotation::ID);

    let test_cases = [
        (RangeMove::<3>::Ur(2, 2), Y, Face::Up, 1, 1),      // Layer 2-2 = slice index 1-1
        (RangeMove::<3>::Dr(1, 2), Y3, Face::Down, 0, 1),   // Layer 1-2 = slice index 0-1
        (RangeMove::<3>::Lr(2, 3), X3, Face::Left, 1, 2),   // Layer 2-3 = slice index 1-2
    ];

    for (mov, rotation, face, start_slice, end_slice) in test_cases {
        let move_perm = TilePerm::<3>::from(&mov);
        let rotation_perm = TilePerm::<3>::from(&rotation);

        // Should agree with rotation on slices in the range
        for slice_idx in start_slice..=end_slice {
            let affected_slice = Slice { face, slice_index: slice_idx };
            assert!(move_perm.agree_on(&rotation_perm, affected_slice),
                    "{:?} should agree with {:?} on {:?} slice {}", mov, rotation, face, slice_idx);
        }

        // Should agree with identity on slices outside the range
        for slice_idx in 0..3 {
            if slice_idx < start_slice || slice_idx > end_slice {
                let unaffected_slice = Slice { face, slice_index: slice_idx };
                assert!(move_perm.agree_on(&identity, unaffected_slice),
                        "{:?} should agree with identity on {:?} slice {}", mov, face, slice_idx);
            }
        }
    }
}

#[test]
fn test_middle_moves_agree_with_rotations_on_slices() {
    // MiddleMove should agree with rotation on the middle slice (N/2)
    // Test with 3x3 (middle = slice 1) and 5x5 (middle = slice 2)

    let identity_3x3 = TilePerm::<3>::from(&CubeRotation::ID);
    let identity_5x5 = TilePerm::<5>::from(&CubeRotation::ID);

    // 3x3 tests (middle slice at index 1)
    let test_cases_3x3 = [
        (MiddleMove::<3>::M, X3, Face::Left, 1),
        (MiddleMove::<3>::E, Y3, Face::Down, 1),
        (MiddleMove::<3>::S, Z, Face::Front, 1),
    ];

    for (mov, rotation, face, middle_idx) in test_cases_3x3 {
        let move_perm = TilePerm::<3>::from(&mov);
        let rotation_perm = TilePerm::<3>::from(&rotation);

        let middle_slice = Slice { face, slice_index: middle_idx };

        // Should agree with rotation on the middle slice
        assert!(move_perm.agree_on(&rotation_perm, middle_slice),
                "{:?} should agree with {:?} on {:?} slice {} (3x3)", mov, rotation, face, middle_idx);

        // Should agree with identity on other slices
        for slice_idx in 0..3 {
            if slice_idx != middle_idx {
                let other_slice = Slice { face, slice_index: slice_idx };
                assert!(move_perm.agree_on(&identity_3x3, other_slice),
                        "{:?} should agree with identity on {:?} slice {} (3x3)", mov, face, slice_idx);
            }
        }
    }

    // 5x5 tests (middle slice at index 2)
    let test_cases_5x5 = [
        (MiddleMove::<5>::M, X3, Face::Left, 2),
        (MiddleMove::<5>::E, Y3, Face::Down, 2),
        (MiddleMove::<5>::S, Z, Face::Front, 2),
    ];

    for (mov, rotation, face, middle_idx) in test_cases_5x5 {
        let move_perm = TilePerm::<5>::from(&mov);
        let rotation_perm = TilePerm::<5>::from(&rotation);

        let middle_slice = Slice { face, slice_index: middle_idx };

        // Should agree with rotation on the middle slice
        assert!(move_perm.agree_on(&rotation_perm, middle_slice),
                "{:?} should agree with {:?} on {:?} slice {} (5x5)", mov, rotation, face, middle_idx);

        // Should agree with identity on other slices
        for slice_idx in 0..5 {
            if slice_idx != middle_idx {
                let other_slice = Slice { face, slice_index: slice_idx };
                assert!(move_perm.agree_on(&identity_5x5, other_slice),
                        "{:?} should agree with identity on {:?} slice {} (5x5)", mov, face, slice_idx);
            }
        }
    }
}