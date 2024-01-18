#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
use const_generics_rs::{Eq, Ex, Range};

fn main() {
    // Eq<I> + Eq<I> -> Eq<I>
    let a: Eq<7> = 7.into();
    let b: Eq<5> = 5.into();
    let c = a + b;
    println!("{} + {} = {}", a, b, c);

    // Range<I, I> + Range<I, I> -> Range<I, I>
    let a: Range<5, 10> = 6.into();
    let b: Range<8, 15> = 9.into();
    let c = a + b;
    println!("{} + {} = {}", a, b, c);

    // Ex<I> + Ex<I> -> I
    let a: Ex<5> = 6.into();
    let b: Ex<5> = 7.into();
    let c = a + b;
    println!("{} + {} = {}", a, b, c);

    // Eq<I> + Range<I, I> -> Range<I, I>
    let a: Eq<5> = 5.into();
    let b: Range<8, 15> = 9.into();
    let c = a + b;
    println!("{} + {} = {}", a, b, c);

    // Eq<I> + Ex<I> -> Ex<I>
    let a: Eq<5> = 5.into();
    let b: Ex<5> = 6.into();
    let c = a + b;
    println!("{} + {} = {}", a, b, c);

    // Range<I, I> + Ex<I> -> I
    let a: Ex<5> = 6.into();
    let b: Range<8, 15> = 9.into();
    let c = a + b;
    println!("{} + {} = {}", a, b, c);
}
