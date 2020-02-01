//! This crate provides a derive macro for generating from and to tuple conversion functions.

#![no_std]

pub use struple_impl::Struple;

mod private {
    pub trait Sealed {}
}
/// A marker trait implemented by all tuples up to arity 26.
pub trait GenericTuple: private::Sealed {}

/// Implementors of this trait allow the conversion from and to tuples.
pub trait Struple {
    /// The tuple representation of this type.
    type Tuple: GenericTuple;
    /// Constructs a value of this type from its tuple representation.
    fn from_tuple(tuple: Self::Tuple) -> Self;
    /// Turns this value into its tuple representation.
    fn into_tuple(self) -> Self::Tuple;
}

macro_rules! gen_tuple_impl {
    ($($types:ident),*) => { gen_tuple_impl!(@rec [$($types,)*] []); };
    (@rec [] [$($types:ident,)*]) => { gen_tuple_impl!(@do_impl $($types,)*); };
    (@rec [$head:ident, $($tail:ident,)*] [$($types:ident,)*]) => {
        gen_tuple_impl!(@do_impl $($types,)*);
        gen_tuple_impl!(@rec [$($tail,)*] [$($types,)* $head,]);
    };
    (@do_impl $($types:ident,)*) => {
        impl<$($types,)*> private::Sealed for ($($types,)*) {}
        impl<$($types,)*> GenericTuple for ($($types,)*) {}
    };
}
gen_tuple_impl!(A, B, C, D, E, F, G, H, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);
