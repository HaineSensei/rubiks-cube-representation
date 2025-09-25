use crate::core::rubiks::moves::{BasicMove, WideMove, SliceMove, RangeMove, MiddleMove};
use crate::core::cube::rotations::CubeRotation;
use super::TilePerm;

impl<const N: usize> From<&BasicMove<N>> for TilePerm<N> {
    fn from(_value: &BasicMove<N>) -> Self {
        todo!("Implement BasicMove to TilePerm conversion")
    }
}

impl<const N: usize> From<BasicMove<N>> for TilePerm<N> {
    fn from(value: BasicMove<N>) -> Self {
        Self::from(&value)
    }
}

impl<const N: usize> From<&WideMove<N>> for TilePerm<N> {
    fn from(_value: &WideMove<N>) -> Self {
        todo!("Implement WideMove to TilePerm conversion")
    }
}

impl<const N: usize> From<WideMove<N>> for TilePerm<N> {
    fn from(value: WideMove<N>) -> Self {
        Self::from(&value)
    }
}

impl<const N: usize> From<&SliceMove<N>> for TilePerm<N> {
    fn from(_value: &SliceMove<N>) -> Self {
        todo!("Implement SliceMove to TilePerm conversion")
    }
}

impl<const N: usize> From<SliceMove<N>> for TilePerm<N> {
    fn from(value: SliceMove<N>) -> Self {
        Self::from(&value)
    }
}

impl<const N: usize> From<&RangeMove<N>> for TilePerm<N> {
    fn from(_value: &RangeMove<N>) -> Self {
        todo!("Implement RangeMove to TilePerm conversion")
    }
}

impl<const N: usize> From<RangeMove<N>> for TilePerm<N> {
    fn from(value: RangeMove<N>) -> Self {
        Self::from(&value)
    }
}

impl<const N: usize> From<&MiddleMove<N>> for TilePerm<N> {
    fn from(_value: &MiddleMove<N>) -> Self {
        todo!("Implement MiddleMove to TilePerm conversion")
    }
}

impl<const N: usize> From<MiddleMove<N>> for TilePerm<N> {
    fn from(value: MiddleMove<N>) -> Self {
        Self::from(&value)
    }
}

impl<const N: usize> From<&CubeRotation> for TilePerm<N> {
    fn from(_value: &CubeRotation) -> Self {
        todo!("Implement CubeRotation to TilePerm conversion")
    }
}

impl<const N: usize> From<CubeRotation> for TilePerm<N> {
    fn from(value: CubeRotation) -> Self {
        Self::from(&value)
    }
}

#[cfg(test)]
mod tests;