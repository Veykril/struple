pub use struple_impl::Struple;

mod private {
    pub trait Sealed {}
}
pub trait GenericTuple: private::Sealed {}

pub trait Struple {
    type Tuple: GenericTuple;
    fn from_tuple(tuple: Self::Tuple) -> Self;
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
