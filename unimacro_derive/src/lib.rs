use proc_macro::TokenStream;
use syn::{parse_macro_input, AttributeArgs, Lit, NestedMeta};
use quote::quote;

/// A fun "unimacro" that can veto the item it's applied to based on a condition.
///
/// Usage:
/// #[veto_if(condition = "cfg(debug_assertions)")]
/// fn my_debug_only_function() { /* ... */ }
///
/// #[veto_if(condition = "always_true")] // This would always veto
/// struct MyForbiddenStruct;
///
/// Note: "cfg(...)" conditions are evaluated at compile time.
#[proc_macro_attribute]
pub fn veto_if(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);

    let mut condition_str = None;

    for arg in args {
        if let NestedMeta::Meta(syn::Meta::NameValue(nv)) = arg {
            if nv.path.is_ident("condition") {
                if let Lit::Str(lit_str) = nv.lit {
                    condition_str = Some(lit_str.value());
                } else {
                    return syn::Error::new_spanned(nv.lit, "expected string literal for 'condition'")
                        .to_compile_error()
                        .into();
                }
            }
        }
    }

    let Some(condition) = condition_str else {
        return syn::Error::new_spanned(
            proc_macro2::TokenStream::new(), // Use proc_macro2::TokenStream::new() instead of TokenStream::from(quote!{}) for cleaner span
            "missing 'condition' argument, e.g., #[veto_if(condition = \"cfg(debug_assertions)\")]"
        )
        .to_compile_error()
        .into();
    };

    // For simplicity, we'll only support "cfg(...)" conditions for now.
    // A real "unimacro" would have a more complex evaluation engine.
    if condition.starts_with("cfg(") && condition.ends_with(')') {
        let inner_condition = &condition["cfg(".len()..condition.len() - 1];

        // This is a simplification. Real cfg evaluation involves more than just checking
        // predefined strings. `rustc_parse::parse_in_src_root_attr_cfg` could be used
        // if we had access to the compiler session, but we don't in a proc-macro.
        // For this fun task, we'll simulate a simple check.

        // Simulating cfg evaluation:
        let should_veto = match inner_condition {
            "debug_assertions" => cfg!(debug_assertions),
            "always_true" => true,
            "always_false" => false,
            // Add more conditions or a more robust parsing/evaluation here
            _ => {
                // Unknown condition, perhaps emit a warning or error, or default to false
                // For this fun task, we'll treat unknown as false to not break builds unexpectedly
                false
            }
        };

        if should_veto {
            // If the condition is true, return an empty token stream, effectively deleting the item.
            TokenStream::new()
        } else {
            // Otherwise, return the original item unchanged.
            item
        }
    } else {
        // If it's not a cfg-like condition, for simplicity, we'll treat it as false
        // or let the user define custom string conditions.
        // For this "fun" unimacro, let's treat non-cfg conditions as always false for now,
        // so they don't veto unless explicitly "always_true".
        item
    }
}
