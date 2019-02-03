extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(TypeInfo)]
pub fn string_rep(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let tokens = quote! {
        impl TypeInfo for #name {
            fn type_name() -> String {String::from(stringify!(#name))}
            fn type_of(&self) ->  String {String::from(stringify!(#name))}

        }
    };
    TokenStream::from(tokens)
}
