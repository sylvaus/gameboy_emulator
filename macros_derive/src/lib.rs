use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Ident};

#[proc_macro_derive(AddEnumName)]
pub fn add_enum_names(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;
    if let Data::Enum(enum_data) = &ast.data {
        let names: Vec<&Ident> = enum_data.variants.iter().map(|v| &v.ident).collect();
        let result = quote! {
            impl #name {
                pub fn get_name(&self) -> &'static str {
                    match self {
                        #(#name::#names => stringify!(#names)),*
                    }
                }
            }
        };

        TokenStream::from(result)
    } else {
        panic!("#[derive(AddEnumName)] is only defined for enums!");
    }
}
