extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro2::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(SimumoComponent)]
pub fn simumo_component(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let output = quote! {
        impl SimumoComponent for #name{
            fn register(&self, simulation: &mut World, isdone: &mut bool){
                if !*isdone {
                    simulation.register::<#name>();
                    *isdone = true;
                }
            }
        }
    };
    output.into()
}


#[proc_macro_attribute]
pub fn simucomponent_tag(_metadata: proc_macro::TokenStream, input: proc_macro::TokenStream)
                         -> proc_macro::TokenStream {
    let input: TokenStream = input.into();
    let output = quote! {
        #[derive(
            Component,
            SimumoComponent,
            TypeInfo,
            Debug,
            Clone,
            //Serialize,
            )]
        #input
    };
    output.into()
}


#[proc_macro_attribute]
pub fn simucomponent_data(_metadata: proc_macro::TokenStream, input: proc_macro::TokenStream)
                          -> proc_macro::TokenStream {
    let input: TokenStream = input.into();
    let output = quote! {
        #[derive(
            Component,
            SimumoComponent,
            TypeInfo,
            Debug,
            Clone,
            //Serialize
            )]
        #input
    };
    output.into()
}


#[proc_macro_attribute]
pub fn simucomponent_base(_metadata: proc_macro::TokenStream, input: proc_macro::TokenStream)
                          -> proc_macro::TokenStream {
    let input: TokenStream = input.into();
    let output = quote! {
        #[derive(
            Component,
            SimumoComponent,
            TypeInfo
            )]
        #input
    };
    output.into()
}

