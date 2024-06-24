use itertools::Itertools;
use std::collections::HashMap;
use syn::{Data, DeriveInput};

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(mixin))]
struct MixinFieldAttributes {
    #[deluxe(rest)]
    mixin: std::collections::HashMap<syn::Path, syn::Expr>,
}

fn extract_mixin_field_map(
    ast: &mut DeriveInput,
) -> deluxe::Result<HashMap<String, MixinFieldAttributes>> {
    let mut field_map = HashMap::new();
    if let Data::Struct(data_struct) = &mut ast.data {
        for field in data_struct.fields.iter_mut() {
            let result: Result<MixinFieldAttributes, _> = deluxe::extract_attributes(field);
            if let Ok(value) = result {
                if let Some(field_name) = &field.ident {
                    field_map.insert(field_name.to_string(), value);
                }
            }
        }
    }
    Ok(field_map)
}

fn derive_macro2(item: proc_macro2::TokenStream) -> deluxe::Result<proc_macro2::TokenStream> {
    let mut ast: DeriveInput = syn::parse2(item)?;
    let field_map: HashMap<String, MixinFieldAttributes> = extract_mixin_field_map(&mut ast)?;
    let mut method_tokens = quote::quote! {};

    for (source_property, attrs) in field_map.into_iter() {
        for (source_method, expr) in attrs.mixin.into_iter() {
            let property_definition = match &expr {
                syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(lit_str),
                    ..
                }) => lit_str.value(),
                _ => "".to_string(),
            };
            let source_method = source_method
                .get_ident()
                .ok_or_else(|| syn::Error::new_spanned(&expr, "Expected source method!"))?
                .to_string();
            let method_name: syn::Ident;
            let return_type: syn::Type;
            if property_definition.contains(':') {
                let (name, type_) = property_definition
                    .splitn(2, ':')
                    .into_iter()
                    .collect_tuple()
                    .ok_or_else(|| {
                        syn::Error::new_spanned(
                            &expr,
                            format!("Expected colon in property value '{}'", property_definition),
                        )
                    })?;
                method_name = syn::Ident::new(name.trim(), proc_macro2::Span::call_site());
                return_type = syn::parse_str(type_.trim()).expect("Failed to parse type");
            } else {
                method_name = syn::Ident::new(&source_method, proc_macro2::Span::call_site());
                return_type = syn::parse_str(&property_definition).expect("Failed to parse type");
            }
            let source_property_ident =
                syn::Ident::new(&source_property, proc_macro2::Span::call_site());
            let source_method_ident =
                syn::Ident::new(&source_method, proc_macro2::Span::call_site());
            method_tokens.extend(quote::quote! {
                pub fn #method_name(&self) -> #return_type {
                    self.#source_property_ident.#source_method_ident()
                }
            });
        }
    }

    let ident = &ast.ident;
    Ok(quote::quote! {
        impl #ident {
            #method_tokens
        }
    })
}

#[proc_macro_derive(Intermix, attributes(mixin))]
pub fn intermix_derive_macro(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_macro2(item.into())
        .unwrap_or_else(|e| {
            let error_message = format!("Macro error: {}", e);
            proc_macro2::TokenStream::from(quote::quote! {
                compile_error!(#error_message);
            })
        })
        .into()
}
