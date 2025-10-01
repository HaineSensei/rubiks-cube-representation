use super::*;
use crate::Face;

#[test]
fn test_slice_0_iterator_terminates_3x3() {
    // Test that slice 0 (end slice) iterator terminates for a 3x3 cube
    let slice = Slice { face: Face::Up, slice_index: 0 };
    let positions: Vec<TilePos> = <Slice as Restriction<3>>::restricted_positions(&slice).collect();

    // Should include the 9 face tiles + 4 edges of 3 tiles each = 21 total
    assert_eq!(positions.len(), 21, "Slice 0 should have 21 positions for 3x3 cube");
}

#[test]
fn test_slice_1_iterator_terminates_3x3() {
    // Test that slice 1 (middle slice) iterator terminates for a 3x3 cube
    let slice = Slice { face: Face::Front, slice_index: 1 };
    let positions: Vec<TilePos> = <Slice as Restriction<3>>::restricted_positions(&slice).collect();

    // Should include only the 4 edges of 3 tiles each = 12 total
    assert_eq!(positions.len(), 12, "Slice 1 should have 12 positions for 3x3 cube");
}

#[test]
fn test_slice_0_iterator_terminates_2x2() {
    // Test that slice 0 works for 2x2 cubes
    let slice = Slice { face: Face::Left, slice_index: 0 };
    let positions: Vec<TilePos> = <Slice as Restriction<2>>::restricted_positions(&slice).collect();

    // Should include the 4 face tiles + 4 edges of 2 tiles each = 12 total
    assert_eq!(positions.len(), 12, "Slice 0 should have 12 positions for 2x2 cube");
}

#[test]
fn test_slice_range_iterator_terminates_3x3() {
    // Test that slice range iterator terminates
    let slice_range = SliceRange {
        face: Face::Right,
        start_slice_index: 0,
        end_slice_index: 2
    };
    let positions: Vec<TilePos> = <SliceRange as Restriction<3>>::restricted_positions(&slice_range).collect();

    // For a 3x3 cube, slices 0, 1, 2 should cover all relevant positions
    // This is more about termination than exact count verification
    assert!(positions.len() > 0, "Slice range should produce some positions");
    assert!(positions.len() < 100, "Slice range should not produce excessive positions");
}

#[test]
fn test_slice_range_single_slice_3x3() {
    // Test slice range with start == end
    let slice_range = SliceRange {
        face: Face::Back,
        start_slice_index: 1,
        end_slice_index: 1
    };
    let range_positions: Vec<TilePos> = <SliceRange as Restriction<3>>::restricted_positions(&slice_range).collect();

    // Should be equivalent to a single slice 1
    let single_slice = Slice { face: Face::Back, slice_index: 1 };
    let single_positions: Vec<TilePos> = <Slice as Restriction<3>>::restricted_positions(&single_slice).collect();

    assert_eq!(range_positions, single_positions,
        "Single-element slice range should match individual slice");
}

#[test]
fn test_combined_restriction_terminates_3x3() {
    // Test that combined restrictions work and terminate
    let slice1 = Slice { face: Face::Up, slice_index: 0 };
    let slice2 = Slice { face: Face::Down, slice_index: 0 };

    let combined = CombinedRestriction {
        first: &slice1,
        second: &slice2
    };

    let positions: Vec<TilePos> = <CombinedRestriction<3, Slice, Slice> as Restriction<3>>::restricted_positions(&combined).collect();

    // Should be sum of both slices
    let pos1: Vec<TilePos> = <Slice as Restriction<3>>::restricted_positions(&slice1).collect();
    let pos2: Vec<TilePos> = <Slice as Restriction<3>>::restricted_positions(&slice2).collect();
    assert_eq!(positions.len(), pos1.len() + pos2.len(),
        "Combined restriction should sum individual restrictions");
}

#[test]
fn test_slice_positions_are_valid_3x3() {
    // Test that all positions returned are within valid cube bounds
    let slice = Slice { face: Face::Front, slice_index: 0 };
    let positions: Vec<TilePos> = <Slice as Restriction<3>>::restricted_positions(&slice).collect();

    for pos in positions {
        // For a 3x3 cube, row and col should be 0, 1, or 2
        assert!(pos.row < 3, "Row {} should be < 3", pos.row);
        assert!(pos.col < 3, "Col {} should be < 3", pos.col);

        // Face should not be opposite Front
        assert_ne!(pos.face, Face::Back, "Face {:?} should not be {:?}", pos.face, Face::Back);
    }
}

#[test]
fn test_slice_includes_face_tiles_for_end_slice_3x3() {
    // Test that slice 0 includes tiles from the specified face
    let slice = Slice { face: Face::Up, slice_index: 0 };
    let positions: Vec<TilePos> = <Slice as Restriction<3>>::restricted_positions(&slice).collect();

    // Should include at least some tiles from the Up face itself
    let up_face_tiles: Vec<_> = positions.iter()
        .filter(|pos| pos.face == Face::Up)
        .collect();

    assert!(up_face_tiles.len() > 0, "End slice should include tiles from the face itself");
    // For a 3x3 cube, should include all 9 tiles from the face
    assert_eq!(up_face_tiles.len(), 9, "End slice should include all 9 face tiles for 3x3");
}

#[test]
fn test_slice_excludes_face_tiles_for_middle_slice_3x3() {
    // Test that slice 1 does not include tiles from the specified face
    let slice = Slice { face: Face::Down, slice_index: 1 };
    let positions: Vec<TilePos> = <Slice as Restriction<3>>::restricted_positions(&slice).collect();

    // Should not include any tiles from the Down face itself
    let down_face_tiles: Vec<_> = positions.iter()
        .filter(|pos| pos.face == Face::Down)
        .collect();

    assert_eq!(down_face_tiles.len(), 0, "Middle slice should not include tiles from the face itself");
}

#[test]
fn test_slice_positions_are_unique_3x3() {
    // Test that slice doesn't return duplicate positions
    let slice = Slice { face: Face::Right, slice_index: 0 };
    let positions: Vec<TilePos> = <Slice as Restriction<3>>::restricted_positions(&slice).collect();

    let mut unique_positions = positions.clone();
    unique_positions.sort_by_key(|pos| (pos.face as u8, pos.row, pos.col));
    unique_positions.dedup();

    assert_eq!(positions.len(), unique_positions.len(),
        "Slice should not produce duplicate positions");
}

#[test]
fn test_slice_range_empty_range_3x3() {
    // Test slice range where start > end (should be empty)
    let slice_range = SliceRange {
        face: Face::Front,
        start_slice_index: 2,
        end_slice_index: 1
    };
    let positions: Vec<TilePos> = <SliceRange as Restriction<3>>::restricted_positions(&slice_range).collect();

    assert_eq!(positions.len(), 0, "Invalid slice range should produce no positions");
}