# Rubik's Cube Representation

A Rust library for representing and analyzing Rubik's cubes using group theory and tile permutations.

## Features

- **Generic cube dimensions**: Works with N×N×N cubes of any size via const generics
- **Complete move notation system**: All standard moves (basic, wide, slice, range, middle)
- **Tile permutation tracking**: Track individual tile movements through operation sequences
- **Algebraic composition**: Compose rotations and moves using natural `*` operator syntax
- **Multiple color schemes**: Western and Japanese color schemes, plus custom scheme support
- **Rotation-invariant solving**: Detect solved state regardless of cube orientation
- **Group-theoretic foundation**: Cube rotations via diagonal permutations (octahedral group)

## Key Types

### State Representation
- `RubiksState<N>`: Complete state of an N×N×N cube
- `TilePos`: Position of an individual tile (face, row, col)
- `TilePerm<N>`: Permutation of all 6N² tiles on a cube

### Operations
- `CubeRotation`: Whole-cube rotations (X, Y, Z and variants)
- `BasicMove<N>`: Standard face turns (U, D, L, R, F, B with 2/3 variants)
- `WideMove<N>`: Multi-layer turns (Uw, Dw, etc.)
- `SliceMove<N>`: Single internal layer moves (Us, Ds, etc.)
- `RangeMove<N>`: Layer range moves (Ur, Dr, etc.)
- `MiddleMove<N>`: Traditional middle slice moves (M, E, S)

### Color Schemes
- `ColourScheme`: Trait for different color schemes
- `Western`: Standard Western color scheme (white opposite yellow, etc.)
- `Japanese`: Japanese color scheme variant

## Examples

### Basic Cube Manipulation

```rust
use rubiks_cube_representation::*;
use rubiks_cube_representation::core::cube::schemes::Western;
use rubiks_cube_representation::core::cube::rotations::Y;
use rubiks_cube_representation::core::rubiks::moves::BasicMove;

// Create a solved 3×3×3 cube in Western colors
let cube = RubiksState::<3>::solved_in(Western);

// Apply a U move
let cube_after_u = &cube * &BasicMove::<3>::U;

// Compose operations algebraically
let cube_rotated = &cube * &Y * &BasicMove::<3>::R * &BasicMove::<3>::U;

// Check solving state
assert!(cube.is_solved_in(Western));
assert!(cube.is_solved_up_to_rotation_in(Western));
```

### Tile Permutation Tracking

```rust
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
```

### Multi-Dimensional Cubes

```rust
use rubiks_cube_representation::*;
use rubiks_cube_representation::core::cube::schemes::Western;
use rubiks_cube_representation::core::rubiks::moves::{WideMove, MiddleMove};

// Create a 5×5×5 cube
let cube_5x5 = RubiksState::<5>::solved_in(Western);

// Wide moves work on any dimension
let after_wide = &cube_5x5 * &WideMove::<5>::Uw(3);

// Middle moves adjust to cube size (N/2 for odd N)
let after_middle = &cube_5x5 * &MiddleMove::<5>::M;
```

## Mathematical Foundation

This library uses a group-theoretic approach to cube representation:

- **Octahedral group**: Cube rotations represented as permutations of the four main diagonals
- **Tile permutations**: All operations (rotations and moves) convert to permutations of individual tiles
- **Composition**: Operations compose following standard cubing notation (left-to-right)
- **Clean separation**: Abstract group theory (`core::cube`) vs concrete implementation (`core::rubiks`)

## Current Status (v0.1.0)

The mathematical foundation is complete with a fully tested tile permutation system. All standard move types are implemented and verified to agree with cube rotations on their respective slices.

**Implemented:**
- Complete tile permutation system with composition and inverse
- All five move type families with parameterized dimensions
- Algebraic operation system with `*` operator
- Comprehensive test coverage (50+ tests)
- Full documentation

**Future directions:**
- Solving algorithms and analysis tools
- Pattern generation and recognition

## Images

[`Japanese_colors.webp`](Japanese_colors.webp) and [`Western_colors.webp`](Western_colors.webp) are from [here](https://rubiks.fandom.com/wiki/Western_Color_Scheme)