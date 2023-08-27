use proc_macro::TokenStream;
use proc_macro2;
use quote::{quote, ToTokens};
use syn;

#[proc_macro_derive(AddEnumName)]
pub fn add_enum_names(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);

    let name = &ast.ident;
    if let syn::Data::Enum(enum_data) = &ast.data {
        let names: Vec<&syn::Ident> = enum_data.variants.iter().map(|v| &v.ident).collect();
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

#[proc_macro_derive(BitAccessor, attributes(bit_offset_size))]
pub fn bit_accessor(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);

    if let syn::Data::Struct(data_struct) = &ast.data {
        let struct_name = &ast.ident;
        let accessors: Vec<proc_macro2::TokenStream> = data_struct
            .fields
            .iter()
            .flat_map(|field| get_bit_infos(field))
            .collect();

        let result = quote! {
            impl #struct_name {
                #(#accessors)*
            }
        };

        TokenStream::from(result)
    } else {
        panic!("Bit Accessor should only be applied on struct");
    }
}

fn get_bit_infos(field: &syn::Field) -> Vec<proc_macro2::TokenStream> {
    if field.ident.is_none() {
        return Vec::new();
    }
    let name = field.ident.clone().unwrap();
    field
        .attrs
        .iter()
        .filter(|attribute| attribute.path().is_ident("bit_offset_size"))
        .map(|attribute| get_bit_info(&name, attribute))
        .collect()
}

fn get_bit_info(value_name: &syn::Ident, attribute: &syn::Attribute) -> proc_macro2::TokenStream {
    if let syn::Meta::List(arguments) = &attribute.meta {
        let BitOffsetSize {
            bit_name,
            offset,
            size,
        } = syn::parse2(arguments.tokens.to_token_stream()).unwrap();

        let read_bit_name = syn::Ident::new(&format!("read_{}", bit_name), bit_name.span());
        let write_bit_name = syn::Ident::new(&format!("write_{}", bit_name), bit_name.span());

        quote! {
            pub fn #read_bit_name(&self) -> u8 {
                (self.#value_name >> #offset) & ((1 << #size) - 1)
            }
            pub fn #write_bit_name(&mut self, value: u8) {
                let inv_mask: u8 = 0xFF - (((1 << #size) - 1) << #offset);
                self.#value_name &= inv_mask;
                self.#value_name += value << #offset;
            }
        }
    } else {
        panic!("{}", error_with_help("invalid format"));
    }
}

struct BitOffsetSize {
    bit_name: syn::Ident,
    offset: syn::Lit,
    size: syn::Lit,
}

impl syn::parse::Parse for BitOffsetSize {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let bit_name: syn::Ident = input.parse()?;
        let _: proc_macro2::Punct = input.parse()?; // process the comma
        let offset: syn::Lit = input.parse()?;
        let _: proc_macro2::Punct = input.parse()?; // process the comma
        let size: syn::Lit = input.parse()?;

        Ok(BitOffsetSize {
            bit_name,
            offset,
            size,
        })
    }
}

fn error_with_help(error: &str) -> String {
    format!(
        "bit_accessor_offset_size failed to be applied: {}\n\
        Usage should be bit_accessor_offset_size(bit_name, offset, bit_size)",
        error
    )
}
