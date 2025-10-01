use super::*;
use crate::core::rubiks::moves::{BasicMove};
use crate::core::rubiks::tiles::restrictions::{Restriction, Slice};
use crate::core::cube::rotations::CubeRotation;
use crate::Face;

#[test]
fn test_u_move_affects_only_top_slice() {
    let u_move = TilePerm::<3>::from(&BasicMove::<3>::U);
    let identity = TilePerm::<3>::from(&CubeRotation::ID);
    let top_slice = Slice { face: Face::Up, slice_index: 0 };
    let second_slice = Slice { face: Face::Up, slice_index: 1 };

    // Check if U move affects the top slice (it should)
    let affects_top = !u_move.agree_on(&identity, top_slice);
    println!("U move affects top slice (slice_index=0): {}", affects_top);
    assert!(affects_top, "U move should affect the top slice");

    // Check if U move affects the second slice (it shouldn't)
    let affects_second = !u_move.agree_on(&identity, second_slice);
    println!("U move affects second slice (slice_index=1): {}", affects_second);

    if affects_second {
        // Debug: find which positions disagree
        println!("\nPositions where U move disagrees with identity on second slice:");
        for pos in <Slice as Restriction<3>>::restricted_positions(&second_slice) {
            let u_dest = u_move[pos];
            let id_dest = identity[pos];
            if u_dest != id_dest {
                println!("  Position {:?} -> U: {:?}, Identity: {:?}", pos, u_dest, id_dest);
            }
        }
    }

    assert!(!affects_second, "U move should NOT affect the second slice");
}

#[test]
fn test_basic_moves_have_order_4() {
    // All basic quarter-turn moves should have order 4 (M^4 = identity)
    let identity = TilePerm::<3>::from(&CubeRotation::ID);

    let moves = [
        BasicMove::<3>::U, BasicMove::<3>::D,
        BasicMove::<3>::L, BasicMove::<3>::R,
        BasicMove::<3>::F, BasicMove::<3>::B,
    ];

    for mov in moves {
        let m = TilePerm::<3>::from(&mov);
        let m4 = &(&(&m * &m) * &m) * &m;
        assert_eq!(m4, identity, "{:?}^4 should equal identity", mov);
    }
}

#[test]
fn test_basic_moves_double_equals_half_turn() {
    // M * M should equal M2 for all basic moves
    let test_pairs = [
        (BasicMove::<3>::U, BasicMove::<3>::U2),
        (BasicMove::<3>::D, BasicMove::<3>::D2),
        (BasicMove::<3>::L, BasicMove::<3>::L2),
        (BasicMove::<3>::R, BasicMove::<3>::R2),
        (BasicMove::<3>::F, BasicMove::<3>::F2),
        (BasicMove::<3>::B, BasicMove::<3>::B2),
    ];

    for (quarter, half) in test_pairs {
        let m = TilePerm::<3>::from(&quarter);
        let m2_composed = &m * &m;
        let m2_direct = TilePerm::<3>::from(&half);
        assert_eq!(m2_composed, m2_direct, "{:?} * {:?} should equal {:?}", quarter, quarter, half);
    }
}

#[test]
fn test_basic_moves_affect_only_their_face_slice() {
    // Each basic move should affect slice 0 of its face and edge tiles on adjacent faces,
    // but should NOT affect other slices of its own face
    let identity = TilePerm::<3>::from(&CubeRotation::ID);

    let test_cases = [
        (BasicMove::<3>::U, Face::Up),
        (BasicMove::<3>::D, Face::Down),
        (BasicMove::<3>::L, Face::Left),
        (BasicMove::<3>::R, Face::Right),
        (BasicMove::<3>::F, Face::Front),
        (BasicMove::<3>::B, Face::Back),
    ];

    for (mov, face) in test_cases {
        let m = TilePerm::<3>::from(&mov);
        let face_slice = Slice { face, slice_index: 0 };

        // Should affect the face's slice 0
        assert!(!m.agree_on(&identity, face_slice),
                "{:?} should affect {:?} slice 0", mov, face);

        // Should not affect other slices of that face (slices 1 and 2 are internal/opposite)
        for slice_idx in 1..3 {
            let other_slice = Slice { face, slice_index: slice_idx };
            assert!(m.agree_on(&identity, other_slice),
                    "{:?} should NOT affect {:?} slice {}", mov, face, slice_idx);
        }
    }
}
