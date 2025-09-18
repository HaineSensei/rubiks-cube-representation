#[cfg(test)]
mod tests {
    use crate::{Face, FACES, CubeCorner, CubeDiag};

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
}