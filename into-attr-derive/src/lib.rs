extern crate dot_structures;
extern crate proc_macro;

use dot_generator::attr;
use into_attr::IntoAttribute;
use proc_macro::TokenStream;
use quote::quote;
use syn::{self, Data};

#[proc_macro_derive(IntoAttribute)]
pub fn into_attr_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_into_attr_macro(&ast)
}

fn impl_into_attr_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let name_str = name.to_string();
    let gen = match &ast.data {
        Data::Enum(de) => {
            quote! {
              impl IntoAttribute for #name {
                fn into_attr(self)  -> Attribute {
                        let v = format!("{:?}",self);
                        let v =  v.as_str().strip_suffix("_").unwrap_or(v.as_str());
                        attr!(#name_str,v)
                }
              }
            }
        }
        Data::Struct(ds) => {
            quote! {
              impl IntoAttribute for #name {
                fn into_attr(self) -> Attribute {
                 attr!(#name_str,self.0)
                }
              }
            }
        }
        _ => panic!("the unions are unexpected"),
    };

    gen.into()
}
