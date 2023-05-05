use proc_macro2::TokenStream;
use quote::{
    quote,
    quote_spanned,
};
use syn::{
    spanned::Spanned,
    Data,
    DataStruct,
    Fields,
};

pub fn getters(attrs: TokenStream, s: synstructure::Structure) -> TokenStream {
    let trait_ident = attrs.clone();
    let struct_ident = s.ast().ident.clone();

    let item = match s.ast().data.clone() {
        Data::Struct(struct_item) => generate_struct(&s, struct_item),
        _ => panic!("Only structs are supported"),
    };

    let fields: Vec<_> = extract_fields(s.clone());

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

    let (impl_generics, type_generics, where_clause) = s.ast().generics.split_for_impl();

    (quote! {
        #item

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

fn generate_struct(s: &synstructure::Structure, struct_item: DataStruct) -> TokenStream {
    let struct_ident = s.ast().ident.clone();
    let vis = s.ast().vis.clone();
    let attrs = s.ast().attrs.clone();
    let types = s.ast().generics.clone();
    let (_, _, where_closure) = s.ast().generics.split_for_impl();

    let fields = struct_item
        .clone()
        .fields
        .into_iter()
        .map(|field| consume_getter_attrs(&field));

    match struct_item.fields {
        Fields::Unnamed(_) => {
            quote! {
                #(#attrs)*
                #vis struct #struct_ident #types #where_closure (
                    #(#fields),*
                );
            }
        }
        _ => {
            quote! {
                #(#attrs)*
                #vis struct #struct_ident #types #where_closure {
                    #(#fields),*
                }
            }
        }
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

fn extract_fields(s: synstructure::Structure) -> Vec<&syn::Field> {
    let struct_item = match s.ast().data.clone() {
        Data::Struct(struct_item) => struct_item,
        _ => panic!("Only structs are supported"),
    };

    struct_item
        .fields
        .iter()
        .filter(|field| field.attrs.iter().find(|a| a.path.is_ident("get")).is_some())
        .collect::<Vec<_>>()
        .clone()
}
