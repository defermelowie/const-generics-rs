use std::fmt::{Debug, Display};

/// Base integer type
pub type I = u128;

/// Trait combo for wrapped integer types to make sure they can be:
/// - Constructed using [From<T>]<[I]>
/// - Unwrapped using [Into<T>]<[I]>
/// - Displayed using [Display]
/// - Debug representation using [Debug]
pub(crate) trait IType: Display + Debug + From<I> + Into<I> {}
