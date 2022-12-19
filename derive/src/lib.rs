use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

use proc_macro::{self, TokenStream};

fn match_started(fields: &Fields) -> TokenStream2 {
    match fields {
        Fields::Named(fields) => fields
            .named
            .first()
            .map(|first| first.ident.as_ref().unwrap())
            .map(|first| quote!({ #first, .. } => #first.start())),
        Fields::Unnamed(fields) => fields.unnamed.first().map(|_| {
            quote! {
                (x, ..) => x.start()
            }
        }),
        Fields::Unit => None,
    }
    .expect("at least one field expected")
}

fn match_ended(fields: &Fields) -> TokenStream2 {
    match fields {
        Fields::Named(fields) => fields
            .named
            .last()
            .map(|last| last.ident.as_ref().unwrap())
            .map(|last| quote!({ #last, .. } => #last.end())),
        Fields::Unnamed(fields) => fields.unnamed.last().map(|_| {
            quote! {
                (.., x) => x.end()
            }
        }),
        Fields::Unit => None,
    }
    .expect("at least one field expected")
}

fn derive_fields<F>(
    input: TokenStream,
    match_fields: F,
    trait_name: TokenStream2,
    meth_name: TokenStream2,
) -> TokenStream
where
    F: Fn(&Fields) -> TokenStream2,
{
    let DeriveInput {
        ident,
        generics,
        data,
        ..
    } = parse_macro_input!(input);
    let (impl_generics, ty_generics, where_generics) =
        generics.split_for_impl();
    let inner = match data {
        Data::Struct(s) => {
            let match_fields = match_fields(&s.fields);
            quote! { match self { Self #match_fields, } }
        }
        Data::Enum(e) => {
            let branch = e.variants.iter().map(|variant| {
                let ident = &variant.ident;
                let fields = match_fields(&variant.fields);
                quote! { Self::#ident #fields, }
            });
            quote! { match self { #(#branch)* } }
        }
        Data::Union(_) => panic!("union not supported"),
    };
    quote! {
        impl #impl_generics #trait_name for #ident #ty_generics #where_generics
        {
           #[inline]
           fn #meth_name(&self) -> usize {
                #inner
            }
        }
    }
    .into()
}

#[proc_macro_derive(Started)]
pub fn derive_started(input: TokenStream) -> TokenStream {
    derive_fields(
        input,
        match_started,
        quote!(cys_span::Started),
        quote!(start),
    )
}

#[proc_macro_derive(Ended)]
pub fn derive_ended(input: TokenStream) -> TokenStream {
    derive_fields(input, match_ended, quote!(cys_span::Ended), quote!(end))
}
