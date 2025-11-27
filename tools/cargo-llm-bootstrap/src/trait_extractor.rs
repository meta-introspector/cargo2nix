use std::fs;
use std::path::Path;
use syn::{File, Item, ItemEnum, ItemFn, ItemImpl, ItemStruct, ItemTrait, Signature, ReturnType, Type as SynType};
use syn::punctuated::Punctuated;
use syn::token::Paren;
//use crate::error::AppError;
use crate::trait_types::{DeclKind, DeclTrait, TraitDeps};

pub struct TraitExtractor;

impl TraitExtractor {
    pub fn extract_from_file(&self, file_path: &Path) -> Result<(Vec<DeclTrait>, Vec<TraitDeps>), AppError> {
        let content = fs::read_to_string(file_path).map_err(AppError::Io)?;
        let syntax_tree = syn::parse_file(&content).map_err(|e| AppError::Custom(format!("Failed to parse Rust file {}: {}", file_path.display(), e)))?;
        self.extract_from_syntax_tree(syntax_tree)
    }

    pub fn extract_from_syntax_tree(&self, syntax_tree: File) -> Result<(Vec<DeclTrait>, Vec<TraitDeps>), AppError> {
        let mut decl_traits = Vec::new();
        let mut trait_deps = Vec::new();

        for item in syntax_tree.items {
            match item {
                Item::Struct(item_struct) => {
                    decl_traits.push(self.extract_struct(&item_struct));
                },
                Item::Enum(item_enum) => {
                    decl_traits.push(self.extract_enum(&item_enum));
                },
                Item::Fn(item_fn) => {
                    decl_traits.push(self.extract_fn(&item_fn));
                },
                Item::Trait(item_trait) => {
                    decl_traits.push(self.extract_trait(&item_trait));
                },
                Item::Impl(item_impl) => {
                    let (decl_trait, deps) = self.extract_impl(&item_impl)?;
                    decl_traits.push(decl_trait);
                    trait_deps.push(deps);
                },
                Item::Mod(item_mod) => {
                    // Handle modules later if needed, for now just create a basic trait
                    decl_traits.push(DeclTrait {
                        name: item_mod.ident.to_string(),
                        kind: DeclKind::Module,
                        generics: Vec::new(),
                        bounds: Vec::new(),
                        associated_items: Vec::new(),
                        godel_number: 0, // Placeholder
                    });
                },
                _ => {
                    // Ignore other items for now or handle as 'Other'
                }
            }
        }

        Ok((decl_traits, trait_deps))
    }

    fn extract_struct(&self, item_struct: &ItemStruct) -> DeclTrait {
        let generics = item_struct.generics.params.iter().map(|param| param.to_string()).collect();
        let bounds = item_struct.generics.where_clause.as_ref().map_or(Vec::new(), |clause| {
            clause.predicates.iter().map(|pred| pred.to_string()).collect()
        });

        DeclTrait {
            name: item_struct.ident.to_string(),
            kind: DeclKind::Struct,
            generics,
            bounds,
            associated_items: Vec::new(), // Structs don't have associated items in this context
            godel_number: 0, // Placeholder
        }
    }

    fn extract_enum(&self, item_enum: &ItemEnum) -> DeclTrait {
        let generics = item_enum.generics.params.iter().map(|param| param.to_string()).collect();
        let bounds = item_enum.generics.where_clause.as_ref().map_or(Vec::new(), |clause| {
            clause.predicates.iter().map(|pred| pred.to_string()).collect()
        });

        DeclTrait {
            name: item_enum.ident.to_string(),
            kind: DeclKind::Enum,
            generics,
            bounds,
            associated_items: Vec::new(),
            godel_number: 0, // Placeholder
        }
    }

