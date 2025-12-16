/// Helper macro for generating implementations for `one` fragments.
macro_rules! __generate_mut_visit_with_one_arm {
    (
        $Kind:ident, // e.g., Expr
        $AstTy:ty,   // e.g., Box<ast::Expr>
        $kind_name:expr, // e.g., "expression"
        $mut_visit_ast:ident, // e.g., visit_expr (method on MutVisitor)
        $visit_ast:ident, // e.g., visit_expr (method on Visitor)
        $ast_to_string:path, // e.g., pprust::expr_to_string
        $make_ast:ident // e.g., make_expr
    ) => {
        // Newtype wrapper
        // The structs ExpandedX are already defined in invocation_collector_node.rs
        // pub struct $Kind(pub $AstTy);

        impl $crate::InvocationCollectorNode for $Kind {
            type FlatMapOutputTy = $crate::SmallVec<Self, 1>;
            type VisitOutputTy = Option<Self>;

            const KIND: $crate::AstFragmentKind = $crate::AstFragmentKind::$Kind;

            fn to_annotatable(self) -> $crate::Annotatable {
                // Assuming Annotatable has a variant for each $Kind
                // This might need a custom match for each type, but for now, generic.
                $crate::Annotatable::$Kind(self.0)
            }

            fn fragment_to_visit_output(fragment: $crate::AstFragment) -> Option<Self> {
                match fragment {
                    $crate::AstFragment::$Kind(node) => Some(Self(node)),
                    _ => None, // Or panic, depending on expected behavior
                }
            }

            fn fragment_to_flat_map_output(fragment: $crate::AstFragment) -> $crate::SmallVec<Self, 1> {
                match fragment {
                    $crate::AstFragment::$Kind(node) => $crate::SmallVec::new_1(Self(node)),
                    _ => panic!("unexpected fragment kind"),
                }
            }

            fn descr() -> &'static str {
                $kind_name
            }

            fn walk<D: $crate::MutVisitor>(mut self, visitor: &mut D) -> Self {
                // Call the appropriate mut visitor method
                // Note: self.0 is Box<ast::Expr>, so &mut *self.0 gives &mut ast::Expr
                visitor.$mut_visit_ast(&mut *self.0);
                self
            }

            fn walk_flat_map<D: $crate::MutVisitor>(self, _visitor: &mut D) -> $crate::SmallVec<Self, 1> {
                // For 'one' fragments, the underlying MutVisitor doesn't have a direct 'flat_map_X' equivalent
                // that returns a SmallVec for a single item. The transformation happens in InvocationCollector::flat_map_node.
                // So, we just wrap the current node in a SmallVec.
                $crate::SmallVec::new_1(self)
            }

            fn is_mac_call(&self) -> bool {
                // Implement based on the inner AstTy. This will vary.
                // For Expr, it's matches!(&*self.0, ast::Expr { kind: ast::ExprKind::MacCall(_), .. })
                false
            }

            fn take_mac_call(self) -> (Box<ast::MacCall>, ast::AttrVec, $crate::AddSemicolon) {
                unreachable!() // Only for MacCall statement
            }

            fn delegation(&self) -> Option<(&ast::DelegationMac, &ast::Item<Self::ItemKind>)> {
                None
            }

            fn delegation_item_kind(_deleg: Box<ast::Delegation>) -> Self::ItemKind {
                unreachable!()
            }

            fn from_item(_item: ast::Item<Self::ItemKind>) -> Self {
                unreachable!()
            }

            fn flatten_outputs(_outputs: impl Iterator<Item = $crate::SmallVec<Self, 1>>) -> $crate::SmallVec<Self, 1> {
                unreachable!()
            }

            fn pre_flat_map_node_collect_attr<F>(_cfg: &F, _attr: &ast::Attribute) {}

            fn post_flat_map_node_collect_bang(_output: &mut $crate::SmallVec<Self, 1>, _add_semicolon: $crate::AddSemicolon) {}

            fn wrap_flat_map_node_walk_flat_map<D>(
                node: Self,
                _collector: &mut D,
                walk_flat_map: impl FnOnce(Self, &mut D) -> $crate::SmallVec<Self, 1>,
            ) -> Result<$crate::SmallVec<Self, 1>, Self> {
                Ok(walk_flat_map(node, _collector))
            }

            fn expand_cfg_false<D>(
                &mut self,
                _collector: &mut D,
                _pos: usize,
                _span: Span,
            ) {
            }

            fn declared_idents(&self) -> Vec<Ident> {
                vec![]
            }
        }
    };
}