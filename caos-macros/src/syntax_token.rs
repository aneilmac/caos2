use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Result, Token};

/// Represents a syntax token and its contents.
#[derive(Default)]
pub struct SyntaxToken {
    pub meta: Punctuated<syn::NestedMeta, Token![,]>,
}

impl SyntaxToken {
    /// Find the contents of the metadata field `name = "..."`
    /// if it exists.
    pub fn name(&self) -> Option<&syn::LitStr> {
        self.str_for_tag("name")
    }

    /// Find the contents of the metadata field `with_value = "..."`
    /// if it exists.
    pub fn custom_parser(&self) -> Option<&syn::LitStr> {
        self.str_for_tag("with_parser")
    }

    /// Find the contents of the metadata field `with_evaluator = "..."`
    /// if it exists.
    pub fn custom_evaluator(&self) -> Option<&syn::LitStr> {
        self.str_for_tag("with_evaluator")
    }

    fn str_for_tag(&self, tag: &str) -> Option<&syn::LitStr> {
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
