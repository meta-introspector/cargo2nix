use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// A placeholder for a future `patch_build!` procedural macro.
#[proc_macro_attribute]
pub fn patch_build(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;

    let expanded = quote! {
        // The original function is preserved.
        #input

        // Some generated code could be added here in the future.
        fn #fn_name _patched() {
            println!("This function was processed by patch_build!");
        }
    };

    TokenStream::from(expanded)
}
