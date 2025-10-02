#[test]
fn basic_cube_manipulation() {
    use rubiks_cube_representation::*;
    use rubiks_cube_representation::core::cube::schemes::Western;
    use rubiks_cube_representation::core::cube::rotations::Y;
    use rubiks_cube_representation::core::rubiks::moves::BasicMove;

    // Create a solved 3×3×3 cube in Western colors
    let cube = RubiksState::<3>::solved_in(Western);

    // Apply a U move
    let cube_after_u = &cube * &BasicMove::<3>::U;

    // Compose operations algebraically
    let cube_rotated = &cube * &Y * &BasicMove::R * &BasicMove::<3>::U;

    // Check solving state
    assert!(cube.is_solved_in(Western));
    assert!(cube.is_solved_up_to_rotation_in(Western));
}

#[test]
fn tile_permutation_tracking() {
    use rubiks_cube_representation::core::rubiks::tiles::TilePerm;
    use rubiks_cube_representation::core::rubiks::moves::BasicMove;
    use rubiks_cube_representation::core::cube::rotations::X;

    // Convert operations to tile permutations
    let u_perm = TilePerm::<3>::from(&BasicMove::<3>::U);
    let x_perm = TilePerm::<3>::from(&X);

    // Compose permutations
    let combined = &u_perm * &x_perm * &u_perm;

    // Compute inverse
    let inverse = combined.inverse();
    assert_eq!(&combined * &inverse, TilePerm::<3>::ID);
}

#[test]
fn multi_dimensional_cubes() {
    use rubiks_cube_representation::*;
    use rubiks_cube_representation::core::cube::schemes::Western;
    use rubiks_cube_representation::core::rubiks::moves::{WideMove, MiddleMove};

    // Create a 5×5×5 cube
    let cube_5x5 = RubiksState::<5>::solved_in(Western);

    // Wide moves work on any dimension
    let after_wide = &cube_5x5 * &WideMove::<5>::Uw(3);

    // Middle moves adjust to cube size (N/2 for odd N)
    let after_middle = &cube_5x5 * &MiddleMove::<5>::M;
}