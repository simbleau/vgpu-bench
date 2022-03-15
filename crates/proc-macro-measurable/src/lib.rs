#![feature(proc_macro_quote)]

extern crate proc_macro;
use proc_macro::quote;
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn measurable_attribute(
    _input: TokenStream,
    annotated_item: TokenStream,
) -> TokenStream {
    let clone = annotated_item.clone();
    let DeriveInput { ident, .. } = parse_macro_input!(clone);
    let name = ident.to_token_stream();
    quote! {
        unsafe impl Sync for $name {}
        unsafe impl Send for $name {}
        #[derive(Debug)]
        #[derive(::serde::Serialize)]
        $annotated_item
    }
    .into()
}
