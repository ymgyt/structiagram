use quote::quote;
use std::fmt::{self, Formatter};

pub(super) struct IdentFormatter<'a>(Option<&'a syn::Ident>);

impl<'a> fmt::Display for IdentFormatter<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0.map(|ident| ident.to_string()).unwrap_or_default()
        )
    }
}

impl<'a> From<Option<&'a syn::Ident>> for IdentFormatter<'a> {
    fn from(ident: Option<&'a syn::Ident>) -> Self {
        Self(ident)
    }
}

pub(super) struct TypeFormatter<'a>(&'a syn::Type);

impl<'a> TypeFormatter<'a> {
    pub(super) fn new(ty: &'a syn::Type) -> Self {
        Self(ty)
    }
}

impl<'a> fmt::Display for TypeFormatter<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.0 {
            syn::Type::Path(type_path) => {
                type_path
                    .path
                    .segments
                    .pairs()
                    .try_for_each(|pair| match pair {
                        syn::punctuated::Pair::Punctuated(seg, _colon) => {
                            // mermaid does not allow to use `::`
                            write!(f, "{}_", seg.ident)
                        }
                        syn::punctuated::Pair::End(seg) => {
                            write!(
                                f,
                                "{}",
                                quote!(#seg)
                                    .to_string()
                                    .replace(" ", "")
                                    .replace("::", "_")
                                    .replace("<", "_")
                                    .replace(">", "")
                            )
                        }
                    })
            }
            _ => write!(f, ""),
        }
    }
}
