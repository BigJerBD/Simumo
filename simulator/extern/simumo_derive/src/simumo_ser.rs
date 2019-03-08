use proc_macro2::{TokenStream, Ident};
use syn::{Type, Path, Fields};
use std::vec::Vec;

/// Generate a block serialized field for a
/// collection of fields
pub fn make_ser_block(fields: &Fields, struct_name: &Ident) -> Vec<TokenStream> {
    //note :: it is probably possible to use ident without cloning
    let names: Vec<Ident> = fields.iter().map(|f| match f.ident.clone() {
        Some(i) => i,
        None => struct_name.clone()
    }).collect();
    let types: Vec<String> = fields.iter().map(|f| type_to_str(&f.ty)).collect();

    let metric_tags: Vec<bool> = fields.iter()
        .map(|f| f.attrs.iter()
            .any(|att| path_to_str(&att.path) == "simumo_metric"))
        .collect();

    let result: Vec<TokenStream> = izip!(&names, &types, &metric_tags)
        .map(|(n, t, m)| make_ser_statement(n, t, m, struct_name))
        .collect();

    result
}

/// Generatea serialize statement
/// if it is a metric from Dimensioned it gets unwraped
pub fn make_ser_statement(field_name: &Ident, ttype: &String,
                          is_metric: &bool, struct_name: &Ident)
                          -> TokenStream {
    let mut tag = field_name.to_string();
    if *is_metric {
        tag.push(':');
        tag.push_str(ttype);

        if field_name == struct_name {
            quote! {state.serialize_field(#tag, &self.0.value_unsafe)?;}
        }
        else{
            quote! {state.serialize_field(#tag, &self.#field_name.value_unsafe)?;}
        }

    } else {
        if field_name == struct_name {
            quote! {state.serialize_field(#tag, &self.0)?;}
        }
        else {
            quote! {state.serialize_field(#tag, &self.#field_name)?;}
        }
    }
}


/// convert type to a string only if it is a path type
/// path types are a sequence of identifier splitted with ::
pub fn type_to_str(unknown_type: &Type) -> String {
    if let syn::Type::Path(type_path) = unknown_type {
        path_to_str(&type_path.path)
    } else {
        String::from("")
    }
}

/// Get the first element in a identifier path
/// example path = path::to::selement
pub fn path_to_str(path: &Path) -> String {
    let segments = &path.segments;
    if segments.len() == 1 {
        //note :: it's probably possible to do it with clones
        segments[0].ident.clone().to_string()
    } else {
        String::from("")
    }
}