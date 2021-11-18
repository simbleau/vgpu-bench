extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(BenchmarkData)]
pub fn benchmark_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of code as a syntax tree that we can
    // manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_benchmark_macro(&ast)
}

fn impl_benchmark_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let item_name = name.to_string();
    let item_name_ref: &str = &item_name;
    let gen = quote! {
        impl BenchmarkData for #name {
            fn name(&self) -> &'static str {
                #item_name_ref
            }
        }
        impl Benchmark for #name {}
    };
    gen.into()
}
