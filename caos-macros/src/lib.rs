//!
//!  Produces implementations for the `CommandList` and `CaosParsable` derivatives.
//!

mod caos_evaluator;
mod caos_parser;
mod syntax_token;

use proc_macro::TokenStream;
use quote::quote_spanned;
use syn::spanned::Spanned;
use syn::{parse_macro_input, Variant};

use self::caos_parser::*;
use self::syntax_token::SyntaxToken;

#[proc_macro_derive(EvaluateCommand, attributes(return_type, syntax))]
pub fn caos_evaluate_derive_fn(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    let ret_type;
    let ret_types: Vec<_> = input
        .attrs
        .iter()
        .filter(|a| a.path.is_ident("return_type"))
        .collect();

    if ret_types.len() != 1 {
        panic!("Must have exactly 1 `#[ret_type]");
    } else {
        ret_type = ret_types
            .first()
            .unwrap()
            .parse_args::<syn::Type>()
            .expect("Good path");
    }

    if let syn::Data::Enum(ref content) = input.data {
        let marked_variants: Vec<_> = marked_variants(content, "syntax").collect();
        let evaluators = marked_variants
            .iter()
            .map(|(v, s)| match s.custom_evaluator() {
                Some(p) => {
                    let custom_evaluator: syn::Ident = p.parse().expect("Expected valid function");
                    caos_evaluator::to_match_expression(v, custom_evaluator)
                }
                None => caos_evaluator::to_match_todo_expression(v),
            });
        let name = &input.ident;
        let q = quote_spanned!(input.span() =>
            impl<'a> crate::engine::EvaluateCommand for #name  {
                type ReturnType = #ret_type;
                fn evaluate(&self, script: &mut crate::engine::ScriptRefMut<'_>) -> crate::Result<Self::ReturnType> {
                    match self {
                        #(#evaluators)*
                    }
                }
            }
        );
        q.into()
    } else {
        panic!("This macro can only be used on enums");
    }
}

/// Produces a const, static parameter on a type that derives from `CommandList` of the form:
///  `ALL_KEYWORDS : &[&str]`.
///
/// This is a hardcoded list of all known keywords/commands that have been marked with the
/// `#[syntax]` element in a type and *do not* have a custom parser associated.
///
/// These can be iterated against/tested in real-time.
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
        let marked_variants: Vec<_> = marked_variants(content, "syntax").collect();
        let keywords = marked_variants.iter().filter_map(|(v, s)| {
            if s.custom_parser().is_none() {
                Some(syntax_keyword(v, s))
            } else {
                None
            }
        });
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
        let marked_variants: Vec<_> = marked_variants(content, "syntax").collect();

        let parse_combos = marked_variants.iter().map(|(v, s)| {
            let vname = v.ident.to_string();
            let parser = parse_variant(v, s);
            quote_spanned!(v.span()=> context(#vname, #parser))
        });

        let alt_statement = alt_chunk(parse_combos, 20);

        let name = &input.ident;
        let name_as_str = name.to_string();

        let q = quote_spanned!(input.span() =>
            impl crate::parser::CaosParsable for #name  {
                fn parse_caos<'a>(input: &'a str) -> CaosParseResult<&'a str, Self> {
                    use crate::parser::caos_skippable1;
                    use nom::bytes::complete::tag_no_case;
                    use nom::combinator::{cut};
                    use nom::error::context;
                    use nom::error::{VerboseError, ErrorKind, ParseError};
                    context(#name_as_str, #alt_statement)(input)
                }
            }
        );
        q.into()
    } else {
        panic!("This macro can only be used on enums");
    }
}

/// For each SyntaxToken, produces it's keyword, if none have been provided, we
/// default to the lowercase version of the variant identifier.
fn syntax_keyword(variant: &Variant, syntax: &SyntaxToken) -> String {
    syntax
        .name()
        .map(|c| c.value())
        .unwrap_or_else(|| variant.ident.to_string().to_lowercase())
}

/// Discover all variants in an enum marked with the `#[syntax]` attribute. It is possible to have
/// multiple syntax tokens per field but this is not used in practice.
fn marked_variants<'a, 'b: 'a>(
    enum_content: &'a syn::DataEnum,
    tag: &'b str,
) -> impl Iterator<Item = (&'a Variant, SyntaxToken)> {
    return enum_content
        .variants
        .iter()
        .map(move |v| v.attrs.iter().map(move |a| (v, a)))
        .flatten()
        .filter_map(move |(v, a)| {
            if a.path.is_ident(tag) {
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
