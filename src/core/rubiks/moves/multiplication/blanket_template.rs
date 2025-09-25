impl<const N: usize, Op1: CubeOperation<N>, Op2: CubeOperation<N>> Mul<Op2> for Op1 {
    type Output = TilePerm<N>;
    fn mul(self, rhs: Op2) -> Self::Output {
        <Self as Into<TilePerm<N>>>::into(self) * rhs.into()
    }
}