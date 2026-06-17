use core::panic;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields};

extern crate proc_macro;
#[proc_macro_derive(Serialize)]
pub fn serialize_struct(_item: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(_item as DeriveInput);

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

        let is_str = match &field.ty {
            syn::Type::Path(type_path) => {
                if let Some(seg) = type_path.path.segments.last() {
                    seg.ident == "String"
                } else {
                    false
                }
            }
            _ => false,
        };

        let comma = if i > 0 { "," } else { "" };

        if is_str {
            quote! {
                json.push_str(&format!("{}\"{}\": \"{}\"",#comma, #field_name, self.#field_ident));
            }
        } else {
            quote! {
                json.push_str(&format!("{}\"{}\": {}", #comma, #field_name, self.#field_ident));
            }
        }
    });
    quote! {
        impl Serializer for #ident {
            fn to_str(&self) -> String {
                let mut json = String::new();
                json.push_str("{");
                #(#field_serializations)*
                json.push_str("}");
                json
            }
        }
    }
    .into()
}
