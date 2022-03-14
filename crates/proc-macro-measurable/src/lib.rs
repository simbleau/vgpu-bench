#![feature(proc_macro_quote)]

extern crate proc_macro;
use proc_macro::*;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn measurable_attribute(
    _input: TokenStream,
    annotated_item: TokenStream,
) -> TokenStream {
    /*
    let input = parse_macro_input!(annotated_item);
    let DeriveInput { ident, .. } = input;
    quote! {
        unsafe impl Sync for $ident {}
        unsafe impl Send for $ident {}
        #[derive(serde::Serialize, Debug)]
        $annotated_item
    }
    .into()
    */
    // TODO disabled for the time being.
    quote! {
        $annotated_item
    }
}
