extern crate proc_macro;

use proc_macro::TokenStream;

use quote::{format_ident, quote};
use syn::{DeriveInput, Field, FieldsNamed, Ident, parse_macro_input, Token};
use syn::__private::TokenStream2;
use syn::punctuated::Punctuated;
use syn::token::Comma;

#[proc_macro_derive(WithDeps)]
pub fn with_depth(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    let name = &input.ident;
    let fields = extract_fields(&input);

    let field_names_for_constructor = fields.iter().map(|f| &f.ident).collect::<Vec<_>>();
    let new_arguments = get_new_arguments(fields);
    let methods = get_methods(fields);

    let result = quote! {
        impl #name {
            pub fn new(#(#new_arguments),*) -> Self {
                Self {
                    #(#field_names_for_constructor),*
                }
            }
            #(#methods)*
        };
    };
    result.into()
}

fn get_methods(fields: &Punctuated<Field, Comma>) -> Vec<TokenStream2> {
    let methods = fields
        .iter()
        .map(|f| {
            let ident = &f.ident.clone().unwrap();
            let method_name = format_ident!("set_{}",ident);
            let ty = &f.ty;
            return quote! {
                pub fn #method_name(&mut self,#ident:#ty){
                    self.#ident = #ident;
                }
            };
        })
        .collect::<Vec<_>>();
    methods
}

fn get_new_arguments(fields: &Punctuated<Field, Comma>) -> Vec<TokenStream2> {
    let new_arguments = fields
        .iter()
        .map(|f| {
            let ident = &f.ident;
            let ty = &f.ty;
            quote! {
                #ident: #ty
            }
        })
        .collect::<Vec<_>>();
    new_arguments
}

fn extract_fields(input: &DeriveInput) -> &Punctuated<Field, Token![,]> {
    let fields = if let syn::Data::Struct(syn::DataStruct {
                                              fields: syn::Fields::Named(FieldsNamed { named: fields, .. }),
                                              ..
                                          }) = &input.data
    {
        fields
    } else {
        panic!("#[derive(WithDeps)] can only be used on structs with named fields");
    };
    fields
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
