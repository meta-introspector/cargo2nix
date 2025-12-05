use super::traits::{CodeGenerator, RustParser};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_file, File};

/// Concrete implementation of RustParser and CodeGenerator using `syn` and `quote`.
pub struct SynQuoteProcessor;

impl RustParser<syn::Error> for SynQuoteProcessor {
    fn parse_str(&self, code: &str) -> Result<File, syn::Error> {
        parse_file(code)
    }
}

impl CodeGenerator<syn::Error> for SynQuoteProcessor {
    fn generate_code(&self, ast: &File) -> Result<TokenStream, syn::Error> {
        Ok(quote! { #ast })
    }
}
