use crate::{syntax_keyword, syntax_token::SyntaxToken};
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, Variant};

/// Converts a variant into a parser implementation.
pub fn parse_variant(variant: &Variant, syntax: &SyntaxToken) -> proc_macro2::TokenStream {
    if let Some(custom_parser) = syntax.custom_parser() {
        let custom_parser: syn::Ident = custom_parser
            .parse()
            .expect("Must provide a valid function name");
        quote_spanned!(variant.span()=>#custom_parser)
    } else {
        parse_variant_default(variant, syntax)
    }
}

/// Workaround for nom's `alt` method. `alt` has a limit of 21 tuple entries -- this function,
/// splits a list of parser functions into groups of 20 which are nested on top of one another
/// to avoid this limit. Order is not preserved.
pub fn alt_chunk(
    inputs: impl Iterator<Item = proc_macro2::TokenStream>,
    chunk_size: usize,
) -> proc_macro2::TokenStream {
    let mut v = Vec::<proc_macro2::TokenStream>::with_capacity(chunk_size);
    let mut final_token = quote!(fail);
    for parse_token in inputs {
        if v.len() < chunk_size {
            v.push(parse_token)
        } else {
            final_token = quote!(alt((#(#v),* , #final_token)));
            v.clear();
            v.push(parse_token);
        }
    }

    if !v.is_empty() {
        final_token = quote!(alt((#(#v),* , #final_token)));
    }
    return final_token;
}

/// If `with_parser` is not specified this method produces the default parser for a CAOS command.
/// It attempts to parse keyword name: either the variant name or something custom passed to the
/// syntax token, then attempts to parse each field of the variant left-to-right in order of
/// declaration.
fn parse_variant_default(variant: &Variant, syntax: &SyntaxToken) -> proc_macro2::TokenStream {
    let parse_lines = variant.fields.iter().map(|f| parse_field(f));

    let field_names = variant.fields.iter().map(|f| &f.ident);

    let var_ident = &variant.ident;

    // Break down a keyword with spaces such that it can have any number
    // of spaces. That is "NEW: CREA" and "NEW:   CREA" are equivalent.
    let keywords = syntax_keyword(variant, syntax);
    let keywords = keywords
        .split_whitespace()
        .map(|k| {
            std::iter::once(
                quote_spanned!(variant.span()=> let (input, _) = tag_no_case(#k)(input)?;),
            )
            .chain(std::iter::once(
                quote_spanned!(variant.span()=> let (input, _) = caos_skippable1(input)?;),
            ))
        })
        .flatten();
    let keywords: Vec<_> = keywords.collect();
    // This would be simplified through use of intersperse but that is still in Nightly.
    // Remove the last element of the list to get equivalent behaviour.
    let keywords = keywords.iter().take(keywords.len() - 1);

    let construction = if variant.fields.is_empty() {
        quote!(Self::#var_ident)
    } else {
        quote!(Self::#var_ident { #(#field_names),* })
    };

    quote_spanned!(variant.span() =>
        |input: &'a str| -> CaosParseResult<&str, Self> {
            #(#keywords)*

            #(#parse_lines)*

            Ok((input, #construction))
        }
    )
}

/// Attempts to parse a variant field. Each field is expected to be a type that can itself be parsed as
/// some sort of CAOS expression. Note that all failures are hard failures at this point in the parsing
/// process. That is: not being able to parse a keyword is acceptable, but being able to parse a keyword and
/// not the arguments is not a recoverable error.
fn parse_field(field: &syn::Field) -> proc_macro2::TokenStream {
    let field_ident = &field.ident;
    let field_ident_as_str = field_ident
        .as_ref()
        .map(|i| i.to_string())
        .unwrap_or(String::new());
    let ty = &field.ty;
    quote_spanned!(field.span() =>
        let (input, _) = caos_skippable1(input)?;
        let (input, #field_ident) = context(#field_ident_as_str, cut(<#ty as crate::parser::CaosParsable>::parse_caos))(input)?;
    )
}
