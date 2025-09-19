//! Tile-level permutations for complete cube state manipulation.
//!
//! This module extends the permutation concept from faces and diagonals to individual tiles,
//! providing representations for the positions and permutations of all N²×6 tiles on a cube.
//! This enables fine-grained analysis of cube state transformations beyond face-level operations.

use std::{array::from_fn, ops::{Index, Mul}};

use crate::Face;

mod implementations;

#[cfg(test)]
mod tests;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TilePos {
    pub face: Face,
    pub row: usize,
    pub col: usize
}

// would normally implement Copy, but... feels concerning for big things.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TileGrid<const N: usize> {
    pub vals: [[TilePos;N];N]
}

// would normally implement Copy, but as above, feels too big for that.
// Most of the time, we'll just be passing around references, so it'll be fine anyway.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TilePerm<const N: usize> {
    pub up: TileGrid<N>,
    pub down: TileGrid<N>,
    pub left: TileGrid<N>,
    pub right: TileGrid<N>,
    pub front: TileGrid<N>,
    pub back: TileGrid<N>
}

impl<const N: usize> Index<TilePos> for TilePerm<N> {
    type Output = TilePos;

    fn index(&self, index: TilePos) -> &Self::Output {
        let TilePos { face, row, col } = index;
        match face {
            Face::Up => &self.up.vals[row][col],
            Face::Down => &self.down.vals[row][col],
            Face::Left => &self.left.vals[row][col],
            Face::Right => &self.right.vals[row][col],
            Face::Front => &self.front.vals[row][col],
            Face::Back => &self.back.vals[row][col],
        }
    }
}

impl<const N: usize> Mul for TilePerm<N> {
    type Output = TilePerm<N>;

    fn mul(self, rhs: Self) -> Self::Output {
        TilePerm {
            up: TileGrid { vals: from_fn(|row| from_fn(|col| {
                let tile = TilePos { face: Face::Up, row, col };
                rhs[self[tile]]
            })) },
            down: TileGrid { vals: from_fn(|row| from_fn(|col| {
                let tile = TilePos { face: Face::Down, row, col };
                rhs[self[tile]]
            })) },
            left: TileGrid { vals: from_fn(|row| from_fn(|col| {
                let tile = TilePos { face: Face::Left, row, col };
                rhs[self[tile]]
            })) },
            right: TileGrid { vals: from_fn(|row| from_fn(|col| {
                let tile = TilePos { face: Face::Right, row, col };
                rhs[self[tile]]
            })) },
            front: TileGrid { vals: from_fn(|row| from_fn(|col| {
                let tile = TilePos { face: Face::Front, row, col };
                rhs[self[tile]]
            })) },
            back: TileGrid { vals: from_fn(|row| from_fn(|col| {
                let tile = TilePos { face: Face::Back, row, col };
                rhs[self[tile]]
            })) },
        }
    }
}

impl<const N: usize> TilePerm<N> {
    pub fn inverse(&self) -> Self {
        let mut result = TilePerm {
            up: TileGrid { vals: from_fn(|row| from_fn(|col| TilePos { face: Face::Up, row, col })) },
            down: TileGrid { vals: from_fn(|row| from_fn(|col| TilePos { face: Face::Down, row, col })) },
            left: TileGrid { vals: from_fn(|row| from_fn(|col| TilePos { face: Face::Left, row, col })) },
            right: TileGrid { vals: from_fn(|row| from_fn(|col| TilePos { face: Face::Right, row, col })) },
            front: TileGrid { vals: from_fn(|row| from_fn(|col| TilePos { face: Face::Front, row, col })) },
            back: TileGrid { vals: from_fn(|row| from_fn(|col| TilePos { face: Face::Back, row, col })) },
        };

        for face in [Face::Up, Face::Down, Face::Left, Face::Right, Face::Front, Face::Back] {
            for row in 0..N {
                for col in 0..N {
                    let source = TilePos { face, row, col };
                    let destination = self[source];
                    let dest_grid = match destination.face {
                        Face::Up => &mut result.up.vals,
                        Face::Down => &mut result.down.vals,
                        Face::Left => &mut result.left.vals,
                        Face::Right => &mut result.right.vals,
                        Face::Front => &mut result.front.vals,
                        Face::Back => &mut result.back.vals,
                    };
                    dest_grid[destination.row][destination.col] = source;
                }
            }
        }

        result
    }
}