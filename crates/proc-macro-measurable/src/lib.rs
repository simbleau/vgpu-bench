extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Measurable)]
pub fn proc_macro_measurable(input: TokenStream) -> TokenStream {
    // Construct a representation of code as a syntax tree that we can
    // manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_measurable_macro(&ast)
}

fn impl_measurable_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let expanded = quote! {
        impl Send for #name {}
        impl Sync for #name {}
        [#derive(serde::Serialize, Debug)]
    };
    TokenStream::from(expanded)
}
