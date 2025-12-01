use crate::declarations::{DeclarationKind, NixDeclaration};
use proc_macro2::Span as RealSpan;
use quote::ToTokens;
use std::collections::HashMap;
use syn::spanned::Spanned;
use syn::ItemImpl;
use syn::{
    visit::{self, Visit},
    Expr, File, Item, ItemConst, ItemEnum, ItemFn, ItemMod, ItemStatic, ItemStruct, ItemTrait,
    ItemType, ItemUse, Lit, Type,
};

/// A specialized visitor to collect `NixDeclaration`s with their properties,
/// focusing on bit field sizes and literal values.
pub struct InductiveDeclarationsCollector {
    pub declarations: Vec<NixDeclaration>,
    current_file_path: String,
    // Add any context needed for pathing, etc.
}

impl InductiveDeclarationsCollector {
    pub fn new(file_path: String) -> Self {
        Self {
            declarations: Vec::new(),
            current_file_path: file_path,
        }
    }

    /// Helper to create a path string
    fn create_path(&self, span: RealSpan) -> String {
        let span_str = span.to_string();
        let parts: Vec<&str> = span_str.split(':').collect();
        let (line, column) = if parts.len() >= 3 {
            // Format is typically "file.rs:line:column" or "file.rs:line:column:line:column"
            // We want the starting line and column.
            (
                parts[1].parse::<usize>().unwrap_or(0),
                parts[2].parse::<usize>().unwrap_or(0),
            )
        } else {
            (0, 0) // Default or error case
        };
        format!("{}:{}:{}", self.current_file_path, line, column)
    }

    // Helper to determine bit size for primitive types
    fn get_primitive_bit_size(ty: &Type) -> Option<u64> {
        match ty {
            Type::Path(type_path) => {
                if let Some(segment) = type_path.path.segments.last() {
                    let ident = segment.ident.to_string();
                    match ident.as_str() {
                        "bool" => Some(1),
                        "char" => Some(32), // Unicode scalar value
                        "u8" | "i8" => Some(8),
                        "u16" | "i16" => Some(16),
                        "u32" | "i32" | "f32" => Some(32),
                        "u64" | "i64" | "f64" => Some(64),
                        "u128" | "i128" => Some(128),
                        "usize" | "isize" => Some(64), // Assuming 64-bit platform
                        _ => None,
                    }
                } else {
                    None
                }
            }
            Type::Reference(type_ref) => {
                // Size of a reference (pointer)
                if type_ref.mutability.is_some() {
                    Some(64)
                } else {
                    Some(64)
                } // Assuming 64-bit pointers
            }
            Type::Array(type_array) => {
                if let (Some(elem_size), Some(len_expr)) = (
                    Self::get_primitive_bit_size(&type_array.elem),
                    Self::get_literal_value(&type_array.len),
                ) {
                    if let Ok(len) = len_expr.parse::<u64>() {
                        return Some(elem_size * len);
                    }
                }
                None
            }
            Type::Slice(type_slice) => {
                // Size of a slice is pointer + length (e.g., 2 * 64 bits)
                Some(128) // Assuming 64-bit platform for pointer and length
            }
            Type::Tuple(type_tuple) => {
                let mut total_size = 0;
                for elem_ty in &type_tuple.elems {
                    if let Some(size) = Self::get_primitive_bit_size(elem_ty) {
                        total_size += size;
                    } else {
                        return None; // Contains non-primitive or complex type
                    }
                }
                Some(total_size)
            }
            _ => None,
        }
    }

