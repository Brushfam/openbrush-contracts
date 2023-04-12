use proc_macro2::TokenStream;
use quote::{
    quote,
    quote_spanned,
    ToTokens,
};
use syn::{
    parse2,
    spanned::Spanned,
};

pub fn getters(item: TokenStream) -> TokenStream {
    let struct_item: syn::DeriveInput = parse2(item.clone()).expect("Expected DeriveInput");

    let trait_ident = attrs.clone();
    let struct_ident = struct_item.ident.clone();

    let data_struct = match struct_item.data.clone() {
        syn::Data::Struct(s) => s,
        _ => panic!("Expected struct"),
    };

    let fields: Vec<_> = extract_fields(&data_struct);

    let trait_messages = fields.iter().map(|field| {
        let field_ident = field.ident.clone().unwrap();
        let field_type = field.ty.clone();
        let span = field.span();

        quote_spanned! {span =>
            #[ink(message)]
            fn #field_ident(&self) -> #field_type;
        }
    });

    let impls = fields.iter().map(|field| {
        let field_ident = field.ident.clone().unwrap();
        let field_type = field.ty.clone();
        let span = field.span();

        quote_spanned! {span =>
            default fn #field_ident(&self) -> #field_type {
                self.data().#field_ident
            }
        }
    });

    (quote! {
        #struct_item

        #[openbrush::trait_definition]
        pub trait Getters {
            #(#trait_messages)*
        }

        impl<T: Storage<#struct_ident>> Getters for T {
            #(#impls)*
        }
    })
    .into()
}

fn extract_fields(s: &syn::DataStruct) -> Vec<&syn::Field> {
    s.fields
        .iter()
        .filter(|field| field.attrs.iter().find(|a| a.path.is_ident("get")).is_some())
        .collect::<Vec<_>>()
        .clone()
}
