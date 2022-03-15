#![feature(proc_macro_quote)]

extern crate proc_macro;
use proc_macro::quote;
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn measurement(
    _input: TokenStream,
    annotated_item: TokenStream,
) -> TokenStream {
    let clone = annotated_item.clone();
    let DeriveInput { ident, .. } = parse_macro_input!(clone);
    let name = ident.to_token_stream();
    quote! {
        unsafe impl ::core::marker::Send for $name {}
        unsafe impl ::core::marker::Sync for $name {}
        #[derive(::core::fmt::Debug)]
        #[derive(::serde::Serialize)]
        $annotated_item
    }
    .into()
}
