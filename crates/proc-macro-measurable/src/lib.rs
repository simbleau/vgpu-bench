#![feature(proc_macro_quote)]

extern crate proc_macro;
use proc_macro::*;

#[proc_macro_attribute]
pub fn measurable_attribute(
    _input: TokenStream,
    annotated_item: TokenStream,
) -> TokenStream {
    quote! {
        unsafe impl Sync for Measurable {{}}
        unsafe impl Send for Measurable {{}}
        #[derive(serde::Serialize, Debug)]
        $annotated_item
    }
    .into()
}
