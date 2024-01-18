//! # Playing with const generics
//! 
//! This library defines the following wrapper types for [I], ordered on specificity:
//! 1. [Eq] - An integer equal to a specific value
//! 1. [Range] - An integer within a range of values
//! 1. [Ex] - An integer not equal to a specific value
//! 
//! Additions of these types have the following properties:
//! - [x] `Eq<I>       + Eq<I>       -> Eq<I>`
//! - [x] `Eq<I>       + Range<I, I> -> Range<I, I>`
//! - [x] `Eq<I>       + Ex<I>       -> Ex<I>`
//! - [x] `Range<I, I> + Range<I, I> -> Range<I, I>`
//! - [x] `Range<I, I> + Ex<I>       -> I`
//! - [x] `Ex<I>       + Ex<I>       -> I`

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
#![allow(private_bounds)]

mod assert;
mod base;
mod equal;
mod range;
mod exclude;

pub use base::I;
pub use equal::Eq;
pub use range::Range;
pub use exclude::Ex;
