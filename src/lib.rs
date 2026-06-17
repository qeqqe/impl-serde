use core::panic;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields};

extern crate proc_macro;
#[proc_macro_derive(Serialize)]
pub fn serialize_struct(_item: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(_item as DeriveInput);
    println!("{:#?}", ast);

    let ident = ast.ident;
    let fields = match ast.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields_named) => fields_named.named,
            _ => panic!("Only named fields are supported"),
        },
        _ => panic!("Serialize derive only works on Structs"),
    };

    let field_serializations = fields.iter().enumerate().map(|(i, field)| {
        let field_ident = field.ident.as_ref().unwrap();
        let field_name = field_ident.to_string();

        let seg_ident = match &field.ty {
            syn::Type::Path(type_path) => {
                if let Some(seg) = type_path.path.segments.last() {
                    &seg.ident.to_string()
                } else {
                    "_Invalid"
                }
            }
            _ => "_Invalid",
        };

        let is_str = seg_ident == "String" || seg_ident.to_lowercase() == "char";
        let is_num =
            seg_ident.starts_with('i') || seg_ident.starts_with('u') || seg_ident.starts_with('f');
        let is_vec = seg_ident == "Vec"; // need to handle non-(primitive/string/char) types

        let comma = if i > 0 { "," } else { "" };

        if is_str {
            quote! {
                json.push_str(&format!("{}\"{}\": \"{}\"",#comma, #field_name, self.#field_ident));
            }
        } else if is_num {
            quote! {
                json.push_str(&format!("{}\"{}\": {}", #comma, #field_name, self.#field_ident));
            }
        } else if is_vec {
            quote! {
                json.push_str(&format!("\"{}\": [\n", #field_name));
                let v = &self.#field_ident;
                for item in v {
                    json.push_str(&format!("{},\n", item));
                }
                json.push_str("\n],\n");
            }
        }
        // for now we will derive that the type is struct (already serialized)
        else {
            quote! {

                json.push_str(&format!("\"{}\": ", #field_name));
                json.push_str(&self.#field_ident.to_str());
                json.push_str(",\n");
            }
        }
    });

    quote! {
        impl Serializer for #ident {
            fn to_str(&self) -> String {
                let mut json = String::new();
                json.push_str("{\n");
                #(#field_serializations)*
                json.push_str("\n}");
                json
            }
        }
    }
    .into()
}
