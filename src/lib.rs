#![allow(dead_code, unused)]

mod serialize;

use core::panic;

use proc_macro::TokenStream;
use syn::{DeriveInput, Expr::Field};

extern crate proc_macro;

#[derive(Debug)]
enum Type {
    Itype, // also includes unsigined
    String,
}

type FieldTuple = (String, Type);

fn get_fields(data: &syn::Data) -> Vec<FieldTuple> {
    let fields = match data {
        syn::Data::Struct(data_struct) => &data_struct.fields,
        _ => unimplemented!(),
    };

    let mut fields_tuple: Vec<FieldTuple> = Vec::new();

    if let syn::Fields::Named { .. } = fields {
        for field in fields {
            let name = field.ident.as_ref().unwrap().to_string();
            let path_ty = match &field.ty {
                syn::Type::Path(type_path) => type_path
                    .path
                    .segments
                    .last()
                    .unwrap()
                    .ident
                    .to_string()
                    .clone(),
                _ => todo!(),
            };

            let ty = match path_ty {
                s if s.starts_with("i") || s.starts_with("u") => Type::Itype,
                s if s.starts_with("String") => Type::String,
                _ => {
                    panic!("type not available")
                }
            };

            fields_tuple.push((name, ty));
        }
    }

    fields_tuple
}

#[proc_macro_derive(Serialize)]
pub fn serialize_struct(_item: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(_item as DeriveInput);

    let fields: Vec<FieldTuple> = get_fields(&ast.data);
    println!("{:#?}", fields);
    TokenStream::new()
}
