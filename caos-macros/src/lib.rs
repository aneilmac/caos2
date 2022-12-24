mod caos_parser;
///
/// Produces implementations for the `CommandList` and `CaosParsable` derivatives.
///
mod syntax_token;

use proc_macro::TokenStream;
use quote::quote_spanned;
use syn::spanned::Spanned;
use syn::{parse_macro_input, Variant};

use self::caos_parser::*;
use self::syntax_token::SyntaxToken;

/// Produces a const, static parameter on a type that derives from `CommandList` of the form:
///  `ALL_KEYWORDS : &[&str]`.
///
/// This is a hardcoded list of all known keywords/commands that have been marked with the
/// `#[syntax]` element in a type, and can be iterated against/tested in real-time.
///
/// For example:
///
/// ```ignore
/// #[derive(CommandList)]
/// struct Foo {
///   #[syntax]
///   Bari,
///   #[syntax(name="new: this")]
///   NewThis,
///   IgnoredThing,
/// }
/// ```
///
/// Would produce:
///
/// ```ignore
/// impl Foo {
///   pub const ALL_KEYWORDS: &'static [&'static str] = ["bari", "new this"];
/// }
/// ```
#[proc_macro_derive(CommandList, attributes(syntax))]
pub fn command_list_fn(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    if let syn::Data::Enum(ref content) = input.data {
        let marked_variants: Vec<_> = marked_variants(content).collect();
        let keywords = marked_variants.iter().map(|(v, s)| syntax_keyword(v, s));
        let name = &input.ident;
        let q = quote_spanned!(input.span() =>
            impl #name  {
                pub const ALL_KEYWORDS: &'static [&'static str] = &[#(#keywords),*];
            }
        );
        q.into()
    } else {
        panic!("This macro can only be used on enums");
    }
}

/// Produces an implementation of `CaosParsable` for an Enum which derives from `CaosParsable`.
///
/// Each variant marked with `#[syntax]` will have an implementation produced. The attribute
/// `name` can be used to overwrite the default keyword used, and the attribute `with_parser` can be used
/// to change the default parsing behaviour to something custom.
///
/// For example:
///
/// ```ignore
/// #[derive(CaosParsable)]
/// struct Foo {
///   #[syntax]
///   Bari,
///   #[syntax(name="new: this")]
///   NewThis,
///   #[syntax(with_parser="parse_value")]
///   Value(u32)
///   #[syntax]
///   Recu{recursive: Box<Foo>}
/// }
///
/// fn parse_value(input: &str) -> CaosParseResult<&str, Foo> {
///   map_res(digit1, |s: &str| s.parse::<u32>())(input)
/// }
/// ```
///
/// Would produce an implementation of `CaosParsable::parse_caos`, which could parse the following CAOS commands:
///
/// - `bari`
/// - `new: this`
/// - `recu bari`
/// - `19`
/// - `recu 19`
/// - and so on...
///
#[proc_macro_derive(CaosParsable, attributes(syntax))]
pub fn caos_parsable_derive_fn(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    if let syn::Data::Enum(ref content) = input.data {
        let marked_variants: Vec<_> = marked_variants(content).collect();

        let parse_combos = marked_variants.iter().map(|(v, s)| parse_variant(v, s));

        let alt_statement = alt_chunk(parse_combos, 20);

        let name = &input.ident;
        let q = quote_spanned!(input.span() =>
            impl crate::parser::CaosParsable for #name  {
                fn parse_caos<'a>(input: &'a str) -> CaosParseResult<&'a str, Self> {
                    use crate::parser::caos_skippable1;
                    use nom::bytes::complete::tag_no_case;
                    use nom::branch::alt;
                    use nom::combinator::{fail, cut};

                    #alt_statement(input)
                }
            }
        );
        q.into()
    } else {
        panic!("This macro can only be used on enums");
    }
}

/// For each SyntaxToken, produces it's keyword, if none have been provided, we
//. default to the lowercase version of the variant identifier.
fn syntax_keyword(variant: &Variant, syntax: &SyntaxToken) -> String {
    syntax
        .name()
        .map(|c| c.value())
        .unwrap_or_else(|| variant.ident.to_string().to_lowercase())
}

/// Discover all variants in an enum marked with the `#[syntax]` attribute. It is possible to have
/// multiple syntax tokens per field but this is not used in practice.
fn marked_variants(enum_content: &syn::DataEnum) -> impl Iterator<Item = (&Variant, SyntaxToken)> {
    return enum_content
        .variants
        .iter()
        .map(|v| v.attrs.iter().map(move |a| (v, a)))
        .flatten()
        .filter_map(|(v, a)| {
            if a.path.is_ident("syntax") {
                Some(if a.tokens.is_empty() {
                    (v, Default::default())
                } else {
                    a.parse_args::<SyntaxToken>().map(|a| (v, a)).unwrap()
                })
            } else {
                None
            }
        });
}
