use darling::{ast, FromDeriveInput, FromVariant};
use proc_macro::TokenStream;
use quote::quote_spanned;
use syn::parse_macro_input;

#[derive(Debug, FromDeriveInput)]
#[darling(supports(enum_any))]
struct Expression {
    ident: syn::Ident,
    data: ast::Data<ExpressionVariant, ()>,
}

#[derive(Debug, FromVariant)]
#[darling(attributes(parse))]
struct ExpressionVariant {
    ident: syn::Ident,
    ignore: Option<()>,
    rule: Option<syn::Path>,
    fields: darling::ast::Fields<syn::Field>,
}

#[proc_macro_derive(ExpressionParser, attributes(parse))]
pub fn expression_parser(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as syn::DeriveInput);
    let expression = Expression::from_derive_input(&derive_input).unwrap();

    if let Some(v) = expression.data.take_enum() {
        let marked_variants : Vec<_> = v.iter().filter_map(|v| {
            let name = &v.ident;
            let field_count = v.fields.len();
            let rule = &v.rule;
            match (v.ignore,rule) {
                (None, None) => panic!("Expected ignore or rule"),
                (Some(_), Some(_)) => panic!("Cannot have ignore and rule"),
                (Some(_), None) => None,
                (None, Some(rule)) => Some({
                    if field_count == 0 {
                        quote_spanned!(name.span()=>
                            #rule => Some(crate::parser::ExpressionThunk::Completed(crate::ast::Anything::from(Self::#name)))
                        )
                    } else {
                        let fields = v.fields.iter().map(|field| {
                            let fname = &field.ident;
                            quote_spanned!(name.span()=>
                                #fname: Box::new(arg_it.next().unwrap().try_into()?)
                            )
                        });
                        quote_spanned!(name.span()=>
                            #rule => Some(crate::parser::ExpressionThunk::Partial {
                                origin: pair,
                                arg_parts: Vec::<crate::ast::Anything>::with_capacity(#field_count),
                                target_args: #field_count,
                                complete_method: Box::new(|pair, args| -> std::result::Result<crate::ast::Anything, crate::CaosError> {
                                    if args.len() == #field_count {
                                        let mut arg_it = args.into_iter();
                                        Ok(crate::ast::Anything::from(Self::#name { #(#fields),* }))
                                    } else {
                                        Err(crate::CaosError::new_arg_count_error(#field_count, args.len(), pair.line_col()))
                                    }
                                })
                        }))
                    }
            }),
            }
        }).collect();

        let name = &expression.ident;
        quote_spanned!(name.span()=>
            impl crate::parser::ExpressionParser for #name {
            fn parse_thunk<'i>(pair : pest::iterators::Pair<'i, crate::Rule>) -> Option<crate::parser::ExpressionThunk<'i>> {
                match pair.as_rule() {
                    #(#marked_variants),*,
                    _ => None
                }
            }
            }).into()
    } else {
        TokenStream::default()
    }
}
