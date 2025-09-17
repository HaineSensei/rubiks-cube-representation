# Rubik's Cube Representation

A Rust library for representing and analyzing Rubik's cubes using group theory.

## Features

- **Generic cube dimensions**: Works with NxNxN cubes of any size
- **Multiple color schemes**: Support for Western and Japanese color schemes, plus custom schemes
- **Rotation-invariant solving detection**: Can detect if a cube is solved regardless of orientation
- **Efficient group operations**: Cube rotations represented as permutations of main diagonals

## Key Types

- `RubiksState<const DIM: usize>`: Represents the state of a DIM×DIM×DIM cube
- `CubeRotation`: Represents rotations of the entire cube (not face turns)
- `ColourScheme`: Trait for different color schemes (Western, Japanese, custom)

## Example

```rust
use rubiks_cube_representation::*;

// Create a solved 3x3x3 cube in Western colors
let cube = RubiksState::<3>::solved_in(Western);

// Check if it's solved
assert!(cube.is_solved());

// Check if it's solved even when viewed from different orientations
assert!(cube.is_solved_up_to_rotation_in(Western));
```

## Current Status

This is an early-stage library focused on cube representation and rotation detection. Face turn operations (R, L, U, D, F, B moves) are not yet implemented.

## Images

[`Japanese_colors.webp`](Japanese_colors.webp) and [`Western_colors.webp`](Western_colors.webp) are from [here](https://rubiks.fandom.com/wiki/Western_Color_Scheme)