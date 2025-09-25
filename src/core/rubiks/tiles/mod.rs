//! Tile-level permutations for complete cube state manipulation.
//!
//! This module extends the permutation concept from faces and diagonals to individual tiles,
//! providing representations for the positions and permutations of all N²×6 tiles on a cube.
//! This enables fine-grained analysis of cube state transformations beyond face-level operations.

use std::{array::from_fn, ops::{Index, Mul}};

use crate::{core::rubiks::moves::{BasicMove, MiddleMove, RangeMove, SliceMove, WideMove}, CubeRotation, Face, RubiksState};

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

// tile_perm[tile] -> 

impl<'a, 'b, const N: usize> Mul<&'b TilePerm<N>> for &'a TilePerm<N> {
    type Output = TilePerm<N>;

    fn mul(self, rhs: &TilePerm<N>) -> Self::Output {
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

impl<'a, const N: usize> Mul<&'a Self> for TilePerm<N> {
    type Output = Self;

    fn mul(self, rhs: &'a Self) -> Self::Output {
        &self * rhs
    }
}

impl<const N: usize> Mul<TilePerm<N>> for &'_ TilePerm<N> {
    type Output = TilePerm<N>;

    fn mul(self, rhs: TilePerm<N>) -> Self::Output {
        self * &rhs
    }
}

impl<const N: usize> Mul for TilePerm<N> {
    type Output = TilePerm<N>;

    fn mul(self, rhs: Self) -> Self::Output {
        &self * &rhs
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

pub(crate) trait CubeOperation<const N: usize>: Into<TilePerm<N>> {
    fn on(self, cube: &RubiksState<N>) -> RubiksState<N>;
}

impl<'a, const N: usize, Operation> CubeOperation<N> for &'a Operation 
where
    &'a Operation: Into<TilePerm<N>>
{
    fn on(self, cube: &RubiksState<N>) -> RubiksState<N> {
        let tile_perm: TilePerm<N> = self.into();
        tile_perm.on(cube)
    }
}

impl<const N: usize> CubeOperation<N> for TilePerm<N> {
    fn on(self, cube: &RubiksState<N>) -> RubiksState<N> {
        let perm_inverse = self.inverse();
        RubiksState { 
            up: super::FaceState { 
                vals: 
                from_fn(|row| 
                    from_fn(|col| 
                        cube[
                            perm_inverse[
                                TilePos{face: Face::Up, row, col}
                            ]
                        ]
                    )
                ) 
            }, 
            down: super::FaceState {
                vals:
                from_fn(|row|
                    from_fn(|col|
                        cube[
                            perm_inverse[
                                TilePos{face: Face::Down, row, col}
                            ]
                        ]
                    )
                )
            }, 
            left: super::FaceState {
                vals:
                from_fn(|row|
                    from_fn(|col|
                        cube[
                            perm_inverse[
                                TilePos{face: Face::Left, row, col}
                            ]
                        ]
                    )
                )
            }, 
            right: super::FaceState {
                vals:
                from_fn(|row|
                    from_fn(|col|
                        cube[
                            perm_inverse[
                                TilePos{face: Face::Right, row, col}
                            ]
                        ]
                    )
                )
            }, 
            front: super::FaceState {
                vals:
                from_fn(|row|
                    from_fn(|col|
                        cube[
                            perm_inverse[
                                TilePos{face: Face::Front, row, col}
                            ]
                        ]
                    )
                )
            }, 
            back: super::FaceState {
                vals:
                from_fn(|row|
                    from_fn(|col|
                        cube[
                            perm_inverse[
                                TilePos{face: Face::Back, row, col}
                            ]
                        ]
                    )
                )
            },
        }
    }
}

// notation cube * <Ops multiplied in order> also works once we've implemented something else that won't work as a blanket impl for annoying reasons...
impl<const N: usize, T: CubeOperation<N>> Mul<T> for RubiksState<N> {
    type Output = RubiksState<N>;

    fn mul(self, rhs: T) -> Self::Output {
        rhs.on(&self)
    }
}

pub(crate) trait NonTilePermOperation<const N: usize>: Into<TilePerm<N>> {}

impl<const N: usize> NonTilePermOperation<N> for BasicMove<N> {}
impl<const N: usize> NonTilePermOperation<N> for WideMove<N> {}
impl<const N: usize> NonTilePermOperation<N> for SliceMove<N> {}
impl<const N: usize> NonTilePermOperation<N> for RangeMove<N> {}
impl<const N: usize> NonTilePermOperation<N> for MiddleMove<N> {}
impl<const N: usize> NonTilePermOperation<N> for CubeRotation {}