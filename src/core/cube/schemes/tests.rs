use super::*;
use crate::core::cube::rotations::{CubeRotation, X, Y, Z};
use crate::core::Colour;

#[test]
fn test_colour_scheme_rotated() {
    use Colour::*;

    // Create a test scheme
    let original_scheme = ColourPerm {
        up: White,
        down: Yellow,
        left: Blue,
        right: Green,
        front: Red,
        back: Orange,
    };

    // Test X rotation: Up->Back->Down->Front->Up cycle, Left/Right unchanged
    let x_rotated = original_scheme.rotated(X);
    assert_eq!(x_rotated.up(), Red);        // original front -> up
    assert_eq!(x_rotated.down(), Orange);   // original back -> down
    assert_eq!(x_rotated.left(), Blue);     // unchanged
    assert_eq!(x_rotated.right(), Green);   // unchanged
    assert_eq!(x_rotated.front(), Yellow);  // original down -> front
    assert_eq!(x_rotated.back(), White);    // original up -> back

    // Test Y rotation: Front->Left->Back->Right->Front cycle, Up/Down unchanged
    let y_rotated = original_scheme.rotated(Y);
    assert_eq!(y_rotated.up(), White);      // unchanged
    assert_eq!(y_rotated.down(), Yellow);   // unchanged
    assert_eq!(y_rotated.left(), Red);      // original front -> left
    assert_eq!(y_rotated.right(), Orange);  // original back -> right
    assert_eq!(y_rotated.front(), Green);   // original right -> front
    assert_eq!(y_rotated.back(), Blue);     // original left -> back

    // Test Z rotation: Up->Right->Down->Left->Up cycle, Front/Back unchanged
    let z_rotated = original_scheme.rotated(Z);
    assert_eq!(z_rotated.up(), Blue);       // original left -> up
    assert_eq!(z_rotated.down(), Green);    // original right -> down
    assert_eq!(z_rotated.left(), Yellow);   // original down -> left
    assert_eq!(z_rotated.right(), White);   // original up -> right
    assert_eq!(z_rotated.front(), Red);     // unchanged
    assert_eq!(z_rotated.back(), Orange);   // unchanged

    // Test that identity rotation doesn't change the scheme
    let id_rotated = original_scheme.rotated(CubeRotation::ID);
    assert_eq!(id_rotated.up(), original_scheme.up());
    assert_eq!(id_rotated.down(), original_scheme.down());
    assert_eq!(id_rotated.left(), original_scheme.left());
    assert_eq!(id_rotated.right(), original_scheme.right());
    assert_eq!(id_rotated.front(), original_scheme.front());
    assert_eq!(id_rotated.back(), original_scheme.back());
}