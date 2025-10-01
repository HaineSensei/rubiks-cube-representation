use crate::{FACES, CubeDiag};
use super::{FaceSide, FACE_SIDES};

#[test]
fn test_principal_corner_consistency() {
    for &face in &FACES {
        let corner = face.principal_corner();
        let diag = face.principal_diag();

        // The principal corner should convert to the principal diagonal
        assert_eq!(CubeDiag::from(corner), diag,
                    "Face {:?}: principal_corner -> diagonal mismatch", face);

        // The principal corner should touch the face
        assert!(corner.touching(face),
                "Face {:?}: principal corner doesn't touch the face", face);
    }
}

#[test]
fn test_bidirectional_adjacency() {
    for &face in &FACES {
        for &side in &FACE_SIDES {
            let adjacent = face.adjacent(side);
            let adjacent_adjacent = adjacent.face.adjacent(adjacent.side);

            assert_eq!(face, adjacent_adjacent.face,
                "Face {:?} side {:?}: bidirectional adjacency failed for face", face, side);
            assert_eq!(side, adjacent_adjacent.side,
                "Face {:?} side {:?}: bidirectional adjacency failed for side", face, side);
        }
    }
}

#[test]
fn test_principal_corner_adjacency() {
    for &face in &FACES {
        let north_adjacent = face.adjacent(FaceSide::North);
        let west_adjacent = face.adjacent(FaceSide::West);

        let principal_corner = face.principal_corner();

        // The principal corner should touch the face itself and its north and west adjacent faces
        assert!(principal_corner.touching(face),
            "Face {:?}: principal corner doesn't touch the face itself", face);
        assert!(principal_corner.touching(north_adjacent.face),
            "Face {:?}: principal corner doesn't touch north adjacent face {:?}", face, north_adjacent.face);
        assert!(principal_corner.touching(west_adjacent.face),
            "Face {:?}: principal corner doesn't touch west adjacent face {:?}", face, west_adjacent.face);
    }
}