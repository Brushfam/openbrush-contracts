use heck::SnakeCase;
use proc_macro2::TokenStream;
use quote::{
    format_ident,
    quote,
    ToTokens,
};
use syn::{
    parse2,
    ItemTrait,
};

pub fn generate(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let original_trait = parse2::<syn::Ident>(attrs).expect("Wrong trait naming used");
    let trait_item: ItemTrait = parse2(input).unwrap();

    let trait_name = trait_item.ident.clone().to_token_stream();

    let macro_name = format!("impl_{}", original_trait.to_string().to_snake_case()).to_token_stream();

    let trait_impls = generate_trait_impl(original_trait.clone().to_token_stream(), trait_item.clone());

    quote! {
        #[macro_export]
        macro_rules! #macro_name {
            ($contract:ident) => {
                impl #trait_name for $contract {}

                impl #original_trait for $contract {
                    #(#trait_impls),*
                }
            };
        }
    }
}

pub fn generate_trait_impl(trait_name: TokenStream, trait_ts: ItemTrait) -> Vec<TokenStream> {
    let mut impl_messages = vec![];

    trait_ts
        .items
        .iter()
        .filter_map(|item| {
            if let syn::TraitItem::Method(method) = item {
                Some(method)
            } else {
                None
            }
        })
        .for_each(|method| {
            let output_ty = match method.sig.output.clone() {
                syn::ReturnType::Default => quote! { () },
                syn::ReturnType::Type(_, return_type) => quote! { #return_type },
            };
            let input_bindings = method
                .sig
                .inputs
                .iter()
                .filter_map(|input| {
                    if let syn::FnArg::Typed(pat_typed) = input {
                        Some(pat_typed)
                    } else {
                        None
                    }
                })
                .enumerate()
                .map(|(n, _)| format_ident!("__openbrush_binding_{}", n))
                .collect::<Vec<_>>();
            let input_types = method
                .sig
                .inputs
                .iter()
                .filter_map(|input| {
                    if let syn::FnArg::Typed(pat_typed) = input {
                        Some(pat_typed)
                    } else {
                        None
                    }
                })
                .map(|pat_type| pat_type.ty.clone())
                .collect::<Vec<_>>();
            let method_name = method.sig.ident.clone();
            impl_messages.push(quote! {
                #[ink(message)]
                fn #method_name(&mut self, #(#input_bindings: #input_types),*) -> #output_ty {
                    #trait_name::#method_name(self, #(#input_bindings),*)
                }
            })
        });
    impl_messages
}
