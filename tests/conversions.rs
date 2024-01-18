#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
use const_generics_rs::{Eq, Ex, Range};

#[test]
fn into_equal_eq() {
    let x: Eq<4> = 4.into();
    let y: u128 = x.into();
    assert_eq!(y, 4)
}

#[test]
#[should_panic]
fn into_equal_ne() {
    let _x: Eq<4> = 7.into();
}

#[test]
fn into_exclude_ne() {
    let x: Ex<4> = 7.into();
    let y: u128 = x.into();
    assert_eq!(y, 7)
}

#[test]
#[should_panic]
fn into_exclude_eq() {
    let _x: Ex<4> = 4.into();
}

#[test]
fn into_range_inside() {
    let x: Range<10, 15> = 12.into();
    let y: u128 = x.into();
    assert_eq!(y, 12)
}

#[test]
#[should_panic]
fn into_range_above() {
    let _x: Range<10, 15> = 17.into();
}

#[test]
#[should_panic]
fn into_range_below() {
    let _x: Range<10, 15> = 4.into();
}
