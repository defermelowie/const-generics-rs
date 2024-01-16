#![feature(generic_const_exprs)]

use std::fmt::Display;
use std::ops::Add;

use const_bool_expr::{If, True, False};
mod const_bool_expr;

/// Base numeric type
type N = u128;

// -----------------------------------------------------------------------------
// Public types
// -----------------------------------------------------------------------------

/// N ∈ I\E
#[derive(Clone, Copy)]
pub struct Exclude<const E: N>(N);

/// N ∈ {E}
#[derive(Clone, Copy)]
pub struct Equal<const E: N>(N);

/// N ∈ [L; U]
#[derive(Clone, Copy)]
pub struct Range<const L: N, const U: N>(N);

// -----------------------------------------------------------------------------
// Conversions from & to base type
// -----------------------------------------------------------------------------

impl<const E: N> From<N> for Exclude<E> {
    fn from(n: N) -> Self {
        assert_ne!(E, n, "n should not be equal to excluded value");
        Self(n)
    }
}

impl<const E: N> From<Exclude<E>> for N {
    fn from(n: Exclude<E>) -> Self {
        n.0
    }
}


impl<const E: N> From<N> for Equal<E> {
    fn from(n: N) -> Self {
        assert_eq!(E, n, "n should be equal to E");
        Self(n)
    }
}

impl<const E: N> From<Equal<E>> for N {
    fn from(n: Equal<E>) -> Self {
        n.0
    }
}

impl<const L: N, const U: N> From<N> for Range<L, U> where If<{L <= U}>: True{
    fn from(n: N) -> Self {
        assert!(L <= n, "n should be bigger than lower bound");
        assert!(U >= n, "n should be smaller than upper bound");
        Self(n)
    }
}

impl<const L: N, const U: N> From<Range<L, U>> for N {
    fn from(n: Range<L, U>) -> Self {
        n.0
    }
}


// -----------------------------------------------------------------------------
// Display trait
// -----------------------------------------------------------------------------

impl<const E: N> Display for Exclude<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<!{}>({})", E, self.0)
    }
}

impl<const E: N> Display for Equal<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}>({})", E, self.0)
    }
}

impl<const L: N, const U: N> Display for Range<L, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}..{}>({})", L, U, self.0)
    }
}

// -----------------------------------------------------------------------------

impl<const E_LHS: N, const E_RHS: N> PartialEq<Exclude<E_RHS>> for Exclude<E_LHS> {
    fn eq(&self, other: &Exclude<E_RHS>) -> bool {
        self.0 == other.0 && E_LHS == E_RHS
    }
}

impl<const E_LHS: N, const E_RHS: N> PartialEq<Equal<E_RHS>> for Equal<E_LHS> {
    fn eq(&self, other: &Equal<E_RHS>) -> bool {
        self.0 == other.0 && E_LHS == E_RHS
    }
}

impl<const L_LHS: N, const U_LHS: N, const L_RHS: N, const U_RHS: N> PartialEq<Range<L_RHS, U_RHS>> for Range<L_LHS, U_LHS> {
    fn eq(&self, other: &Range<L_RHS, U_RHS>) -> bool {
        self.0 == other.0 && L_LHS == L_RHS && U_LHS == U_RHS
    }
}

// -----------------------------------------------------------------------------

impl<const E_LHS: N, const E_RHS: N> Add<Exclude<E_RHS>> for Exclude<E_LHS> {
    type Output = N;

    fn add(self, rhs: Exclude<E_RHS>) -> Self::Output {
        self.0 + rhs.0
    }

}

impl<const E_LHS: N, const E_RHS: N> Add<Equal<E_RHS>> for Equal<E_LHS>
where
    [N; (E_LHS + E_RHS) as usize]:,
{
    type Output = Equal<{ E_LHS + E_RHS }>;

    fn add(self, rhs: Equal<E_RHS>) -> Self::Output {
        Equal::<{ E_LHS + E_RHS }>(self.0 + rhs.0)
    }
}

impl<const E_LHS: N, const E_RHS: N> Add<Exclude<E_RHS>> for Equal<E_LHS>
where
    [N; (E_LHS + E_RHS) as usize]:,
{
    type Output = Exclude<{ E_LHS + E_RHS }>;

    fn add(self, rhs: Exclude<E_RHS>) -> Self::Output {
        Exclude::<{ E_LHS + E_RHS }>(self.0 + rhs.0)
    }
}

impl<const E_LHS: N, const E_RHS: N> Add<Equal<E_RHS>> for Exclude<E_LHS>
where
    [N; (E_LHS + E_RHS) as usize]:,
{
    type Output = Exclude<{ E_LHS + E_RHS }>;

    fn add(self, rhs: Equal<E_RHS>) -> Self::Output {
        Exclude::<{ E_LHS + E_RHS }>(self.0 + rhs.0)
    }
}

// -----------------------------------------------------------------------------
