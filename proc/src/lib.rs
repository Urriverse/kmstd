use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr, Item, Meta, MetaNameValue};
use syn::punctuated::Punctuated;
use syn::Token;

/// # API Item Status
/// 
/// Use this macro to set what status your API item is.
/// 
/// In Kstd project, there are some well-known:
/// 
/// | Status | Comment |
/// | --- | --- |
/// | `deprecated` | API item has better alternatives and will be removed in future |
/// | `stable` | API item is ready to production and will not change breakly for a long time |
/// | `unstable` | API item is almost ready to production but can change breakly in near future |
/// | `experimental` | API item is a brand-new beast and you need to get feedback |
/// | `incomplete` | You just started development of this API item and don't know how it will look in future |
#[proc_macro_attribute]
pub fn status(args: TokenStream, input: TokenStream) -> TokenStream {
    let input_item = parse_macro_input!(input as Item);
    
    let args: Punctuated<Meta, Token![,]> = parse_macro_input!(args with Punctuated::parse_terminated);
    
    let mut generated_attrs: Vec<proc_macro2::TokenStream> = Vec::new();
    
    for meta in args {
        match meta {
            // #[status(feature_name)]
            Meta::Path(path) => {
                if let Some(ident) = path.get_ident() {
                    let feature_name = ident.to_string();
                    generated_attrs.push(quote! {
                        #[cfg(feature = #feature_name)]
                        #[doc(cfg(feature = #feature_name))]
                    });
                }
            }
            // #[status(deprecated = "reason")]
            Meta::NameValue(MetaNameValue { path, value, .. }) => {
                if let Some(ident) = path.get_ident() {
                    if ident == "deprecated" {
                        if let Expr::Lit(expr_lit) = &value {
                            if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                                let reason_str = lit_str.value();
                                generated_attrs.push(quote! {
                                    #[deprecated(note = #reason_str)]
                                });
                            }
                        }
                    }
                }
            }
            _ => { }
        }
    }
    
    let expanded = quote! {
        #(#generated_attrs)*
        #input_item
    };
    
    TokenStream::from(expanded)
}
