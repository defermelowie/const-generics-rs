use std::{fmt::Display, ops::Add};

use crate::base::{IType, I};
use crate::equal::Eq;

/// n ∈ I\E
#[derive(Clone, Copy, Debug)]
pub struct Ex<const E: I>(pub(crate) I);
impl<const E: I> IType for Ex<E> {}

impl<const E: I> From<I> for Ex<E> {
    fn from(n: I) -> Self {
        assert_ne!(E, n, "Could not convert n to Ex<{}>: n == {}", E, E);
        Self(n)
    }
}

impl<const E: I> From<Ex<E>> for I {
    fn from(n: Ex<E>) -> Self {
        n.0
    }
}

impl<const E: I> Display for Ex<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<!{}>({})", E, self.0)
    }
}

impl<const E_LHS: I, const E_RHS: I> PartialEq<Ex<E_RHS>> for Ex<E_LHS> {
    fn eq(&self, other: &Ex<E_RHS>) -> bool {
        self.0 == other.0 && E_LHS == E_RHS
    }
}

impl<const E_LHS: I, const E_RHS: I> Add<Ex<E_RHS>> for Ex<E_LHS> {
    type Output = I;

    fn add(self, rhs: Ex<E_RHS>) -> Self::Output {
        self.0 + rhs.0
    }
}

// -----------------------------------------------------------------------------

impl<const E_LHS: I, const E_RHS: I> Add<Ex<E_RHS>> for Eq<E_LHS>
where
    [(); (E_LHS + E_RHS) as usize]:,
{
    type Output = Ex<{ E_LHS + E_RHS }>;

    fn add(self, rhs: Ex<E_RHS>) -> Self::Output {
        Self::Output::from(self.0 + rhs.0)
    }
}

impl<const E_LHS: I, const E_RHS: I> Add<Eq<E_RHS>> for Ex<E_LHS>
where
    [(); (E_RHS + E_LHS) as usize]:,
{
    type Output = Ex<{ E_RHS + E_LHS }>;

    fn add(self, rhs: Eq<E_RHS>) -> Self::Output {
        rhs + self
    }
}