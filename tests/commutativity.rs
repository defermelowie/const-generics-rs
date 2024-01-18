#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
use const_generics_rs::{Eq, Ex, Range};

#[test]
fn add_equal_equal_commutative() {
    // Eq<I> + Eq<I> -> Eq<I>
    let a: Eq<7> = 7.into();
    let b: Eq<5> = 5.into();
    let c = a + b;
    let d = b + a;
    assert_eq!(c, d);
}

#[test]
fn add_range_range_commutative() {
    // Range<I, I> + Range<I, I> -> Range<I, I>
    let a: Range<5, 10> = 6.into();
    let b: Range<8, 15> = 9.into();
    let c = a + b;
    let d = b + a;
    assert_eq!(c, d);
}

#[test]
fn add_exclude_exclude_commutative() {
    // Ex<I> + Ex<I> -> I
    let a: Ex<5> = 6.into();
    let b: Ex<5> = 7.into();
    let c = a + b;
    let d = b + a;
    assert_eq!(c, d);
}

#[test]
fn add_equal_range_commutative() {
    // Eq<I> + Range<I, I> -> Range<I, I>
    let a: Eq<5> = 5.into();
    let b: Range<8, 15> = 9.into();
    let c = a + b;
    let d = b + a;
    assert_eq!(c, d);
}

#[test]
fn add_equal_exclude_commutative() {
    // Eq<I> + Ex<I> -> Ex<I>
    let a: Eq<5> = 5.into();
    let b: Ex<5> = 6.into();
    let c = a + b;
    let d = b + a;
    assert_eq!(c, d);
}

#[test]
fn add_range_exclude_commutative() {
    // Range<I, I> + Ex<I> -> I
    let a: Ex<5> = 6.into();
    let b: Range<8, 15> = 9.into();
    let c = a + b;
    let d = b + a;
    assert_eq!(c, d);
}
