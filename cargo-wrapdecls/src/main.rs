use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use syn::{parse_file, Item};
use quote::quote;
use walkdir::WalkDir;
use anyhow::{Result, anyhow};
use regex::Regex;
use lazy_static::lazy_static;
use tracing::{info, debug, error, Level};
use tracing_subscriber::FmtSubscriber;

// Configuration for skipping specific items or files
lazy_static! {
    static ref SKIP_ITEM_REGEX: Regex = Regex::new(r"#[allow\(dead_code\)]").unwrap();
    static ref SKIP_FILE_REGEX: Regex = Regex::new(r"skip_this_file.rs").unwrap();
}


fn main() -> Result<()> {
    // Initialize tracing subscriber for logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    let args: Vec<_> = env::args().collect();
    // Adjust argument parsing to correctly handle `cargo wrapdecls`
    // The first argument is "cargo", the second is "wrapdecls"
    if args.len() < 2 || args[1] != "wrapdecls" {
        error!("Usage: cargo wrapdecls [OPTIONS]");
        std::process::exit(1);
    }
    
    info!("Starting cargo-wrapdecls plugin...");

    // Find Cargo.toml, read workspace/target crate
    let manifest_path = find_manifest()?;
    info!("Found Cargo.toml at: {}", manifest_path.display());

    let out_dir = env::var("OUT_DIR").ok(); // Reuse if in build context
    debug!("OUT_DIR: {:?}", out_dir);
    
    // Parse src/lib.rs + src/**/*.rs
    let mut items = vec![];
    let src_path = manifest_path.parent().unwrap().join("src");
    info!("Collecting items from: {}", src_path.display());
    collect_items(&src_path, &mut items)?;
    info!("Collected {} top-level items.", items.len());
    
    // Transform: wrap each top-level item
    let wrapped_items: Vec<_> = items.into_iter()
        .filter_map(|item| wrap_decl(item)) // Use filter_map to handle None for skipped items
        .collect();
    info!("Wrapped {} declarations.", wrapped_items.len());
    
    let tokens = quote! {
        pub mod generated_wrapped {
            #(#wrapped_items)*
        }
    };
    
    let out_path = if let Some(d) = out_dir {
        Path::new(&d).join("wrapped.rs")
    } else {
        src_path.join("generated_wrapped.rs")
    };
    
    fs::write(&out_path, tokens.to_string())?;
    
    println!("cargo:rerun-if-changed=src");
    println!("Generated {} with {} wrapped decls", out_path.display(), wrapped_items.len());

    Ok(())
}

fn find_manifest() -> Result<PathBuf> {
    let current_dir = env::current_dir()?;
    let mut current_path = Some(current_dir.as_path());

    while let Some(path) = current_path {
        let manifest_path = path.join("Cargo.toml");
        if manifest_path.exists() {
            return Ok(manifest_path);
        }
        current_path = path.parent();
    }
    Err(anyhow!("Cargo.toml not found in current directory or any parent directory."))
}

fn collect_items(src_dir: &Path, items: &mut Vec<Item>) -> Result<()> {
    for entry in WalkDir::new(src_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
            let content = fs::read_to_string(path)?;
            if SKIP_FILE_REGEX.is_match(&content) {
                debug!("Skipping file due to regex match: {}", path.display());
                continue;
            }
            let file = parse_file(&content)?;
            items.extend(file.items);
        }
    }
    Ok(())
}

fn wrap_decl(item: Item) -> Option<proc_macro2::TokenStream> {
    // Check for comments/attributes that indicate skipping
    let item_string = quote! { #item }.to_string();
    if SKIP_ITEM_REGEX.is_match(&item_string) {
        debug!("Skipping item due to regex match: {}", item_string);
        return None;
    }

    match item {
        Item::Fn(mut f) => {
            let ident = &f.sig.ident;
            let attrs = &f.attrs;
            let sig = &f.sig;
            let body = f.block;
            Some(quote! {
                #(#attrs)*
                pub fn #ident #sig {
                    tracing::trace!("wrapped function: {}", stringify!(#ident));
                    #body
                }
            })
        }
        Item::Struct(s) => {
            let ident = &s.ident;
            let attrs = &s.attrs;
            let vis = &s.vis;
            let generics = &s.generics;
            let fields = &s.fields;
            Some(quote! {
                #(#attrs)*
                #vis struct #ident #generics #fields
            })
        }
        Item::Enum(e) => {
            let ident = &e.ident;
            let attrs = &e.attrs;
            let vis = &e.vis;
            let generics = &e.generics;
            let variants = &e.variants;
            Some(quote! {
                #(#attrs)*
                #vis enum #ident #generics {
                    #variants
                }
            })
        }
        Item::Trait(t) => {
            let ident = &t.ident;
            let attrs = &t.attrs;
            let vis = &t.vis;
            let generics = &t.generics;
            let supertraits = &t.supertraits;
            let items = &t.items;
            Some(quote! {
                #(#attrs)*
                #vis trait #ident #generics: #supertraits {
                    #(#items)*
                }
            })
        }
        Item::Impl(i) => {
            let attrs = &i.attrs;
            let generics = &i.generics;
            let self_ty = &i.self_ty;
            let items = &i.items;

            let trait_impl = if let Some((polarity, path, for_token)) = &i.trait_ {
                // If there's a trait, handle the optional `!` (Not token)
                // and the `For` token.
                if let Some(not_token) = polarity {
                    quote! { #not_token #path #for_token }
                } else {
                    quote! { #path #for_token }
                }
            } else {
                // If no trait, it's an inherent impl, so nothing to quote here
                quote! {}
            };

            Some(quote! {
                #(#attrs)*
                impl #generics #trait_impl #self_ty {
                    #(#items)*
                }
            })
        }
        Item::Use(u) => {
            let attrs = &u.attrs;
            let vis = &u.vis;
            let leading_colon = &u.leading_colon;
            let tree = &u.tree;
            Some(quote! {
                #(#attrs)*
                #vis use #leading_colon #tree;
            })
        }
        // Add more Item variants as needed for comprehensive wrapping
        _ => {
            debug!("Skipping unsupported item type.");
            None // Return None for unsupported item types
        }
    }
}
