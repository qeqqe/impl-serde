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

        let seg_ident = match &field.ty {
            syn::Type::Path(type_path) => {
                if let Some(seg) = type_path.path.segments.last() {
                    seg.ident.to_string()
                } else {
                    "_Invalid".to_string()
                }
            }
            _ => "_Invalid".to_string(),
        };

        let is_str = seg_ident == "String" || seg_ident.to_lowercase() == "char";
        let is_num =
            seg_ident.starts_with('i') || seg_ident.starts_with('u') || seg_ident.starts_with('f');
        let is_vec = seg_ident == "Vec" || seg_ident == "HashSet"; // need to handle non-(primitive/string/char) types
        let is_map = seg_ident == "HashMap";

        let comma = if i > 0 { ",\n" } else { "" };

        if is_str {
            quote! {
                json.push_str(#comma);
                json.push_str(&format!("{}\"{}\": \"{}\"", indent, #field_name, self.#field_ident));
            }
        } else if is_num {
            quote! {
                json.push_str(#comma);
                json.push_str(&format!("{}\"{}\": {}", indent, #field_name, self.#field_ident));
            }
        }
        //
        else if is_vec {
            quote! {
                json.push_str(#comma);
                // one extra level inside vec
                let item_indent = "  ".repeat(depth + 2);
                let v = &self.#field_ident;
                json.push_str(&format!("{}\"{}\": [", indent, #field_name));
                if v.is_empty() {
                    json.push_str("]");
                } else {
                    json.push_str("\n");
                    for (idx, item) in v.iter().enumerate() {
                        if idx > 0 { json.push_str(",\n"); }
                        json.push_str(&format!("{}{}", item_indent, item));
                    }
                    json.push_str(&format!("\n{}]", indent));
                }
            }
        } else if is_map {
            quote! {
                json.push_str(#comma);
                let item_ident = "  ".repeat(depth + 2);
                let map = &self.#field_ident;
                json.push_str(&format!("{}\"{}\": {{", indent, #field_name));
                if map.is_empty() {
                    json.push_str("]")
                } else {
                    json.push_str("\n");
                    for (idx, (k, v)) in map.iter().enumerate() {
                        if idx > 0 { json.push_str(",\n"); }
                        json.push_str(&format!("{}\"{}\": {}", item_ident, k, v));
                    }
                    json.push_str(&format!("\n{}}}", indent));
                }
            }
        }
        // for now we will derive that the type is struct (already serialized)
        else {
            quote! {
                json.push_str(#comma);
                json.push_str(&format!("{}\"{}\": ", indent, #field_name));
                json.push_str(&self.#field_ident.__to_str_depth(depth + 1)); // nested indentation
            }
        }
    });

    quote! {
        impl Serializer for #ident {
            fn to_str(&self) -> String {
                self.__to_str_depth(0)
            }
        }

        impl #ident {
            fn __to_str_depth(&self, depth: usize) -> String {
                let indent = "  ".repeat(depth + 1);
                let mut json = String::new();
                json.push_str("{\n");
                #(#field_serializations)*
                json.push_str(&format!("\n{}}}", "  ".repeat(depth)));
                json
            }
        }
    }
    .into()
}
