//! Multiplication operator implementations for cube operations.
//!
//! This module provides `Mul` trait implementations enabling algebraic composition
//! of all cube operations through the `*` operator. The goal is to support natural
//! mathematical syntax like `rotation * move1 * move2`, where operations compose
//! left-to-right following standard cubing notation.
//!
//! # Design Challenge: Trait Coherence
//!
//! Implementing multiplication for all combinations of operation types faces Rust's
//! trait coherence restrictions. Ideally, we'd write:
//! ```ignore
//! impl<Op1: Into<TilePerm<N>>, Op2: Into<TilePerm<N>>> Mul<Op2> for Op1 { ... }
//! ```
//!
//! But this violates the orphan rule since `Mul` is a foreign trait and `Op1` is
//! an unconstrained type parameter (not covered by a local type).
//!
//! # Solution: Explicit Implementation Matrix
//!
//! Instead, we explicitly implement `Mul` for each combination where the left-hand
//! type is local to this crate:
//! - Each move type (`BasicMove`, `WideMove`, etc.) × any `Into<TilePerm<N>>`
//! - `CubeRotation` × each move type (and `TilePerm<N>` elsewhere)
//! - `TilePerm<N>` × any `NonTilePermOperation<N>` (marker trait workaround)
//!
//! This creates a complete multiplication table while satisfying coherence requirements.
//!
//! # Implementation Pattern
//!
//! All implementations follow the same pattern:
//! 1. Convert both operands to `TilePerm<N>` via `Into`
//! 2. Multiply the resulting tile permutations
//! 3. Return the composed permutation
//!
//! For borrowed vs owned variants, we delegate to the core `&T * Op2` implementation
//! to avoid code duplication.

use std::ops::Mul;

use crate::{core::rubiks::{moves::{BasicMove, MiddleMove, RangeMove, SliceMove, WideMove}, tiles::{NonTilePermOperation, TilePerm}}, CubeRotation};

#[cfg(test)]
mod tests;

impl<const N: usize, Op2: Into<TilePerm<N>>> Mul<Op2> for &BasicMove<N> {
    type Output = TilePerm<N>;
    
    fn mul(self, rhs: Op2) -> Self::Output {
        <&BasicMove<N> as Into<TilePerm<N>>>::into(self) * rhs.into()
    }
}

impl<const N: usize, Op2: Into<TilePerm<N>>> Mul<Op2> for BasicMove<N> {
    type Output = TilePerm<N>;

    fn mul(self, rhs: Op2) -> Self::Output {
        &self * rhs
    }
}

impl<const N: usize, Op2: Into<TilePerm<N>>> Mul<Op2> for &WideMove<N> {
    type Output = TilePerm<N>;

    fn mul(self, rhs: Op2) -> Self::Output {
        <&WideMove<N> as Into<TilePerm<N>>>::into(self) * rhs.into()
    }
}

impl<const N: usize, Op2: Into<TilePerm<N>>> Mul<Op2> for WideMove<N> {
    type Output = TilePerm<N>;

    fn mul(self, rhs: Op2) -> Self::Output {
        &self * rhs
    }
}

impl<const N: usize, Op2: Into<TilePerm<N>>> Mul<Op2> for &SliceMove<N> {
    type Output = TilePerm<N>;

    fn mul(self, rhs: Op2) -> Self::Output {
        <&SliceMove<N> as Into<TilePerm<N>>>::into(self) * rhs.into()
    }
}

impl<const N: usize, Op2: Into<TilePerm<N>>> Mul<Op2> for SliceMove<N> {
    type Output = TilePerm<N>;

    fn mul(self, rhs: Op2) -> Self::Output {
        &self * rhs
    }
}

impl<const N: usize, Op2: Into<TilePerm<N>>> Mul<Op2> for &RangeMove<N> {
    type Output = TilePerm<N>;

    fn mul(self, rhs: Op2) -> Self::Output {
        <&RangeMove<N> as Into<TilePerm<N>>>::into(self) * rhs.into()
    }
}

impl<const N: usize, Op2: Into<TilePerm<N>>> Mul<Op2> for RangeMove<N> {
    type Output = TilePerm<N>;

    fn mul(self, rhs: Op2) -> Self::Output {
        &self * rhs
    }
}

impl<const N: usize, Op2: Into<TilePerm<N>>> Mul<Op2> for &MiddleMove<N> {
    type Output = TilePerm<N>;

    fn mul(self, rhs: Op2) -> Self::Output {
        <&MiddleMove<N> as Into<TilePerm<N>>>::into(self) * rhs.into()
    }
}

impl<const N: usize, Op2: Into<TilePerm<N>>> Mul<Op2> for MiddleMove<N> {
    type Output = TilePerm<N>;

    fn mul(self, rhs: Op2) -> Self::Output {
        &self * rhs
    }
}

impl<const N: usize> Mul<BasicMove<N>> for CubeRotation {
    type Output = TilePerm<N>;

    fn mul(self, rhs: BasicMove<N>) -> Self::Output {
        TilePerm::from(self) * <BasicMove<N> as Into<TilePerm<N>>>::into(rhs)
    }
}

impl<const N: usize> Mul<WideMove<N>> for CubeRotation {
    type Output = TilePerm<N>;

    fn mul(self, rhs: WideMove<N>) -> Self::Output {
        TilePerm::from(self) * <WideMove<N> as Into<TilePerm<N>>>::into(rhs)
    }
}

impl<const N: usize> Mul<SliceMove<N>> for CubeRotation {
    type Output = TilePerm<N>;

    fn mul(self, rhs: SliceMove<N>) -> Self::Output {
        TilePerm::from(self) * <SliceMove<N> as Into<TilePerm<N>>>::into(rhs)
    }
}

impl<const N: usize> Mul<RangeMove<N>> for CubeRotation {
    type Output = TilePerm<N>;

    fn mul(self, rhs: RangeMove<N>) -> Self::Output {
        TilePerm::from(self) * <RangeMove<N> as Into<TilePerm<N>>>::into(rhs)
    }
}

impl<const N: usize> Mul<MiddleMove<N>> for CubeRotation {
    type Output = TilePerm<N>;

    fn mul(self, rhs: MiddleMove<N>) -> Self::Output {
        TilePerm::from(self) * <MiddleMove<N> as Into<TilePerm<N>>>::into(rhs)
    }
}

impl<const N: usize, T: NonTilePermOperation<N>> Mul<&T> for &TilePerm<N> 
where 
    for<'a> &'a T: Into<TilePerm<N>>
{
    type Output = TilePerm<N>;

    fn mul(self, rhs: &T) -> Self::Output {
        self * &<&T as Into<TilePerm<N>>>::into(rhs)
    }
}