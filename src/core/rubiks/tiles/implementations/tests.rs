use crate::core::cube::rotations::{CubeRotation, X, X2, X3, Y, Z};
use crate::core::rubiks::tiles::{TilePos, TilePerm};
use crate::Face;

#[test]
fn test_identity_rotation_3x3() {
    let perm = TilePerm::<3>::from(&CubeRotation::ID);

    // Identity should map every tile to itself
    for face in [Face::Up, Face::Down, Face::Left, Face::Right, Face::Front, Face::Back] {
        for row in 0..3 {
            for col in 0..3 {
                let original_pos = TilePos { face, row, col };
                let mapped_pos = perm[original_pos];
                assert_eq!(original_pos, mapped_pos,
                    "Identity rotation should map {:?} to itself", original_pos);
            }
        }
    }
}

#[test]
fn test_identity_rotation_4x4() {
    let perm = TilePerm::<4>::from(&CubeRotation::ID);

    // Identity should map every tile to itself
    for face in [Face::Up, Face::Down, Face::Left, Face::Right, Face::Front, Face::Back] {
        for row in 0..4 {
            for col in 0..4 {
                let original_pos = TilePos { face, row, col };
                let mapped_pos = perm[original_pos];
                assert_eq!(original_pos, mapped_pos,
                    "Identity rotation should map {:?} to itself", original_pos);
            }
        }
    }
}

#[test]
fn test_x_rotation_face_mapping_3x3() {
    let perm = TilePerm::<3>::from(&X);

    // After X rotation: Up->Back, Down->Front, Front->Up, Back->Down, Left->Left, Right->Right
    // Check a few representative tiles from each face

    // Up face tiles should map to Back face
    let up_center = TilePos { face: Face::Up, row: 1, col: 1 };
    assert_eq!(perm[up_center].face, Face::Back, "Up face should map to Back after X rotation");

    // Down face tiles should map to Front face
    let down_center = TilePos { face: Face::Down, row: 1, col: 1 };
    assert_eq!(perm[down_center].face, Face::Front, "Down face should map to Front after X rotation");

    // Front face tiles should map to Up face
    let front_center = TilePos { face: Face::Front, row: 1, col: 1 };
    assert_eq!(perm[front_center].face, Face::Up, "Front face should map to Up after X rotation");

    // Back face tiles should map to Down face
    let back_center = TilePos { face: Face::Back, row: 1, col: 1 };
    assert_eq!(perm[back_center].face, Face::Down, "Back face should map to Down after X rotation");

    // Left and Right faces should stay on their respective faces
    let left_center = TilePos { face: Face::Left, row: 1, col: 1 };
    assert_eq!(perm[left_center].face, Face::Left, "Left face should stay on Left after X rotation");

    let right_center = TilePos { face: Face::Right, row: 1, col: 1 };
    assert_eq!(perm[right_center].face, Face::Right, "Right face should stay on Right after X rotation");
}

#[test]
fn test_y_rotation_face_mapping_3x3() {
    let perm = TilePerm::<3>::from(&Y);

    // After Y rotation: Front->Left, Left->Back, Back->Right, Right->Front, Up->Up, Down->Down

    let front_center = TilePos { face: Face::Front, row: 1, col: 1 };
    assert_eq!(perm[front_center].face, Face::Left, "Front face should map to Left after Y rotation");

    let left_center = TilePos { face: Face::Left, row: 1, col: 1 };
    assert_eq!(perm[left_center].face, Face::Back, "Left face should map to Back after Y rotation");

    let back_center = TilePos { face: Face::Back, row: 1, col: 1 };
    assert_eq!(perm[back_center].face, Face::Right, "Back face should map to Right after Y rotation");

    let right_center = TilePos { face: Face::Right, row: 1, col: 1 };
    assert_eq!(perm[right_center].face, Face::Front, "Right face should map to Front after Y rotation");

    // Up and Down should stay on their respective faces
    let up_center = TilePos { face: Face::Up, row: 1, col: 1 };
    assert_eq!(perm[up_center].face, Face::Up, "Up face should stay on Up after Y rotation");

    let down_center = TilePos { face: Face::Down, row: 1, col: 1 };
    assert_eq!(perm[down_center].face, Face::Down, "Down face should stay on Down after Y rotation");
}

