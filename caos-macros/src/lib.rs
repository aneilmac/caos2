use proc_macro::TokenStream;
use quote::{quote, quote_spanned, format_ident};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{parse_macro_input, Result, Token, Variant};

#[derive(Default)]
struct SyntaxToken {
    meta: Punctuated<syn::NestedMeta, Token![,]>,
}

impl SyntaxToken {
    fn name(&self) -> Option<&syn::LitStr> {
        self._str_for_tag("name")
    }

    fn custom_parser(&self) -> Option<&syn::LitStr> {
        self._str_for_tag("with_parser")
    }

    fn _str_for_tag(&self, tag: &str) -> Option<&syn::LitStr> {
        for m in self.meta.iter() {
            if let syn::NestedMeta::Meta(m) = m {
                if let syn::Meta::NameValue(m) = m {
                    if m.path.is_ident(tag) {
                        return match &m.lit {
                            syn::Lit::Str(s) => Some(s),
                            _ => panic!("Mut be a lit string"),
                        };
                    }
                }
            }
        }
        return None;
    }
}

impl Parse for SyntaxToken {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(SyntaxToken {
            meta: { Punctuated::parse_terminated(input)? },
        })
    }
}

#[proc_macro_derive(CommandList, attributes(syntax))]
pub fn command_list(input: TokenStream) -> TokenStream {
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

#[proc_macro_derive(CaosParsable, attributes(syntax))]
pub fn caos_parsable_derive_fn(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    if let syn::Data::Enum(ref content) = input.data {
        let marked_variants: Vec<_> = marked_variants(content).collect();

        let parse_combos = marked_variants
            .iter()
            .map(|(v, s)| parse_variant(v, s));

        let alt_statement = alt_chunk(parse_combos, 20);

        let name = &input.ident;
        let q = quote_spanned!(input.span() =>
            impl crate::parser::CaosParsable for #name  {
                fn parse_caos<'a>(input: &'a str) -> nom::IResult<&'a str, Self> {
                    use nom::bytes::complete::tag_no_case;
                    use nom::character::complete::space1;
                    use nom::branch::alt;
                    use nom::combinator::fail;

                    #alt_statement(input)
                }
            }
        );
        q.into()
    } else {
        panic!("This macro can only be used on enums");
    }
}

fn alt_chunk(
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

fn syntax_keyword(variant: &Variant, syntax: &SyntaxToken) -> String {
    syntax
        .name()
        .map(|c| c.value())
        .unwrap_or_else(|| variant.ident.to_string().to_lowercase())
}

fn parse_variant(variant: &Variant, syntax: &SyntaxToken) -> proc_macro2::TokenStream {
    if let Some(custom_parser) = syntax.custom_parser() {
        let custom_parser: syn::Ident = custom_parser.parse().expect("Must provide a valid function name");
        quote!(#custom_parser)
    } else 
    {
        parse_variant_default(variant, syntax)
    }
}

fn parse_variant_default(variant: &Variant, syntax: &SyntaxToken) -> proc_macro2::TokenStream {
    let parse_lines = variant.fields.iter().map(|f| parse_field(f));
    
    let field_names = variant.fields.iter().map(|f| &f.ident);
    
    let var_ident = &variant.ident;

    let keyword = syntax_keyword(variant, syntax);

    let construction = if variant.fields.is_empty() {
        quote!(Self::#var_ident)
    } else {
        quote!(Self::#var_ident { #(#field_names),* })
    };

    quote_spanned!(variant.span() =>
        |input: &'a str| -> nom::IResult<&str, Self> {
            let (input, _ ) = tag_no_case(#keyword)(input)?;

            #(#parse_lines)*

            Ok((input, #construction))
        }
    )
}

fn parse_field(field: &syn::Field) -> proc_macro2::TokenStream {
    let field_ident = &field.ident;
    let ty = &field.ty;
    quote_spanned!(field.span() =>
        let (input, _) = space1(input)?;
        let (input, #field_ident) = <#ty as crate::parser::CaosParsable>::parse_caos(input).map_err(|e| {
            // Any failures past this point are hard failures and we must abort.
            match e {
                nom::Err::Error(e) => nom::Err::Failure(e),
                h => h
            }
        })?;
    )
}

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
