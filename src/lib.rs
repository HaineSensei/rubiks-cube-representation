pub mod core;
pub mod algorithms;

// Re-export commonly used types from core modules
pub use core::{Colour, COLOURS};
pub use core::cube::geometry::{Face, CubeDiag, CubeCorner, FACES};
pub use core::cube::rotations::{CubeRotation, X, X3, Y, Y3, Z, Z3};
pub use core::cube::schemes::{ColourScheme, ColourPerm, Western, Japanese};
pub use core::rubiks::{FaceState, RubiksState};
