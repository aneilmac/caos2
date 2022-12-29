use proc_macro2::TokenStream;
use quote::{quote_spanned, format_ident};
use syn::spanned::Spanned;

pub fn to_match_expression(variant: &syn::Variant, evaluate_call: syn::Ident) -> TokenStream {
    let name = &variant.ident;
    match variant.fields {
        syn::Fields::Named(ref s) => {
            let s: Vec<_> = s.named.iter().map(|i| &i.ident).collect();
            quote_spanned!(variant.span()=> Self::#name{#(#s),*} => #evaluate_call(#(<#s.evaluate(script)),*) )
        }
        syn::Fields::Unnamed(ref s) => {
            let s: Vec<_> = s.unnamed.iter().enumerate().map(|(i, _)| format_ident!("arg{}", i) ).collect();
            quote_spanned!(variant.span()=> Self::#name(#(#s),*) => #evaluate_call(#(<#s.evaluate(script)?),*) )
        }
        syn::Fields::Unit => quote_spanned!(variant.span()=> Self::#name => #evaluate_call() ),
    }
}

pub fn to_match_todo_expression(variant: &syn::Variant) -> TokenStream {
    let name = &variant.ident;
    let todo_str = format!("Evaluator for `{}`", name);
    match variant.fields {
        syn::Fields::Named(..) => {
            quote_spanned!(variant.span()=> Self::#name{..} => todo!(#todo_str))
        }
        syn::Fields::Unnamed(..) => {
            quote_spanned!(variant.span()=> Self::#name(..) => todo!(#todo_str))
        }
        syn::Fields::Unit => quote_spanned!(variant.span()=> Self::#name => todo!(#todo_str)),
    }
}