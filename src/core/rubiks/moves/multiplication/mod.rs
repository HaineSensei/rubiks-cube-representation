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