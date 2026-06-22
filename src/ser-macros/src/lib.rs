use core::panic;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields};

extern crate proc_macro;
#[proc_macro_derive(Serialize)]
pub fn serialize_struct(_item: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(_item as DeriveInput);
    let ident = ast.ident;

    let is_generic = !ast.generics.params.is_empty();

    let method_body = match &ast.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields_named) => {
                let fields = &fields_named.named;

                let field_serializations =  fields.iter().enumerate().map(|(i, field)| {
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



                    let comma = if i > 0 { ",\n" } else { "" };
                    
                    match seg_ident {
                        s if s == "String" || seg_ident.to_lowercase() == "char" => {
                            quote! {
                                json.push_str(#comma);
                                json.push_str(&format!("{}\"{}\": \"{}\"", indent, #field_name, self.#field_ident));
                            }
                        },
                        s if s.starts_with('i') || s.starts_with('u') || s.starts_with('f') => {
                            quote! {
                                json.push_str(#comma);
                                json.push_str(&format!("{}\"{}\": {}", indent, #field_name, self.#field_ident));
                             }
                        },
                        s if s == "Vec" || s == "HashSet" => {
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
                                            json.push_str(&format!("{}{}", item_indent, item.__to_str_depth(depth + 2)));
                                        }            
                                        json.push_str(&format!("\n{}]", indent));
                                    }
                                }
                        },
                        s if s == "HashMap" => {
                                quote! {
                                    json.push_str(#comma);
                                    let item_ident = "  ".repeat(depth + 2);
                                    let map = &self.#field_ident;
                                    json.push_str(&format!("{}\"{}\": {{", indent, #field_name));
                                    if map.is_empty() {
                                        json.push_str("}")
                                    } else {
                                        json.push_str("\n");
                                        for (idx, (k, v)) in map.iter().enumerate() {
                                            if idx > 0 { json.push_str(",\n"); }
                                            json.push_str(&format!("{}\"{}\": {}", item_ident, k, v));
                                        }
                                        json.push_str(&format!("\n{}}}", indent));
                                 }
                            }
                        },
                        _ => {
                            quote! {   
                                json.push_str(#comma);
                                json.push_str(&format!("{}\"{}\": ", indent, #field_name));
                                json.push_str(&self.#field_ident.__to_str_depth(depth + 1)); // nested indentation
                            }
                        }
                    }
                });

                quote! {
                    let indent = "  ".repeat(depth + 1);
                    let mut json = String::new();
                    json.push_str("{\n");
                    #(#field_serializations)*
                    json.push_str(&format!("\n{}}}", "  ".repeat(depth)));
                    json
                }
            }
            Fields::Unnamed(fields_unamed) => {
                let values = &fields_unamed.unnamed;
                if values.is_empty() {
                    panic!("Need fields to put in json");
                }
               let value_serializations = values.iter().enumerate().map(|(i, _)| {
                    let comma = if i > 0 { ",\n" } else { "" };
                    let idx = syn::Index::from(i);
                    quote! {
                        json.push_str(#comma);
                        let item_indent = "  ".repeat(depth + 2);
                        json.push_str(&format!("{}{}", item_indent, self.#idx.__to_str_depth(depth + 2)));
                    }
                });
                quote! {
                    let indent = "  ".repeat(depth + 1);
                    let mut json = String::new();
                    json.push_str("[\n");
                    #(#value_serializations)*
                    json.push_str(&format!("\n{}]", "  ".repeat(depth)));
                    json
                }
            }
            _ => panic!("Only named fields are supported"),
        },
        _ => panic!("Serialize derive only works on Structs"),
    };

        if !is_generic {
            quote! {
                impl Serializer for #ident {
                    fn to_str(&self) -> String {
                        self.__to_str_depth(0)
                    }

                    fn __to_str_depth(&self, depth: usize) -> String {
                        #method_body
                    }
                }
            }
            .into()
        } else {
            let type_params: Vec<&syn::Ident> = ast.generics.type_params().map(|p| &p.ident).collect();

            let generics_tokens = if !type_params.is_empty() {
                quote! { <#(#type_params),*> }
            } else {
                quote! {}
            };

            let where_clause = if !type_params.is_empty() {
                quote! { where #(#type_params: Serializer + Deserializer),* }
            } else {
                quote! {}
            };

            quote! {
                impl #generics_tokens Serializer for #ident #generics_tokens #where_clause {
                    fn to_str(&self) -> String {
                        self.__to_str_depth(0)
                    }

                    fn __to_str_depth(&self, depth: usize) -> String {
                        #method_body
                    }
                }
            }
            .into()
    }
}

