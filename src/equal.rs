use std::{fmt::Display, ops::Add};

use crate::{
    assert::{Assert, True},
    base::{IType, I},
    Ex, Range,
};

/// n ∈ {N}
#[derive(Clone, Copy, Debug)]
pub struct Eq<const N: I>(pub(crate) I);
impl<const N: I> IType for Eq<N> {}

impl<const N: I> From<I> for Eq<N> {
    fn from(n: I) -> Self {
        assert_eq!(N, n, "Could not convert n to Eq<{}>: n != {}", N, N);
        Self(n)
    }
}

impl<const E: I> From<Eq<E>> for I {
    fn from(n: Eq<E>) -> Self {
        n.0
    }
}

impl<const E: I> Display for Eq<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}>({})", E, self.0)
    }
}

impl<const E_LHS: I, const E_RHS: I> PartialEq<Eq<E_RHS>> for Eq<E_LHS> {
    fn eq(&self, other: &Eq<E_RHS>) -> bool {
        self.0 == other.0 && E_LHS == E_RHS
    }
}

// -----------------------------------------------------------------------------

impl<const E_LHS: I, const E_RHS: I> Add<Eq<E_RHS>> for Eq<E_LHS>
where
    [(); { E_LHS + E_RHS } as usize]:,
{
    type Output = Eq<{ E_LHS + E_RHS }>;

    fn add(self, rhs: Eq<E_RHS>) -> Self::Output {
        Self::Output::from(self.0 + rhs.0)
    }
}

impl<const L: I, const U: I, const N: I> Add<Range<L, U>> for Eq<N>
where
    Assert<{ L <= U }>: True,
    Assert<{ L + N <= U + N }>: True,
{
    type Output = Range<{ L + N }, { U + N }>;

    fn add(self, rhs: Range<L, U>) -> Self::Output {
        Self::Output::from(self.0 + rhs.0)
    }
}

impl<const E: I, const N: I> Add<Ex<E>> for Eq<N>
where
    [(); (N + E) as usize]:,
{
    type Output = Ex<{ N + E }>;

    fn add(self, rhs: Ex<E>) -> Self::Output {
        Self::Output::from(self.0 + rhs.0)
    }
}