    // Helper to extract literal value from an expression
    fn get_literal_value(expr: &Expr) -> Option<String> {
        match expr {
            Expr::Lit(expr_lit) => Some(expr_lit.lit.to_token_stream().to_string()),
            Expr::Unary(expr_unary) => {
                if let syn::UnOp::Neg(_) = expr_unary.op {
                    // Recursively get the value of the negated expression
                    Self::get_literal_value(&expr_unary.expr).map(|val| format!("-{}", val))
                } else {
                    None
                }
            }
            Expr::Path(expr_path) => {
                // This might be a reference to a const.
                // For now, we're not resolving paths, but if it's a simple path,
                // we can return its string representation.
                if expr_path.path.get_ident().is_some() {
                    Some(expr_path.path.get_ident().unwrap().to_string())
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    // Method to finalize and assign monster factors
    pub fn finalize_declarations(&mut self, monster_primes: &[u64]) {
        let sorted_monster_primes = {
            let mut s = monster_primes.to_vec();
            s.sort_unstable();
            s.dedup(); // Remove duplicates for efficiency in checks
            s
        };

        for decl in &mut self.declarations {
            // Collect factors from bit_size
            if let Some(bit_size) = decl.bit_size {
                for &prime in sorted_monster_primes.iter() {
                    if prime == 1 {
                        // Handle 1-bit explicitly as a direct match only
                        if bit_size == 1 {
                            decl.monster_factors.push(prime);
                        }
                    } else if bit_size == prime || (bit_size % prime == 0 && bit_size > prime) {
                        decl.monster_factors.push(prime);
                    }
                }
            }

            // Collect factors from value
            if let Some(value_str) = &decl.value {
                if let Ok(value) = value_str.parse::<u64>() {
                    for &prime in sorted_monster_primes.iter() {
                        if prime == 1 {
                            // Handle 1-value explicitly as a direct match only
                            if value == 1 {
                                decl.monster_factors.push(prime);
                            }
                        } else if value == prime || (value % prime == 0 && value > prime) {
                            decl.monster_factors.push(prime);
                        }
                    }
                }
            }
            decl.monster_factors.sort_unstable();
            decl.monster_factors.dedup();
        }
    }
}

impl<'ast> Visit<'ast> for InductiveDeclarationsCollector {
    fn visit_item_fn(&mut self, i: &'ast ItemFn) {
        self.declarations.push(NixDeclaration {
            kind: DeclarationKind::Fn,
            name: i.sig.ident.to_string(),
            path: self.create_path(i.span()),
            bit_size: None, // Functions don't have a direct bit size
            value: None,    // Or value
            monster_factors: Vec::new(),
        });
        visit::visit_item_fn(self, i);
    }

    fn visit_item_struct(&mut self, i: &'ast ItemStruct) {
        let mut total_bit_size = 0;
        // Simplified approach for now: sum primitive field sizes
        for field in &i.fields {
            if let Some(size) = Self::get_primitive_bit_size(&field.ty) {
                total_bit_size += size;
            } else {
                // If it's a complex type, we need to handle it later
                total_bit_size = 0; // Reset or mark as unknown if complex type
                break;
            }
        }

        self.declarations.push(NixDeclaration {
            kind: DeclarationKind::Struct,
            name: i.ident.to_string(),
            path: self.create_path(i.span()),
            bit_size: if total_bit_size > 0 {
                Some(total_bit_size)
            } else {
                None
            },
            value: None,
            monster_factors: Vec::new(),
        });
        visit::visit_item_struct(self, i);
    }

    fn visit_item_enum(&mut self, i: &'ast ItemEnum) {
        // Enums are trickier for direct bit size; typically depend on variants and discriminant
        // For now, only record the enum itself.
        self.declarations.push(NixDeclaration {
            kind: DeclarationKind::Enum,
            name: i.ident.to_string(),
            path: self.create_path(i.span()),
            bit_size: None, // Complex, will handle later if needed
            value: None,
            monster_factors: Vec::new(),
        });
        visit::visit_item_enum(self, i);
    }

    fn visit_item_const(&mut self, i: &'ast ItemConst) {
        self.declarations.push(NixDeclaration {
            kind: DeclarationKind::Const,
            name: i.ident.to_string(),
            path: self.create_path(i.span()),
            bit_size: Self::get_primitive_bit_size(&i.ty),
            value: Self::get_literal_value(&i.expr),
            monster_factors: Vec::new(),
        });
        visit::visit_item_const(self, i);
    }

    fn visit_item_static(&mut self, i: &'ast ItemStatic) {
        self.declarations.push(NixDeclaration {
            kind: DeclarationKind::Static,
            name: i.ident.to_string(),
            path: self.create_path(i.span()),
            bit_size: Self::get_primitive_bit_size(&i.ty),
            value: Self::get_literal_value(&i.expr),
            monster_factors: Vec::new(),
        });
        visit::visit_item_static(self, i);
    }

    fn visit_item_mod(&mut self, i: &'ast ItemMod) {
        self.declarations.push(NixDeclaration {
            kind: DeclarationKind::Mod,
            name: i.ident.to_string(),
            path: self.create_path(i.span()),
            bit_size: None,
            value: None,
            monster_factors: Vec::new(),
        });
        visit::visit_item_mod(self, i);
    }

    fn visit_item_use(&mut self, i: &'ast ItemUse) {
        // The path in `use` statements can be complex. For simplicity, just store the raw use statement.
        self.declarations.push(NixDeclaration {
            kind: DeclarationKind::Use,
            name: format!("{:?}", i.tree),
            path: self.create_path(i.span()),
            bit_size: None,
            value: None,
            monster_factors: Vec::new(),
        });
        visit::visit_item_use(self, i);
    }

    fn visit_item_type(&mut self, i: &'ast ItemType) {
        self.declarations.push(NixDeclaration {
            kind: DeclarationKind::Type,
            name: i.ident.to_string(),
            path: self.create_path(i.span()),
            bit_size: Self::get_primitive_bit_size(&i.ty), // If it's a type alias to a primitive
            value: None,
            monster_factors: Vec::new(),
        });
        visit::visit_item_type(self, i);
    }

    fn visit_item_trait(&mut self, i: &'ast ItemTrait) {
        self.declarations.push(NixDeclaration {
            kind: DeclarationKind::Trait,
            name: i.ident.to_string(),
            path: self.create_path(i.span()),
            bit_size: None,
            value: None,
            monster_factors: Vec::new(),
        });
        visit::visit_item_trait(self, i);
    }

    fn visit_item_impl(&mut self, i: &'ast ItemImpl) {
        // Impls are complex; typically don't have a direct "size" or "value"
        self.declarations.push(NixDeclaration {
            kind: DeclarationKind::Impl,
            name: format!("{:?}", i.self_ty), // Use debug print for type for now
            path: self.create_path(i.span()),
            bit_size: None,
            value: None,
            monster_factors: Vec::new(),
        });
        visit::visit_item_impl(self, i);
    }
}
