use std::fmt::Display;
use std::ops::Add;

use crate::assert::{Assert, True};
use crate::base::{IType, I};
use crate::equal::Eq;

/// n ∈ \[L; U\]
#[derive(Clone, Copy, Debug)]
pub struct Range<const L: I, const U: I>(pub(crate) I)
where
    Assert<{ L <= U }>: True;
impl<const L: I, const U: I> IType for Range<L, U> where Assert<{ L <= U }>: True {}

impl<const L: I, const U: I> From<I> for Range<L, U>
where
    Assert<{ L <= U }>: True,
{
    fn from(n: I) -> Self {
        assert!(
            L <= n,
            "Could not convert n to Range<{}, {}>: n < {}",
            L,
            U,
            L
        );
        assert!(
            U >= n,
            "Could not convert n to Range<{}, {}>: n > {}",
            L,
            U,
            U
        );
        Self(n)
    }
}

impl<const L: I, const U: I> From<Range<L, U>> for I
where
    Assert<{ L <= U }>: True,
{
    fn from(n: Range<L, U>) -> Self {
        n.0
    }
}

impl<const L: I, const U: I> Display for Range<L, U>
where
    Assert<{ L <= U }>: True,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}..{}>({})", L, U, self.0)
    }
}

impl<const L_LHS: I, const U_LHS: I, const L_RHS: I, const U_RHS: I> PartialEq<Range<L_RHS, U_RHS>>
    for Range<L_LHS, U_LHS>
where
    Assert<{ L_LHS <= U_LHS }>: True,
    Assert<{ L_RHS <= U_RHS }>: True,
{
    fn eq(&self, other: &Range<L_RHS, U_RHS>) -> bool {
        self.0 == other.0 && L_LHS == L_RHS && U_LHS == U_RHS
    }
}

impl<const L_LHS: I, const U_LHS: I, const L_RHS: I, const U_RHS: I> Add<Range<L_RHS, U_RHS>>
    for Range<L_LHS, U_LHS>
where
    Assert<{ L_LHS <= U_LHS }>: True,
    Assert<{ L_RHS <= U_RHS }>: True,
    Assert<{ L_LHS + L_RHS <= U_LHS + U_RHS }>: True,
{
    type Output = Range<{ L_LHS + L_RHS }, { U_LHS + U_RHS }>;

    fn add(self, rhs: Range<L_RHS, U_RHS>) -> Self::Output {
        Self::Output::from(self.0 + rhs.0)
    }
}

// -----------------------------------------------------------------------------

impl<const L: I, const U: I, const N: I> Add<Eq<N>> for Range<L, U>
where
    Assert<{ L <= U }>: True,
    Assert<{ L + N <= U + N }>: True,
{
    type Output = Range<{ L + N }, { U + N }>;

    fn add(self, rhs: Eq<N>) -> Self::Output {
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
        rhs + self
    }
}