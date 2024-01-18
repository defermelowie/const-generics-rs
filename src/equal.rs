use std::{fmt::Display, ops::Add};

use crate::base::{IType, I};

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

impl<const E_LHS: I, const E_RHS: I> Add<Eq<E_RHS>> for Eq<E_LHS>
where
    [(); { E_LHS + E_RHS } as usize]:,
{
    type Output = Eq<{ E_LHS + E_RHS }>;

    fn add(self, rhs: Eq<E_RHS>) -> Self::Output {
        Eq::<{ E_LHS + E_RHS }>(self.0 + rhs.0)
    }
}
