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
gen_tuple_impl!(
    _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17, _18, _19, _20, _21,
    _22, _23, _24, _25, _26, _27, _28, _29, _30, _31, _32, _33, _34, _35, _36, _37, _38, _39, _40,
    _41, _42, _43, _44, _45, _46, _47, _48, _49, _50, _51, _52, _53, _54, _55, _56, _57, _58, _59,
    _60, _61, _62, _63, _64, _65, _66, _67, _68, _69, _70, _71, _72, _73, _74, _75, _76, _77, _78,
    _79, _80, _81, _82, _83, _84, _85, _86, _87, _88, _89, _90, _91, _92, _93, _94, _95, _96, _97,
    _98, _99, _100
);
