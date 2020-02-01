extern crate proc_macro;

use proc_macro2::{Literal, Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{
    parse_macro_input, Data, DataStruct, DeriveInput, Fields, FieldsNamed, FieldsUnnamed, Ident,
};

#[proc_macro_derive(Struple)]
pub fn derive_struple(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

fn expand(
    DeriveInput {
        ident: name,
        generics,
        data,
        ..
    }: DeriveInput,
) -> syn::Result<TokenStream2> {
    let fields = match data {
        Data::Struct(DataStruct { fields, .. }) => fields,
        _ => {
            return Err(syn::Error::new(
                Span::call_site(),
                "Struple may only be derived on structs",
            ))
        }
    };

    let field_tys = fields.iter().map(|field| &field.ty);
    let tuple_ty = quote! { ( #( #field_tys, )* ) };

    let (from_tuple_body, into_tuple_body) = make_conversion_impl(&name, fields);

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics ::struple::Struple for #name #ty_generics #where_clause {
            type Tuple = #tuple_ty;
            fn from_tuple(tuple: Self::Tuple) -> Self {
                #from_tuple_body
            }
            fn into_tuple(self) -> Self::Tuple {
                #into_tuple_body
            }
        }
    })
}

fn make_conversion_impl(name: &Ident, fields: Fields) -> (TokenStream2, TokenStream2) {
    let tuple_indices = |len| (0..len).map(Literal::usize_unsuffixed);
    match fields {
        Fields::Named(FieldsNamed { named, .. }) => (
            {
                let indices = tuple_indices(named.len());
                let fields = named.iter().flat_map(|field| field.ident.as_ref());
                quote! { #name { #( #fields: tuple.#indices, )* } }
            },
            {
                let fields = named.iter().flat_map(|field| field.ident.as_ref());
                quote! { ( #( self.#fields, )* ) }
            },
        ),
        Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => (
            {
                let indices = tuple_indices(unnamed.len());
                quote! { #name( #( tuple.#indices, )* ) }
            },
            {
                let indices = tuple_indices(unnamed.len());
                quote! { ( #( self.#indices, )* ) }
            },
        ),
        Fields::Unit => (quote!(#name), quote!(())),
    }
}