#[test]
fn test_z_rotation_face_mapping_3x3() {
    let perm = TilePerm::<3>::from(&Z);

    // After Z rotation: Up->Right, Right->Down, Down->Left, Left->Up, Front->Front, Back->Back

    let up_center = TilePos { face: Face::Up, row: 1, col: 1 };
    assert_eq!(perm[up_center].face, Face::Right, "Up face should map to Right after Z rotation");

    let right_center = TilePos { face: Face::Right, row: 1, col: 1 };
    assert_eq!(perm[right_center].face, Face::Down, "Right face should map to Down after Z rotation");

    let down_center = TilePos { face: Face::Down, row: 1, col: 1 };
    assert_eq!(perm[down_center].face, Face::Left, "Down face should map to Left after Z rotation");

    let left_center = TilePos { face: Face::Left, row: 1, col: 1 };
    assert_eq!(perm[left_center].face, Face::Up, "Left face should map to Up after Z rotation");

    // Front and Back should stay on their respective faces
    let front_center = TilePos { face: Face::Front, row: 1, col: 1 };
    assert_eq!(perm[front_center].face, Face::Front, "Front face should stay on Front after Z rotation");

    let back_center = TilePos { face: Face::Back, row: 1, col: 1 };
    assert_eq!(perm[back_center].face, Face::Back, "Back face should stay on Back after Z rotation");
}

#[test]
fn test_rotation_composition_3x3() {
    // Test that applying Y then X is equivalent to applying Y*X
    let x_perm = TilePerm::<3>::from(&X);
    let y_perm = TilePerm::<3>::from(&Y);
    let yx_perm = TilePerm::<3>::from(&(Y * X));

    // Composing Y then X should equal Y*X
    let composed_perm = y_perm * x_perm;

    // Test a few representative positions
    for face in [Face::Up, Face::Front, Face::Right] {
        let pos = TilePos { face, row: 0, col: 0 };
        assert_eq!(composed_perm[pos], yx_perm[pos],
            "Composition Y*X should equal applying Y then X for position {:?}", pos);

        let pos = TilePos { face, row: 1, col: 1 };
        assert_eq!(composed_perm[pos], yx_perm[pos],
            "Composition Y*X should equal applying Y then X for position {:?}", pos);

        let pos = TilePos { face, row: 2, col: 2 };
        assert_eq!(composed_perm[pos], yx_perm[pos],
            "Composition Y*X should equal applying Y then X for position {:?}", pos);
    }
}

#[test]
fn test_rotation_inverse_3x3() {
    // Test that applying a rotation then its inverse gives identity
    let x_perm = TilePerm::<3>::from(&X);
    let x3_perm = TilePerm::<3>::from(&X3); // X^-1 = X^3

    let composed = x3_perm * x_perm;
    let identity = TilePerm::<3>::from(&CubeRotation::ID);

    // Test all corners and centers
    for face in [Face::Up, Face::Down, Face::Left, Face::Right, Face::Front, Face::Back] {
        // Corners
        for &(row, col) in &[(0, 0), (0, 2), (2, 0), (2, 2)] {
            let pos = TilePos { face, row, col };
            assert_eq!(composed[pos], identity[pos],
                "X * X^3 should equal identity for position {:?}", pos);
        }

        // Center
        let center = TilePos { face, row: 1, col: 1 };
        assert_eq!(composed[center], identity[center],
            "X * X^3 should equal identity for center {:?}", center);
    }
}

#[test]
fn test_x2_rotation_3x3() {
    // X2 should be equivalent to X * X
    let x_perm = TilePerm::<3>::from(&X);
    let x2_perm = TilePerm::<3>::from(&X2);
    let x_twice = &x_perm * &x_perm;

    for face in [Face::Up, Face::Down, Face::Left, Face::Right, Face::Front, Face::Back] {
        for row in 0..3 {
            for col in 0..3 {
                let pos = TilePos { face, row, col };
                assert_eq!(x2_perm[pos], x_twice[pos],
                    "X2 should equal X*X for position {:?}", pos);
            }
        }
    }
}

#[test]
fn test_different_cube_dimensions() {
    // Test that the same rotation works for different cube sizes
    let x_3x3 = TilePerm::<3>::from(&X);
    let x_4x4 = TilePerm::<4>::from(&X);

    // Check that face mappings are consistent across dimensions
    let pos_3x3 = TilePos { face: Face::Up, row: 1, col: 1 };
    let pos_4x4 = TilePos { face: Face::Up, row: 1, col: 1 };

    assert_eq!(x_3x3[pos_3x3].face, x_4x4[pos_4x4].face,
        "X rotation should map faces consistently across cube dimensions");

    // Check that different positions within the same face map to the same destination face
    let corner_3x3 = TilePos { face: Face::Front, row: 0, col: 0 };
    let corner_4x4 = TilePos { face: Face::Front, row: 0, col: 0 };

    assert_eq!(x_3x3[corner_3x3].face, x_4x4[corner_4x4].face,
        "X rotation should map Front face consistently across cube dimensions");
}