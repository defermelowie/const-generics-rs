use dependent_generics_rs::{Equal, Exclude, Range};

#[test]
fn into_equal_pass() {
    let x: Equal<4> = 4.into();
    let y: u128 = x.into();
    assert_eq!(y, 4)
}

#[test]
#[should_panic]
fn into_equal_panic() {
    let x: Equal<4> = 7.into();
}

#[test]
fn into_exclude_pass() {
    let x: Exclude<4> = 7.into();
    let y: u128 = x.into();
    assert_eq!(y, 7)
}

#[test]
#[should_panic]
fn into_exclude_panic() {
    let x: Exclude<4> = 4.into();
}

#[test]
fn into_range_pass() {
    let x: Range<10, 15> = 12.into();
    let y: u128 = x.into();
    assert_eq!(y, 12)
}

#[test]
#[should_panic]
fn into_range_panic_high() {
    let x: Range<10, 15> = 17.into();
}

#[test]
#[should_panic]
fn into_range_panic_low() {
    let x: Range<10, 15> = 4.into();
}