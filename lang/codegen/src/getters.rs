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

pub fn getters(attrs: TokenStream, item: TokenStream) -> TokenStream {
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
                self.#field_ident
            }
        }
    });

    let (impl_generics, type_generics, where_clause) = struct_item.generics.split_for_impl();

    (quote! {
        #struct_item

        #[openbrush::trait_definition]
        pub trait #trait_ident {
            #(#trait_messages)*
        }

        impl #impl_generics #trait_ident for #struct_ident #type_generics #where_clause {
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

fn generate_item(s: &syn::DeriveInput) -> TokenStream {
    let struct_ident = s.ident.clone();
    let vis = s.vis.clone();
    let attrs = s.attrs.clone();
    let types = s.generics.clone();

    let (_, _, where_closure) = s.ast().generics.split_for_impl();

    let fields = s
        .data
        .clone()
        .into_fields()
        .into_iter()
        .enumerate()
        .map(|field| extract_fields(field.clone()));

    match s.data {
        syn::Data::Struct(ref struct_item) => {
            match struct_item.fields {
                syn::Fields::Unnamed(_) => {
                    quote! {
                        #(#attrs)*
                        #vis struct #struct_ident #types #where_closure(
                            #(#fields),*
                        );
                    }
                }
                _ => {
                    quote! {
                        #(#attrs)*
                        #vis struct #struct_ident #types #where_closure{
                            #(#fields),*
                        }
                    }
                }
            }
        }
        syn::Data::Enum(_) => panic!("Enums are not supported"),
        syn::Data::Union(_) => panic!("Unions are not supported"),
    }
}

fn consume_getter_attrs(field: &syn::Field) -> Vec<syn::Attribute> {
    field
        .attrs
        .iter()
        .filter(|a| !a.path.is_ident("get"))
        .cloned()
        .collect()
}
