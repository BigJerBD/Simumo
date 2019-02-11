extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate itertools;
extern crate syn;

use proc_macro2::{TokenStream, Ident, Span};
use syn::{parse_macro_input, DeriveInput, Type, Path, Pat};
use std::vec::Vec;

mod simumo_ser;

use simumo_ser::*;

/// registerable component derivation
///
/// help registering types to the ECS
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

/// Custom serialization derivation
///
/// can handle custom field name for field having the "simumo_metric" tag
#[proc_macro_derive(SimumoSerialize, attributes(simumo_metric))]
pub fn simumo_serialize(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    //let ast2: Pat = syn::parse(input.clone()).unwrap();
    let ast: DeriveInput = syn::parse(input).unwrap();

    let name = &ast.ident;

    if let syn::Data::Struct(body) = &ast.data {
        let ser_block = make_ser_block(&body.fields, name);
        let size = ser_block.len();
        let output = quote! {

            impl Serialize for #name {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok,S::Error>
                where S: Serializer {
                    let mut state = serializer.serialize_struct(stringify!(#name), #size)?;
                    //expand serialization Tokens
                    #(#ser_block)*
                    state.end()
                }
            }
        };
        output.into()
    } else {
        panic!("#[derive(SimumoLoggable)] is only defined for structs");
    }
}


/// Macro that provide a collection of derivation
/// for components with the role of Tag
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
            // note :: we should not serialzie tag. it is technically useless, but maybe not
            //Serialize,
            )]
        #input

    };
    output.into()
}

/// Macro that provide a collection of derivation
/// for components that have the role of data in Simumo
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
            SimumoSerialize
            )]
        #input
    };
    output.into()
}

/// Macro that provide a collection of derivation
/// for a basic component in simumo
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

