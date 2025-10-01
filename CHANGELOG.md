# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

**Note:** Versions v0.0.x are pre-release development milestones for progress tracking,
not formal releases on crates.io.

## [Unreleased]

## [v0.1.0] - 2025-10-02

### Added
- Complete documentation for tile permutation system
  - Module-level documentation for `restrictions`, `partial`, and `implementations` modules
  - Comprehensive inline documentation for all geometric transformation algorithms
  - `TilePerm::agree_on` method for slice-based permutation comparison
- Comprehensive test suite for move implementations
  - Tests verifying moves agree with cube rotations on affected slices
  - Order and composition tests (M⁴ = identity, M² = M2)
  - Multi-dimensional testing (3×3, 5×5 cubes)
  - Coverage for all move types: `BasicMove`, `WideMove`, `SliceMove`, `RangeMove`, `MiddleMove`
- `From<&Move> for TilePerm<N>` for all move types
- `PartialTilePerm<N>` system for composable sparse permutations with multiplication and inverse
- Helper functions for move construction (`rotate_face_only`, `rotate_outside_of_slice`)
- Additional adjacency methods (`AdjacentFace::side_pos`, `side_pos_at_depth`, `Adjacencies::on_side`)
- `FaceSide * Angle` operator for rotating cardinal directions
- Public (crate-visible) fields on all `*MoveInternal` structs
- 1-indexed layer notation for all move types (layer 1 = face itself)
- Proper opposite face handling for slice_index == N-1 cases
- `From<&CubeRotation> for TilePerm<N>` conversion with geometric tile transformations
- Cube restriction framework (`Restriction<N>` trait, `Slice`, `SliceRange`) for slice-based analysis
- `CubeOperation<N>` trait with blanket implementation for applying operations to `RubiksState<N>`
- `Index<TilePos>` implementation for `RubiksState<N>` to access tile colors by position
- Parameterized all move types by cube dimension (`BasicMove<N>`, `WideMove<N>`, etc.)
- Complete multiplication system for move types and cube rotations
- `NonTilePermOperation<N>` trait for clean algebraic operations
- Reference-based multiplication support for `TilePerm<N>`
- `Clone`, `Copy`, and `Debug` derives for all move types

## [v0.0.5] - 2025-09-21

### Added
- Face adjacency system with cardinal direction edges (`FaceSide`, `AdjacentFace`, `Adjacencies`)
- Complete move notation documentation for all move types (`BasicMove`, `WideMove`, `SliceMove`, `RangeMove`, `MiddleMove`)
- Internal move representations with `From` trait implementations for clean notation-to-geometry conversion
- Comprehensive geometric relationship tests (bidirectional adjacency, principal corner validation)

### Fixed
- Documentation test imports and deprecated rotation constant usage
- Enhanced changelog formatting with proper monospace code segments

## [v0.0.4] - 2025-09-19

### Added
- Tile-level permutation system (`TilePos`, `TileGrid`, `TilePerm`)
- Complete move notation system with directional variants (`BasicMove`, `WideMove`, `SliceMove`, `RangeMove`, `MiddleMove`)
- `TilePerm` composition and inverse operations

### Fixed
- Removed naming conflict between `CubeRotation::E` and `MiddleMove::E`

## [v0.0.3] - 2025-09-18

### Added
- Test coverage for rotation system and color schemes

## [v0.0.2] - 2025-09-18

### Added
- Comprehensive documentation for entire codebase

## [v0.0.1] - 2025-09-18

### Added
- Hierarchical module organization

## [v0.0.0] - 2025-09-17

### Added
- Initial cube representation with rotation-invariant solving