    fn extract_fn(&self, item_fn: &ItemFn) -> DeclTrait {
        let generics = item_fn.sig.generics.params.iter().map(|param| param.to_string()).collect();
        let bounds = item_fn.sig.generics.where_clause.as_ref().map_or(Vec::new(), |clause| {
            clause.predicates.iter().map(|pred| pred.to_string()).collect()
        });

        // Extract return type as an associated item if it's a known trait
        let mut associated_items = Vec::new();
        if let ReturnType::Type(_, ty) = &item_fn.sig.output {
            if let SynType::Path(type_path) = &**ty {
                if let Some(segment) = type_path.path.segments.last() {
                    associated_items.push(segment.ident.to_string());
                }
            }
        }

        DeclTrait {
            name: item_fn.sig.ident.to_string(),
            kind: DeclKind::Function,
            generics,
            bounds,
            associated_items,
            godel_number: 0, // Placeholder
        }
    }

    fn extract_trait(&self, item_trait: &ItemTrait) -> DeclTrait {
        let generics = item_trait.generics.params.iter().map(|param| param.to_string()).collect();
        let bounds = item_trait.supertraits.iter().map(|sup| sup.to_string()).collect();
        let associated_items = item_trait.items.iter().filter_map(|item| {
            if let syn::TraitItem::Fn(method) = item {
                Some(method.sig.ident.to_string())
            } else {
                None
            }
        }).collect();

        DeclTrait {
            name: item_trait.ident.to_string(),
            kind: DeclKind::Trait,
            generics,
            bounds,
            associated_items,
            godel_number: 0, // Placeholder
        }
    }

    fn extract_impl(&self, item_impl: &ItemImpl) -> Result<(DeclTrait, TraitDeps), AppError> {
        let generics = item_impl.generics.params.iter().map(|param| param.to_string()).collect();
        let bounds = item_impl.generics.where_clause.as_ref().map_or(Vec::new(), |clause| {
            clause.predicates.iter().map(|pred| pred.to_string()).collect()
        });

        let mut impl_trait_name = "UnknownTrait".to_string();
        let mut impl_for_type = "UnknownType".to_string();
        let mut dependencies = Vec::new();

        if let Some((_, path, _)) = &item_impl.trait_ {
            impl_trait_name = path.segments.last().map_or("UnknownTrait".to_string(), |segment| segment.ident.to_string());
            // Extract dependencies from the trait path itself if any (e.g., `TraitA for Type where Type: TraitB`)
            for segment in path.segments.iter() {
                dependencies.push(segment.ident.to_string());
            }
        }
        if let SynType::Path(type_path) = &*item_impl.self_ty {
            impl_for_type = type_path.path.segments.last().map_or("UnknownType".to_string(), |segment| segment.ident.to_string());
        }

        // Analyze methods within the impl block for dependencies
        for item in &item_impl.items {
            if let syn::ImplItem::Fn(method) = item {
                // Analyze arguments for trait dependencies
                for input in &method.sig.inputs {
                    if let syn::FnArg::Receiver(_) = input {
                        // `self` implies dependency on the impl's type
                    } else if let syn::FnArg::Typed(pat_type) = input {
                        if let SynType::Path(type_path) = &*pat_type.ty {
                            if let Some(segment) = type_path.path.segments.last() {
                                dependencies.push(segment.ident.to_string());
                            }
                        }
                    }
                }
                // Analyze return type for trait dependencies
                if let ReturnType::Type(_, ty) = &method.sig.output {
                    if let SynType::Path(type_path) = &**ty {
                        if let Some(segment) = type_path.path.segments.last() {
                            dependencies.push(segment.ident.to_string());
                        }
                    }
                }
            }
        }
        // Deduplicate dependencies
        dependencies.sort();
        dependencies.dedup();

        let decl_trait = DeclTrait {
            name: format!("{}::{}", impl_for_type, impl_trait_name),
            kind: DeclKind::Impl,
            generics,
            bounds,
            associated_items: Vec::new(),
            godel_number: 0, // Placeholder
        };

        let trait_deps = TraitDeps {
            impl_trait_name: impl_trait_name.clone(),
            impl_for_type: impl_for_type.clone(),
            dependencies,
            godel_number: 0, // Placeholder
        };

        Ok((decl_trait, trait_deps))
    }
}
