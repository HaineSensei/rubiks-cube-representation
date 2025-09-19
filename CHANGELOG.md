# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

**Note:** Versions v0.0.x are pre-release development milestones for progress tracking,
not formal releases on crates.io.

## [Unreleased]

### TODO
- Implement From<CubeRotation> for TilePerm<N> conversion in tiles::implementations
  - Map diagonal permutations to individual tile movements
  - Handle rotation of tiles around cube faces according to geometric relationships
- Implement From<Move> for TilePerm<N> for all move types (BasicMove, WideMove, SliceMove, RangeMove, MiddleMove)
  - Define how each move type affects individual tile positions
  - Handle layer-specific movements and directional variants (2, 3) for multi-dimensional cubes
- Create comprehensive tests for tile permutation system
  - Test tile permutation composition and inverse operations
  - Verify conversion accuracy from rotations and moves
  - Test edge cases for different cube dimensions
- Implement actual face turn operations that modify RubiksState
  - Apply TilePerm transformations to RubiksState color arrays
  - Enable cube scrambling and move execution

### In Progress
- Tile-level permutation system foundation complete, implementations pending

## [v0.0.4] - 2025-09-19

### Added
- Tile-level permutation system (TilePos, TileGrid, TilePerm)
- Complete move notation system with directional variants (BasicMove, WideMove, SliceMove, RangeMove, MiddleMove)
- TilePerm composition and inverse operations

### Fixed
- Removed naming conflict between CubeRotation::E and MiddleMove::E

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