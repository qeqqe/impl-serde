use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields};

extern crate proc_macro;

#[proc_macro_derive(Deserialize)]
pub fn deser(_input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(_input as DeriveInput);
    let ident = &ast.ident;
    let visitor_ident = format_ident!("__{}Visitor", ident);
    let ident_name = ident.to_string();

    let type_params: Vec<&syn::Ident> = ast.generics.type_params().map(|p| &p.ident).collect();
    let is_generic = !type_params.is_empty();

    let generics_tokens = if is_generic {
        quote! { <#(#type_params),*> }
    } else {
        quote! {}
    };

    let where_clause = if is_generic {
        quote! { where #(#type_params: Serializer + Deserializer),* }
    } else {
        quote! {}
    };

    let (visitor_def, visitor_ctor) = if is_generic {
        (
            quote! {
                struct #visitor_ident #generics_tokens(
                    std::marker::PhantomData<(#(#type_params,)*)>
                );
            },
            quote! { #visitor_ident(std::marker::PhantomData) },
        )
    } else {
        (quote! { struct #visitor_ident; }, quote! { #visitor_ident })
    };

    let output = match &ast.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            // struct UselessFuckingStruct { x: T, y: U } -> visit_map (cus top-level of json is a map)
            Fields::Named(fields_named) => {
                let field_extractions = fields_named.named.iter().map(|field| {
                    let field_ident = field.ident.as_ref().unwrap();
                    let field_name = field_ident.to_string();
                    let field_ty = &field.ty;
                    quote! {
                        #field_ident: <#field_ty as Deserializer>::deserialize(
                            map.get(#field_name)
                               .unwrap_or_else(|| panic!("missing field: {}", #field_name))
                        ),
                    }
                });

                quote! {
                    #visitor_def

                    impl #generics_tokens Visitor for #visitor_ident #generics_tokens #where_clause {
                        type Output = #ident #generics_tokens;

                        fn visit_map(self, map: &std::collections::HashMap<String, JsonValue>) -> Self::Output {
                            #ident { #(#field_extractions)* }
                        }
                        fn visit_seq(self, _s: &[JsonValue]) -> Self::Output {
                            panic!("expected item for {}, got array", #ident_name)
                        }
                    }

                    impl #generics_tokens Deserializer for #ident #generics_tokens #where_clause {
                        fn deserialize(v: &JsonValue) -> Self {
                            match v {
                                JsonValue::Object(map) => #visitor_ctor.visit_map(map),
                                _ => panic!("expected item for {}", #ident_name),
                            }
                        }
                    }
                }
            }
            // struct UselessFuckingUnnamedFieldStruct(T, U) -> visit_seq
            Fields::Unnamed(fields_unnamed) => {
                let field_extractions =
                    fields_unnamed.unnamed.iter().enumerate().map(|(i, field)| {
                        let field_ty = &field.ty;
                        quote! {
                            <#field_ty as Deserializer>::deserialize(
                                seq.get(#i)
                                   .unwrap_or_else(|| panic!("missing index {} in tuple", #i))
                            ),
                        }
                    });

                quote! {
                    #visitor_def

                    impl #generics_tokens Visitor for #visitor_ident #generics_tokens #where_clause {
                        type Output = #ident #generics_tokens;

                        fn visit_seq(self, seq: &[JsonValue]) -> Self::Output {
                            #ident(#(#field_extractions)*)
                        }
                        fn visit_map(self, _m: &std::collections::HashMap<String, JsonValue>) -> Self::Output {
                            panic!("expected array for {}, got object", #ident_name)
                        }
                    }

                    impl #generics_tokens Deserializer for #ident #generics_tokens #where_clause {
                        fn deserialize(v: &JsonValue) -> Self {
                            match v {
                                JsonValue::Array(seq) => #visitor_ctor.visit_seq(seq),
                                _ => panic!("expected array for {}", #ident_name),
                            }
                        }
                    }
                }
            }

            Fields::Unit => panic!("unit structs not supported"),
        },
        _ => panic!("deserialize derive only works on structs"),
    };

    output.into()
}
