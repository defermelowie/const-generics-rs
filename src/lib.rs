#![feature(generic_const_exprs)]

use std::fmt::Display;
use std::ops::Add;

use const_bool_expr::{False, If, True};
mod const_bool_expr;

/// Base numeric type
type I = u128;

// -----------------------------------------------------------------------------
// Public types
// -----------------------------------------------------------------------------

/// N ∈ I\E
#[derive(Clone, Copy)]
pub struct Exclude<const E: I>(I);

/// N ∈ {E}
#[derive(Clone, Copy)]
pub struct Equal<const E: I>(I);

/// N ∈ [L; U]
#[derive(Clone, Copy)]
pub struct Range<const L: I, const U: I>(I)
where
    If<{ L <= U }>: True;

// -----------------------------------------------------------------------------
// Conversions from & to base type
// -----------------------------------------------------------------------------

impl<const E: I> From<I> for Exclude<E> {
    fn from(n: I) -> Self {
        assert_ne!(E, n, "n should not be equal to excluded value");
        Self(n)
    }
}

impl<const E: I> From<Exclude<E>> for I {
    fn from(n: Exclude<E>) -> Self {
        n.0
    }
}

impl<const E: I> From<I> for Equal<E> {
    fn from(n: I) -> Self {
        assert_eq!(E, n, "n should be equal to E");
        Self(n)
    }
}

impl<const E: I> From<Equal<E>> for I {
    fn from(n: Equal<E>) -> Self {
        n.0
    }
}

impl<const L: I, const U: I> From<I> for Range<L, U>
where
    If<{ L <= U }>: True,
{
    fn from(n: I) -> Self {
        assert!(L <= n, "n should be bigger than lower bound");
        assert!(U >= n, "n should be smaller than upper bound");
        Self(n)
    }
}

impl<const L: I, const U: I> From<Range<L, U>> for I
where
    If<{ L <= U }>: True,
{
    fn from(n: Range<L, U>) -> Self {
        n.0
    }
}

// -----------------------------------------------------------------------------
// Display trait
// -----------------------------------------------------------------------------

impl<const E: I> Display for Exclude<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<!{}>({})", E, self.0)
    }
}

impl<const E: I> Display for Equal<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}>({})", E, self.0)
    }
}

impl<const L: I, const U: I> Display for Range<L, U>
where
    If<{ L <= U }>: True,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}..{}>({})", L, U, self.0)
    }
}

// -----------------------------------------------------------------------------

impl<const E_LHS: I, const E_RHS: I> PartialEq<Exclude<E_RHS>> for Exclude<E_LHS> {
    fn eq(&self, other: &Exclude<E_RHS>) -> bool {
        self.0 == other.0 && E_LHS == E_RHS
    }
}

impl<const E_LHS: I, const E_RHS: I> PartialEq<Equal<E_RHS>> for Equal<E_LHS> {
    fn eq(&self, other: &Equal<E_RHS>) -> bool {
        self.0 == other.0 && E_LHS == E_RHS
    }
}

impl<const L_LHS: I, const U_LHS: I, const L_RHS: I, const U_RHS: I> PartialEq<Range<L_RHS, U_RHS>>
    for Range<L_LHS, U_LHS> where
    If<{ L_LHS <= U_LHS }>: True,
    If<{ L_RHS <= U_RHS }>: True,
{
    fn eq(&self, other: &Range<L_RHS, U_RHS>) -> bool {
        self.0 == other.0 && L_LHS == L_RHS && U_LHS == U_RHS
    }
}

// -----------------------------------------------------------------------------

impl<const E_LHS: I, const E_RHS: I> Add<Exclude<E_RHS>> for Exclude<E_LHS> {
    type Output = I;

    fn add(self, rhs: Exclude<E_RHS>) -> Self::Output {
        self.0 + rhs.0
    }
}

impl<const E_LHS: I, const E_RHS: I> Add<Equal<E_RHS>> for Equal<E_LHS>
where
    [I; (E_LHS + E_RHS) as usize]:,
{
    type Output = Equal<{ E_LHS + E_RHS }>;

    fn add(self, rhs: Equal<E_RHS>) -> Self::Output {
        Equal::<{ E_LHS + E_RHS }>(self.0 + rhs.0)
    }
}

impl<const E_LHS: I, const E_RHS: I> Add<Exclude<E_RHS>> for Equal<E_LHS>
where
    [I; (E_LHS + E_RHS) as usize]:,
{
    type Output = Exclude<{ E_LHS + E_RHS }>;

    fn add(self, rhs: Exclude<E_RHS>) -> Self::Output {
        Exclude::<{ E_LHS + E_RHS }>(self.0 + rhs.0)
    }
}

impl<const E_LHS: I, const E_RHS: I> Add<Equal<E_RHS>> for Exclude<E_LHS>
where
    [I; (E_LHS + E_RHS) as usize]:,
{
    type Output = Exclude<{ E_LHS + E_RHS }>;

    fn add(self, rhs: Equal<E_RHS>) -> Self::Output {
        Exclude::<{ E_LHS + E_RHS }>(self.0 + rhs.0)
    }
}

// -----------------------------------------------------------------------------
