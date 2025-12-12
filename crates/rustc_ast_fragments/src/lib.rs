use rustc_ast::{self as ast, NodeId};
use rustc_span::Ident;
use smallvec::SmallVec;

use crate::base::MacResult;

// This needs to be available for the macro to define methods.
// placeholder_visibility() will be defined in ast_fragments.rs
// to avoid circular dependency.
pub trait AstFragmentKindExt {
    fn placeholder_visibility(&self) -> Option<Ident>;
}

// The ast_fragments! macro definition
macro_rules! ast_fragments {
    (
        $($Kind:ident($AstTy:ty) {
            $kind_name:expr;
            $(one
                fn $mut_visit_ast:ident;
                fn $visit_ast:ident;
                fn $ast_to_string:path;
            )?
            $(many
                fn $flat_map_ast_elt:ident;
                fn $visit_ast_elt($($args:tt)*);
                fn $ast_to_string_elt:path;
            )?
            fn $make_ast:ident;
        })*
    ) => {
        /// A fragment of AST that can be produced by a single macro expansion.
        /// Can also serve as an input and intermediate result for macro expansion operations.
        #[derive(Debug)]
        pub enum AstFragment {
            OptExpr(Option<Box<ast::Expr>>),
            MethodReceiverExpr(Box<ast::Expr>),
            $($Kind($AstTy),)*
        }

        /// "Discriminant" of an AST fragment.
        #[derive(Copy, Clone, Debug, PartialEq, Eq)]
        pub enum AstFragmentKind {
            OptExpr,
            MethodReceiverExpr,
            $($Kind,)*
        }

        impl AstFragmentKind {
            pub fn name(self) -> &'static str {
                match self {
                    AstFragmentKind::OptExpr => "expression",
                    AstFragmentKind::MethodReceiverExpr => "expression",
                    $(AstFragmentKind::$Kind => $kind_name,)*
                }
            }

            pub(crate) fn make_from(self, result: Box<dyn MacResult + '_>) -> Option<AstFragment> {
                match self {
                    AstFragmentKind::OptExpr =>
                        result.make_expr().map(Some).map(AstFragment::OptExpr),
                    AstFragmentKind::MethodReceiverExpr =>
                        result.make_expr().map(AstFragment::MethodReceiverExpr),
                    $(AstFragmentKind::$Kind => result.$make_ast().map(AstFragment::$Kind),)*
                }
            }

            pub(crate) fn dummy(self, span: Span, guar: ErrorGuaranteed) -> AstFragment {
                self.make_from(DummyResult::any(span, guar)).expect("couldn't create a dummy AST fragment")
            }
        }

        impl AstFragment {
            pub(crate) fn make_opt_expr(self) -> Option<Box<ast::Expr>> {
                match self {
                    AstFragment::OptExpr(expr) => expr,
                    _ => panic!("AstFragment::make_* called on the wrong kind of fragment"),
                }
            }

            pub(crate) fn make_method_receiver_expr(self) -> Box<ast::Expr> {
                match self {
                    AstFragment::MethodReceiverExpr(expr) => expr,
                    _ => panic!("AstFragment::make_* called on the wrong kind of fragment"),
                }
            }

            $(pub fn $make_ast(self) -> $AstTy {
                match self {
                    AstFragment::$Kind(ast) => ast,
                    _ => panic!("AstFragment::make_* called on the wrong kind of fragment"),
                }
            })*

            pub(crate) fn to_string(&self) -> String {
                match self {
                    AstFragment::OptExpr(Some(expr)) => pprust::expr_to_string(expr),
                    AstFragment::OptExpr(None) => String::new(), // Handle None case, was unreachable!
                    AstFragment::MethodReceiverExpr(expr) => pprust::expr_to_string(expr),
                    $($(AstFragment::$Kind(ast) => $ast_to_string(ast),)?)*
                    $($( 
                        AstFragment::$Kind(ast) => {
                            // The closure unwraps a `P` if present, or does nothing otherwise.
                            elems_to_string(&*ast, |ast| $ast_to_string_elt(&*ast))
                        }
                    )?)*
                }
            }
        }
    }
}

// The ast_fragments! macro invocation
ast_fragments! {
    Expr(Box<ast::Expr>) {
        "expression";
        one fn visit_expr; fn visit_expr; fn pprust::expr_to_string;
        fn make_expr;
    }
    Pat(Box<ast::Pat>) {
        "pattern";
        one fn visit_pat; fn visit_pat; fn pprust::pat_to_string;
        fn make_pat;
    }
    Ty(Box<ast::Ty>) {
        "type";
        one fn visit_ty; fn visit_ty; fn pprust::ty_to_string;
        fn make_ty;
    }
    Stmts(SmallVec<ast::Stmt, 1>) {
        "statement";
        many fn flat_map_stmt; fn visit_stmt(); fn pprust::stmt_to_string;
        fn make_stmts;
    }
    Items(SmallVec<Box<ast::Item>, 1>) {
        "item";
        many fn flat_map_item; fn visit_item(); fn pprust::item_to_string;
        fn make_items;
    }
    TraitItems(SmallVec<Box<ast::AssocItem>, 1>) {
        "trait item";
        many fn flat_map_assoc_item; fn visit_assoc_item(AssocCtxt::Trait);
            fn pprust::assoc_item_to_string;
        fn make_trait_items;
    }
    ImplItems(SmallVec<Box<ast::AssocItem>, 1>) {
        "impl item";
        many fn flat_map_assoc_item; fn visit_assoc_item(AssocCtxt::Impl { of_trait: false });
            fn pprust::assoc_item_to_string;
        fn make_impl_items;
    }
    TraitImplItems(SmallVec<Box<ast::AssocItem>, 1>) {
        "impl item";
        many fn flat_map_assoc_item; fn visit_assoc_item(AssocCtxt::Impl { of_trait: true });
            fn pprust::assoc_item_to_string;
        fn make_trait_impl_items;
    }
    ForeignItems(SmallVec<Box<ast::ForeignItem>, 1>) {
        "foreign item";
        many fn flat_map_foreign_item; fn visit_foreign_item(); fn pprust::foreign_item_to_string;
        fn make_foreign_items;
    }
    Arms(SmallVec<ast::Arm, 1>) {
        "match arm";
        many fn flat_map_arm; fn visit_arm(); fn unreachable_to_string;
        fn make_arms;
    }
    ExprFields(SmallVec<ast::ExprField, 1>) {
        "field expression";
        many fn flat_map_expr_field; fn visit_expr_field(); fn unreachable_to_string;
        fn make_expr_fields;
    }
    PatFields(SmallVec<ast::PatField, 1>) {
        "field pattern";
        many fn flat_map_pat_field; fn visit_pat_field(); fn unreachable_to_string;
        fn make_pat_fields;
    }
    GenericParams(SmallVec<ast::GenericParam, 1>) {
        "generic parameter";
        many fn flat_map_generic_param; fn visit_generic_param(); fn unreachable_to_string;
        fn make_generic_params;
    }
    Params(SmallVec<ast::Param, 1>) {
        "function parameter";
        many fn flat_map_param; fn visit_param(); fn unreachable_to_string;
        fn make_params;
    }
    FieldDefs(SmallVec<ast::FieldDef, 1>) {
        "field";
        many fn flat_map_field_def; fn visit_field_def(); fn unreachable_to_string;
        fn make_field_defs;
    }
    Variants(SmallVec<ast::Variant, 1>) {
        "variant"; many fn flat_map_variant; fn visit_variant(); fn unreachable_to_string;
        fn make_variants;
    }
    WherePredicates(SmallVec<ast::WherePredicate, 1>) {
        "where predicate";
        many fn flat_map_where_predicate; fn visit_where_predicate(); fn unreachable_to_string;
        fn make_where_predicates;
    }
    Crate(ast::Crate) {
        "crate";
        one fn visit_crate; fn visit_crate; fn unreachable_to_string;
        fn make_crate;
    }
}

// Dummy `elems_to_string` to satisfy the macro. This will be replaced or properly defined in ast_fragments.rs
fn elems_to_string<T, F>(_elems: &[T], _f: F) -> String
where
    F: Fn(&T) -> String,
{
    // This is a placeholder, as the actual implementation needs context from ast_fragments.rs
    // For now, return an empty string or a placeholder message.
    String::from("")
}

pub struct ParserAnyMacro<'a> {
    parser: Parser<'a>,

    /// Span of the expansion site of the macro this parser is for
    site_span: Span,
    /// The ident of the macro we're parsing
    macro_ident: Ident,
    lint_node_id: NodeId,
    is_trailing_mac: bool,
    arm_span: Span,
    /// Whether or not this macro is defined in the current crate
    is_local: bool,
}

impl<'a> ParserAnyMacro<'a> {
    pub(crate) fn make(mut self: Box<self::ParserAnyMacro<'a>>, kind: AstFragmentKind) -> AstFragment {
        let self::ParserAnyMacro { 
            site_span,
            macro_ident,
            ref mut parser,
            lint_node_id,
            arm_span,
            is_trailing_mac,
            is_local,
        } = *self;
        let snapshot = &mut parser.create_snapshot_for_diagnostic();
        let fragment = match parse_ast_fragment(parser, kind) {
            Ok(f) => f,
            Err(err) => {
                let guar = diagnostics::emit_frag_parse_err(
                    err, parser, snapshot, site_span, arm_span, kind,
                );
                return kind.dummy(site_span, guar);
            }
        };

        // We allow semicolons at the end of expressions -- e.g., the semicolon in
        // `macro_rules! m { () => { panic!(); } }` isn't parsed by `.parse_expr()`,
        // but `m!()` is allowed in expression positions (cf. issue #34706).
        if kind == AstFragmentKind::Expr && parser.token == ast::token::Semi {
            if is_local {
                parser.psess.buffer_lint(
                    SEMICOLON_IN_EXPRESSIONS_FROM_MACROS,
                    parser.token.span,
                    lint_node_id,
                    errors::TrailingMacro { is_trailing: is_trailing_mac, name: macro_ident },
                );
            }
            parser.bump();
        }

        // Make sure we don't have any tokens left to parse so we don't silently drop anything.
        let path = ast::Path::from_ident(macro_ident.with_span_pos(site_span));
        ensure_complete_parse(parser, &path, kind.name(), site_span);
        fragment
    }

    #[tracing::instrument(skip(cx, tts))]
    pub(crate) fn from_tts<'cx>(
        cx: &'cx mut ExtCtxt<'a>,
        tts: TokenStream,
        site_span: Span,
        arm_span: Span,
        is_local: bool,
        macro_ident: Ident,
    ) -> Self {
        Self {
            parser: Parser::new(&cx.sess.psess, tts, None),

            // Pass along the original expansion site and the name of the macro
            // so we can print a useful error message if the parse of the expanded
            // macro leaves unparsed tokens.
            site_span,
            macro_ident,
            lint_node_id: cx.current_expansion.lint_node_id,
            is_trailing_mac: cx.current_expansion.is_trailing_mac,
            arm_span,
            is_local,
        }
    }
}

pub enum SupportsMacroExpansion {
    No,
    Yes { supports_inner_attrs: bool },
}

impl AstFragmentKind {
    pub fn supports_macro_expansion(self) -> SupportsMacroExpansion {
        match self {
            AstFragmentKind::OptExpr
            | AstFragmentKind::Expr
            | AstFragmentKind::MethodReceiverExpr
            | AstFragmentKind::Stmts
            | AstFragmentKind::Ty
            | AstFragmentKind::Pat => SupportsMacroExpansion::Yes { supports_inner_attrs: false },
            AstFragmentKind::Items
            | AstFragmentKind::TraitItems
            | AstFragmentKind::ImplItems
            | AstFragmentKind::TraitImplItems
            | AstFragmentKind::ForeignItems
            | AstFragmentKind::Crate => SupportsMacroExpansion::Yes { supports_inner_attrs: true },
            AstFragmentKind::Arms
            | AstFragmentKind::ExprFields
            | AstFragmentKind::PatFields
            | AstFragmentKind::GenericParams
            | AstFragmentKind::Params
            | AstFragmentKind::FieldDefs
            | AstFragmentKind::Variants
            | AstFragmentKind::WherePredicates => SupportsMacroExpansion::No,
        }
    }

    pub(crate) fn expect_from_annotatables(
        self,
        items: impl IntoIterator<Item = Annotatable>,
    ) -> AstFragment {
        let mut items = items.into_iter();
        match self {
            AstFragmentKind::Arms => {
                AstFragment::Arms(items.map(Annotatable::expect_arm).collect())
            }
            AstFragmentKind::ExprFields => {
                AstFragment::ExprFields(items.map(Annotatable::expect_expr_field).collect())
            }
            AstFragmentKind::PatFields => {
                AstFragment::PatFields(items.map(Annotatable::expect_pat_field).collect())
            }
            AstFragmentKind::GenericParams => {
                AstFragment::GenericParams(items.map(Annotatable::expect_generic_param).collect())
            }
            AstFragmentKind::Params => {
                AstFragment::Params(items.map(Annotatable::expect_param).collect())
            }
            AstFragmentKind::FieldDefs => {
                AstFragment::FieldDefs(items.map(Annotatable::expect_field_def).collect())
            }
            AstFragmentKind::Variants => {
                AstFragment::Variants(items.map(Annotatable::expect_variant).collect())
            }
            AstFragmentKind::WherePredicates => AstFragment::WherePredicates(
                items.map(Annotatable::expect_where_predicate).collect(),
            ),
            AstFragmentKind::Items => {
                AstFragment::Items(items.map(Annotatable::expect_item).collect())
            }
            AstFragmentKind::ImplItems => {
                AstFragment::ImplItems(items.map(Annotatable::expect_impl_item).collect())
            }
            AstFragmentKind::TraitImplItems => {
                AstFragment::TraitImplItems(items.map(Annotatable::expect_impl_item).collect())
            }
            AstFragmentKind::TraitItems => {
                AstFragment::TraitItems(items.map(Annotatable::expect_trait_item).collect())
            }
            AstFragmentKind::ForeignItems => {
                AstFragment::ForeignItems(items.map(Annotatable::expect_foreign_item).collect())
            }
            AstFragmentKind::Stmts => {
                AstFragment::Stmts(items.map(Annotatable::expect_stmt).collect())
            }
            AstFragmentKind::Expr => AstFragment::Expr(
                items.next().expect("expected exactly one expression").expect_expr(),
            ),
            AstFragmentKind::MethodReceiverExpr => AstFragment::MethodReceiverExpr(
                items.next().expect("expected exactly one expression").expect_expr(),
            ),
            AstFragmentKind::OptExpr => {
                AstFragment::OptExpr(items.next().map(Annotatable::expect_expr))
            }
            AstFragmentKind::Crate => {
                AstFragment::Crate(items.next().expect("expected exactly one crate").expect_crate())
            }
            AstFragmentKind::Pat | AstFragmentKind::Ty => {
                panic!("patterns and types aren't annotatable")
            }
        }
    }
}

enum AddSemicolon {
    Yes,
    No,
}

/// Required for `visit_node` obtained an owned `Node` from `&mut Node`.
trait DummyAstNode {
    fn dummy() -> Self;
}

impl DummyAstNode for ast::Crate {
    fn dummy() -> Self {
        ast::Crate {
            attrs: Default::default(),
            items: Default::default(),
            spans: Default::default(),
            id: DUMMY_NODE_ID,
            is_placeholder: Default::default(),
        }
    }
}

impl DummyAstNode for ast::Ty {
    fn dummy() -> Self {
        ast::Ty {
            id: DUMMY_NODE_ID,
            kind: TyKind::Dummy,
            span: Default::default(),
            tokens: Default::default(),
        }
    }
}

impl DummyAstNode for ast::Pat {
    fn dummy() -> Self {
        ast::Pat {
            id: DUMMY_NODE_ID,
            kind: PatKind::Wild,
            span: Default::default(),
            tokens: Default::default(),
        }
    }
}

impl DummyAstNode for ast::Expr {
    fn dummy() -> Self {
        ast::Expr::dummy()
    }
}

impl DummyAstNode for AstNodeWrapper<ast::Expr, MethodReceiverTag> {
    fn dummy() -> Self {
        AstNodeWrapper::new(ast::Expr::dummy(), MethodReceiverTag)
    }
}

// Add the SmallVec DummyAstNode implementation here
impl<T, const N: usize> DummyAstNode for SmallVec<T, N> {
    fn dummy() -> Self {
        SmallVec::new()
    }
}

impl DummyAstNode for Box<ast::Ty> {
    fn dummy() -> Self {
        Box::new(ast::Ty::dummy())
    }
}

impl DummyAstNode for Box<ast::Pat> {
    fn dummy() -> Self {
        Box::new(ast::Pat::dummy())
    }
}

impl DummyAstNode for Box<ast::Expr> {
    fn dummy() -> Self {
        Box::new(ast::Expr::dummy())
    }
}

impl DummyAstNode for Option<Box<ast::Expr>> {
    fn dummy() -> Self {
        None
    }
}

impl DummyAstNode for AstNodeWrapper<Box<ast::Expr>, MethodReceiverTag> {
    fn dummy() -> Self {
        AstNodeWrapper::new(Box::new(ast::Expr::dummy()), MethodReceiverTag)
    }
}

/// A trait implemented for all `AstFragment` nodes and providing all pieces
/// of functionality used by `InvocationCollector`.
trait InvocationCollectorNode: HasAttrs + HasNodeId + Sized {
    type OutputTy: DummyAstNode;
    type ItemKind = ItemKind;
    const KIND: AstFragmentKind;
    fn to_annotatable(self) -> Annotatable;
    fn fragment_to_output(fragment: AstFragment) -> Self::OutputTy;
    fn descr() -> &'static str { unreachable!() }
    fn walk_flat_map(self, _collector: &mut InvocationCollector<'_, '_>) -> Self::OutputTy { unreachable!() }
    fn walk(&mut self, _collector: &mut InvocationCollector<'_, '_>) { unreachable!() }
    fn is_mac_call(&self) -> bool { false }
    fn take_mac_call(self) -> (Box<ast::MacCall>, ast::AttrVec, AddSemicolon) { unreachable!() }
    fn delegation(&self) -> Option<(&ast::DelegationMac, &ast::Item<Self::ItemKind>)> { None }
    fn delegation_item_kind(_deleg: Box<ast::Delegation>) -> Self::ItemKind { unreachable!() }
    fn from_item(_item: ast::Item<Self::ItemKind>) -> Self { unreachable!() }
    fn flatten_outputs(_outputs: impl Iterator<Item = Self::OutputTy>) -> Self::OutputTy { unreachable!() }
    fn pre_flat_map_node_collect_attr(_cfg: &StripUnconfigured<'_>, _attr: &ast::Attribute) {}
    fn post_flat_map_node_collect_bang(_output: &mut Self::OutputTy, _add_semicolon: AddSemicolon) {}
    fn wrap_flat_map_node_walk_flat_map(
        node: Self,
        collector: &mut InvocationCollector<'_, '_>,
        walk_flat_map: impl FnOnce(Self, &mut InvocationCollector<'_, '_>) -> Self::OutputTy,
    ) -> Result<Self::OutputTy, Self> {
        Ok(walk_flat_map(node, collector))
    }
    fn expand_cfg_false(
        &mut self,
        collector: &mut InvocationCollector<'_, '_>,
        _pos: usize,
        span: Span,
    ) {
        collector.cx.dcx().emit_err(RemoveNodeNotSupported { span, descr: Self::descr() });
    }

    /// All of the identifiers (items) declared by this node.
    /// This is an approximation and should only be used for diagnostics.
    fn declared_idents(&self) -> Vec<Ident> { vec![] }
}

impl InvocationCollectorNode for Box<ast::Item> {
    type OutputTy = SmallVec<Box<ast::Item>, 1>;
    const KIND: AstFragmentKind = AstFragmentKind::Items;
    fn to_annotatable(self) -> Annotatable {
        Annotatable::Item(self)
    }
    fn fragment_to_output(fragment: AstFragment) -> Self::OutputTy {
        fragment.make_items()
    }
    fn walk_flat_map(self, collector: &mut InvocationCollector<'_, '_>) -> Self::OutputTy {
        walk_flat_map_item(collector, self)
    }
    fn is_mac_call(&self) -> bool {
        matches!(self.kind, ItemKind::MacCall(..))
    }
    fn take_mac_call(self) -> (Box<ast::MacCall>, ast::AttrVec, AddSemicolon) {
        match self.kind {
            ItemKind::MacCall(mac) => (mac, self.attrs, AddSemicolon::No),
            _ => unreachable!(),
        }
    }
    fn delegation(&self) -> Option<(&ast::DelegationMac, &ast::Item<Self::ItemKind>)> {
        match &self.kind {
            ItemKind::DelegationMac(deleg) => Some((deleg, self)),
            _ => None,
        }
    }
    fn delegation_item_kind(deleg: Box<ast::Delegation>) -> Self::ItemKind {
        ItemKind::Delegation(deleg)
    }
    fn from_item(item: ast::Item<Self::ItemKind>) -> Self {
        Box::new(item)
    }
    fn flatten_outputs(items: impl Iterator<Item = Self::OutputTy>) -> Self::OutputTy {
        items.flatten().collect()
    }
    fn pre_flat_map_node_collect_attr(cfg: &StripUnconfigured<'_>, attr: &ast::Attribute) {
        cfg.maybe_emit_expr_attr_err(attr);
    }
    fn post_flat_map_node_collect_bang(stmts: &mut Self::OutputTy, add_semicolon: AddSemicolon) {
        // If this is a macro invocation with a semicolon, then apply that
        // semicolon to the final statement produced by expansion.
        if matches!(add_semicolon, AddSemicolon::Yes) {
            if let Some(stmt) = stmts.pop() {
                stmts.push(stmt.add_trailing_semicolon());
            }
        }
    }
    fn wrap_flat_map_node_walk_flat_map(
        mut node: Self,
        collector: &mut InvocationCollector<'_, '_>,
        walk_flat_map: impl FnOnce(Self, &mut InvocationCollector<'_, '_>) -> Self::OutputTy,
    ) -> Result<Self::OutputTy, Self> {
        if !matches!(node.kind, ItemKind::Mod(..)) {
            return Ok(walk_flat_map(node, collector));
        }

        // Work around borrow checker not seeing through `P`'s deref.
        let (span, mut attrs) = (node.span, mem::take(&mut node.attrs));
        let ItemKind::Mod(_, ident, ref mut mod_kind) = node.kind else { unreachable!() };
        let ecx = &mut collector.cx;
        let (file_path, dir_path, dir_ownership) = match mod_kind {
            ModKind::Loaded(_, inline, _) => {
                // Inline `mod foo { ... }`, but we still need to push directories.
                let (dir_path, dir_ownership) = mod_dir_path(
                    ecx.sess,
                    ident,
                    &attrs,
                    &ecx.current_expansion.module,
                    ecx.current_expansion.dir_ownership,
                    *inline,
                );
                // If the module was parsed from an external file, recover its path.
                // This lets `parse_external_mod` catch cycles if it's self-referential.
                let file_path = match inline {
                    Inline::Yes => None,
                    Inline::No { .. } => mod_file_path_from_attr(ecx.sess, &attrs, &dir_path),
                };
                node.attrs = attrs;
                (file_path, dir_path, dir_ownership)
            }
            ModKind::Unloaded => {
                // We have an outline `mod foo;` so we need to parse the file.
                let old_attrs_len = attrs.len();
                let ParsedExternalMod { 
                    items, 
                    spans, 
                    file_path, 
                    dir_path, 
                    dir_ownership, 
                    had_parse_error, 
                } = parse_external_mod(
                    ecx.sess,
                    ident,
                    span,
                    &ecx.current_expansion.module,
                    ecx.current_expansion.dir_ownership,
                    &mut attrs,
                );

                if let Some(lint_store) = ecx.lint_store {
                    lint_store.pre_expansion_lint(
                        ecx.sess,
                        ecx.ecfg.features,
                        ecx.resolver.registered_tools(),
                        ecx.current_expansion.lint_node_id,
                        &attrs,
                        &items,
                        ident.name,
                    );
                }

                *mod_kind = ModKind::Loaded(items, Inline::No { had_parse_error }, spans);
                node.attrs = attrs;
                if node.attrs.len() > old_attrs_len {
                    // If we loaded an out-of-line module and added some inner attributes, 
                    // then we need to re-configure it and re-collect attributes for
                    // resolution and expansion.
                    return Err(node);
                }
                (Some(file_path), dir_path, dir_ownership)
            }
        };

        // Set the module info before we flat map.
        let mut module = ecx.current_expansion.module.with_dir_path(dir_path);
        module.mod_path.push(ident);
        if let Some(file_path) = file_path {
            module.file_path_stack.push(file_path);
        }

        let orig_module = mem::replace(&mut ecx.current_expansion.module, Rc::new(module));
        let orig_dir_ownership =
            mem::replace(&mut ecx.current_expansion.dir_ownership, dir_ownership);

        let res = Ok(walk_flat_map(node, collector));

        collector.cx.current_expansion.dir_ownership = orig_dir_ownership;
        collector.cx.current_expansion.module = orig_module;
        res
    }

    fn declared_idents(&self) -> Vec<Ident> {
        if let ItemKind::Use(ut) = &self.kind {
            fn collect_use_tree_leaves(ut: &ast::UseTree, idents: &mut Vec<Ident>) {
                match &ut.kind {
                    ast::UseTreeKind::Glob => {} // Glob imports don't declare specific idents.
                    ast::UseTreeKind::Simple(_) => idents.push(ut.ident()),
                    ast::UseTreeKind::Nested { items, .. } => {
                        for (ut, _) in items {
                            collect_use_tree_leaves(ut, idents);
                        }
                    }
                }
            }
            let mut idents = Vec::new();
            collect_use_tree_leaves(ut, &mut idents);
            idents
        } else {
            self.kind.ident().into_iter().collect()
        }
    }
}

struct TraitItemTag;
impl InvocationCollectorNode for AstNodeWrapper<Box<ast::AssocItem>, TraitItemTag> {
    type OutputTy = SmallVec<Box<ast::AssocItem>, 1>;
    type ItemKind = AssocItemKind;
    const KIND: AstFragmentKind = AstFragmentKind::TraitItems;
    fn to_annotatable(self) -> Annotatable {
        Annotatable::AssocItem(self.wrapped, AssocCtxt::Trait)
    }
    fn fragment_to_output(fragment: AstFragment) -> Self::OutputTy {
        fragment.make_trait_items()
    }
    fn walk_flat_map(self, collector: &mut InvocationCollector<'_, '_>) -> Self::OutputTy {
        walk_flat_map_assoc_item(collector, self.wrapped, AssocCtxt::Trait)
    }
    fn is_mac_call(&self) -> bool {
        matches!(self.wrapped.kind, AssocItemKind::MacCall(..))
    }
    fn take_mac_call(self) -> (Box<ast::MacCall>, ast::AttrVec, AddSemicolon) {
        let item = self.wrapped;
        match item.kind {
            AssocItemKind::MacCall(mac) => (mac, item.attrs, AddSemicolon::No),
            _ => unreachable!(),
        }
    }
    fn delegation(&self) -> Option<(&ast::DelegationMac, &ast::Item<Self::ItemKind>)> {
        match &self.wrapped.kind {
            AssocItemKind::DelegationMac(deleg) => Some((deleg, &self.wrapped)),
            _ => None,
        }
    }
    fn delegation_item_kind(deleg: Box<ast::Delegation>) -> Self::ItemKind {
        AssocItemKind::Delegation(deleg)
    }
    fn from_item(item: ast::Item<Self::ItemKind>) -> Self {
        AstNodeWrapper::new(Box::new(item), TraitItemTag)
    }
    fn flatten_outputs(items: impl Iterator<Item = Self::OutputTy>) -> Self::OutputTy {
        items.flatten().collect()
    }
}

struct ImplItemTag;
impl InvocationCollectorNode for AstNodeWrapper<Box<ast::AssocItem>, ImplItemTag> {
    type OutputTy = SmallVec<Box<ast::AssocItem>, 1>;
    type ItemKind = AssocItemKind;
    const KIND: AstFragmentKind = AstFragmentKind::ImplItems;
    fn to_annotatable(self) -> Annotatable {
        Annotatable::AssocItem(self.wrapped, AssocCtxt::Impl { of_trait: false })
    }
    fn fragment_to_output(fragment: AstFragment) -> Self::OutputTy {
        fragment.make_impl_items()
    }
    fn walk_flat_map(self, collector: &mut InvocationCollector<'_, '_>) -> Self::OutputTy {
        walk_flat_map_assoc_item(collector, self.wrapped, AssocCtxt::Impl { of_trait: false })
    }
    fn is_mac_call(&self) -> bool {
        matches!(self.wrapped.kind, AssocItemKind::MacCall(..))
    }
    fn take_mac_call(self) -> (Box<ast::MacCall>, ast::AttrVec, AddSemicolon) {
        let item = self.wrapped;
        match item.kind {
            AssocItemKind::MacCall(mac) => (mac, item.attrs, AddSemicolon::No),
            _ => unreachable!(),
        }
    }
    fn delegation(&self) -> Option<(&ast::DelegationMac, &ast::Item<Self::ItemKind>)> {
        match &self.wrapped.kind {
            AssocItemKind::DelegationMac(deleg) => Some((deleg, &self.wrapped)),
            _ => None,
        }
    }
    fn delegation_item_kind(deleg: Box<ast::Delegation>) -> Self::ItemKind {
        AssocItemKind::Delegation(deleg)
    }
    fn from_item(item: ast::Item<Self::ItemKind>) -> Self {
        AstNodeWrapper::new(Box::new(item), ImplItemTag)
    }
    fn flatten_outputs(items: impl Iterator<Item = Self::OutputTy>) -> Self::OutputTy {
        items.flatten().collect()
    }
}

struct TraitImplItemTag;
impl InvocationCollectorNode for AstNodeWrapper<Box<ast::AssocItem>, TraitImplItemTag> {
    type OutputTy = SmallVec<Box<ast::AssocItem>, 1>;
    type ItemKind = AssocItemKind;
    const KIND: AstFragmentKind = AstFragmentKind::TraitImplItems;
    fn to_annotatable(self) -> Annotatable {
        Annotatable::AssocItem(self.wrapped, AssocCtxt::Impl { of_trait: true })
    }
    fn fragment_to_output(fragment: AstFragment) -> Self::OutputTy {
        fragment.make_trait_impl_items()
    }
    fn walk_flat_map(self, collector: &mut InvocationCollector<'_, '_>) -> Self::OutputTy {
        walk_flat_map_assoc_item(collector, self.wrapped, AssocCtxt::Impl { of_trait: true })
    }
    fn is_mac_call(&self) -> bool {
        matches!(self.wrapped.kind, AssocItemKind::MacCall(..))
    }
    fn take_mac_call(self) -> (Box<ast::MacCall>, ast::AttrVec, AddSemicolon) {
        let item = self.wrapped;
        match item.kind {
            AssocItemKind::MacCall(mac) => (mac, item.attrs, AddSemicolon::No),
            _ => unreachable!(),
        }
    }
    fn delegation(&self) -> Option<(&ast::DelegationMac, &ast::Item<Self::ItemKind>)> {
        match &self.wrapped.kind {
            AssocItemKind::DelegationMac(deleg) => Some((deleg, &self.wrapped)),
            _ => None,
        }
    }
    fn delegation_item_kind(deleg: Box<ast::Delegation>) -> Self::ItemKind {
        AssocItemKind::Delegation(deleg)
    }
    fn from_item(item: ast::Item<Self::ItemKind>) -> Self {
        AstNodeWrapper::new(Box::new(item), TraitImplItemTag)
    }
    fn flatten_outputs(items: impl Iterator<Item = Self::OutputTy>) -> Self::OutputTy {
        items.flatten().collect()
    }
}

impl InvocationCollectorNode for Box<ast::ForeignItem> {
    type OutputTy = SmallVec<Box<ast::ForeignItem>, 1>;
    const KIND: AstFragmentKind = AstFragmentKind::ForeignItems;
    fn to_annotatable(self) -> Annotatable {
        Annotatable::ForeignItem(self)
    }
    fn fragment_to_output(fragment: AstFragment) -> Self::OutputTy {
        fragment.make_foreign_items()
    }
    fn walk_flat_map(self, collector: &mut InvocationCollector<'_, '_>) -> Self::OutputTy {
        walk_flat_map_foreign_item(collector, self)
    }
    fn is_mac_call(&self) -> bool {
        matches!(self.kind, ForeignItemKind::MacCall(..))
    }
    fn take_mac_call(self) -> (Box<ast::MacCall>, ast::AttrVec, AddSemicolon) {
        match self.kind {
            ForeignItemKind::MacCall(mac) => (mac, self.attrs, AddSemicolon::No),
            _ => unreachable!(),
        }
    }
}

impl InvocationCollectorNode for ast::Variant {
    type OutputTy = SmallVec<ast::Variant, 1>;
    const KIND: AstFragmentKind = AstFragmentKind::Variants;
    fn to_annotatable(self) -> Annotatable {
        Annotatable::Variant(self)
    }
    fn fragment_to_output(fragment: AstFragment) -> Self::OutputTy {
        fragment.make_variants()
    }
    fn walk_flat_map(self, collector: &mut InvocationCollector<'_, '_>) -> Self::OutputTy {
        walk_flat_map_variant(collector, self)
    }
}

impl InvocationCollectorNode for ast::WherePredicate {
    type OutputTy = SmallVec<ast::WherePredicate, 1>;
    const KIND: AstFragmentKind = AstFragmentKind::WherePredicates;
    fn to_annotatable(self) -> Annotatable {
        Annotatable::WherePredicate(self)
    }
    fn fragment_to_output(fragment: AstFragment) -> Self::OutputTy {
        fragment.make_where_predicates()
    }
    fn walk_flat_map(self, collector: &mut InvocationCollector<'_, '_>) -> Self::OutputTy {
        walk_flat_map_where_predicate(collector, self)
    }
}

impl InvocationCollectorNode for ast::FieldDef {
    type OutputTy = SmallVec<ast::FieldDef, 1>;
    const KIND: AstFragmentKind = AstFragmentKind::FieldDefs;
    fn to_annotatable(self) -> Annotatable {
        Annotatable::FieldDef(self)
    }
    fn fragment_to_output(fragment: AstFragment) -> Self::OutputTy {
        fragment.make_field_defs()
    }
    fn walk_flat_map(self, collector: &mut InvocationCollector<'_, '_>) -> Self::OutputTy {
        walk_flat_map_field_def(collector, self)
    }
}

impl InvocationCollectorNode for ast::PatField {
    type OutputTy = SmallVec<ast::PatField, 1>;
    const KIND: AstFragmentKind = AstFragmentKind::PatFields;
    fn to_annotatable(self) -> Annotatable {
        Annotatable::PatField(self)
    }
    fn fragment_to_output(fragment: AstFragment) -> Self::OutputTy {
        fragment.make_pat_fields()
    }
    fn walk_flat_map(self, collector: &mut InvocationCollector<'_, '_>) -> Self::OutputTy {
        walk_flat_map_pat_field(collector, self)
    }
}

impl InvocationCollectorNode for ast::ExprField {
    type OutputTy = SmallVec<ast::ExprField, 1>;
    const KIND: AstFragmentKind = AstFragmentKind::ExprFields;
    fn to_annotatable(self) -> Annotatable {
        Annotatable::ExprField(self)
    }
    fn fragment_to_output(fragment: AstFragment) -> Self::OutputTy {
        fragment.make_expr_fields()
    }
    fn walk_flat_map(self, collector: &mut InvocationCollector<'_, '_>) -> Self::OutputTy {
        walk_flat_map_expr_field(collector, self)
    }
}

impl InvocationCollectorNode for ast::Param {
    type OutputTy = SmallVec<ast::Param, 1>;
    const KIND: AstFragmentKind = AstFragmentKind::Params;
    fn to_annotatable(self) -> Annotatable {
        Annotatable::Param(self)
    }
    fn fragment_to_output(fragment: AstFragment) -> Self::OutputTy {
        fragment.make_params()
    }
    fn walk_flat_map(self, collector: &mut InvocationCollector<'_, '_>) -> Self::OutputTy {
        walk_flat_map_param(collector, self)
    }
}

impl InvocationCollectorNode for ast::GenericParam {
    type OutputTy = SmallVec<ast::GenericParam, 1>;
    const KIND: AstFragmentKind = AstFragmentKind::GenericParams;
    fn to_annotatable(self) -> Annotatable {
        Annotatable::GenericParam(self)
    }
    fn fragment_to_output(fragment: AstFragment) -> Self::OutputTy {
        fragment.make_generic_params()
    }
    fn walk_flat_map(self, collector: &mut InvocationCollector<'_, '_>) -> Self::OutputTy {
        walk_flat_map_generic_param(collector, self)
    }
}

impl InvocationCollectorNode for ast::Arm {
    type OutputTy = SmallVec<ast::Arm, 1>;
    const KIND: AstFragmentKind = AstFragmentKind::Arms;
    fn to_annotatable(self) -> Annotatable {
        Annotatable::Arm(self)
    }
    fn fragment_to_output(fragment: AstFragment) -> Self::OutputTy {
        fragment.make_arms()
    }
    fn walk_flat_map(self, collector: &mut InvocationCollector<'_, '_>) -> Self::OutputTy {
        walk_flat_map_arm(collector, self)
    }
}

impl InvocationCollectorNode for ast::Stmt {
    type OutputTy = SmallVec<ast::Stmt, 1>;
    const KIND: AstFragmentKind = AstFragmentKind::Stmts;
    fn to_annotatable(self) -> Annotatable {
        Annotatable::Stmt(Box::new(self))
    }
    fn fragment_to_output(fragment: AstFragment) -> Self::OutputTy {
        fragment.make_stmts()
    }
    fn walk_flat_map(self, collector: &mut InvocationCollector<'_, '_>) -> Self::OutputTy {
        walk_flat_map_stmt(collector, self)
    }
    fn is_mac_call(&self) -> bool {
        matches!(self.kind, StmtKind::MacCall(..) | StmtKind::Item(ast::Item { kind: ItemKind::MacCall(..), .. }))
    }
    fn take_mac_call(self) -> (Box<ast::MacCall>, ast::AttrVec, AddSemicolon) {
        let (add_semicolon, mac, attrs) = match self.kind {
            StmtKind::MacCall(mac) => {
                let ast::MacCallStmt { mac, style, attrs, .. } = *mac;
                (style == MacStmtStyle::Semicolon, mac, attrs)
            }
            StmtKind::Item(item) => match *item {
                ast::Item { kind: ItemKind::MacCall(mac), attrs, .. } => {
                    (mac.args.need_semicolon(), mac, attrs)
                }
                _ => unreachable!(),
            },
            StmtKind::Semi(expr) => match *expr {
                ast::Expr { kind: ExprKind::MacCall(mac), attrs, .. } => {
                    (mac.args.need_semicolon(), mac, attrs)
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };
        (mac, attrs, if add_semicolon { AddSemicolon::Yes } else { AddSemicolon::No })
    }
    fn delegation(&self) -> Option<(&ast::DelegationMac, &ast::Item<Self::ItemKind>)> {
        match &self.kind {
            StmtKind::Item(item) => match &item.kind {
                ItemKind::DelegationMac(deleg) => Some((deleg, item)),
                _ => None,
            },
            _ => None,
        }
    }
    fn delegation_item_kind(deleg: Box<ast::Delegation>) -> Self::ItemKind {
        ItemKind::Delegation(deleg)
    }
    fn from_item(item: ast::Item<Self::ItemKind>) -> Self {
        ast::Stmt { id: ast::DUMMY_NODE_ID, span: item.span, kind: StmtKind::Item(Box::new(item)) }
    }
    fn flatten_outputs(items: impl Iterator<Item = Self::OutputTy>) -> Self::OutputTy {
        items.flatten().collect()
    }
    fn post_flat_map_node_collect_bang(stmts: &mut Self::OutputTy, add_semicolon: AddSemicolon) {
        // If this is a macro invocation with a semicolon, then apply that
        // semicolon to the final statement produced by expansion.
        if matches!(add_semicolon, AddSemicolon::Yes) {
            if let Some(stmt) = stmts.pop() {
                stmts.push(stmt.add_trailing_semicolon());
            }
        }
    }
}

impl InvocationCollectorNode for ast::Crate {
    type OutputTy = ast::Crate;
    const KIND: AstFragmentKind = AstFragmentKind::Crate;
    fn to_annotatable(self) -> Annotatable {
        Annotatable::Crate(self)
    }
    fn fragment_to_output(fragment: AstFragment) -> Self::OutputTy {
        fragment.make_crate()
    }
    fn walk(&mut self, collector: &mut InvocationCollector<'_, '_>) {
        walk_crate(collector, self)
    }
    fn expand_cfg_false(
        &mut self,
        collector: &mut InvocationCollector<'_, '_>,
        _pos: usize,
        _span: Span,
    ) {
        // Attributes above `cfg(FALSE)` are left in place, because we may want to configure
        // some global crate properties even on fully unconfigured crates.
        self.attrs.truncate(_pos);
        // Standard prelude imports are left in the crate for backward compatibility.
        self.items.truncate(collector.cx.num_standard_library_imports);
    }
}

impl InvocationCollectorNode for ast::Ty {
    type OutputTy = Box<ast::Ty>;
    const KIND: AstFragmentKind = AstFragmentKind::Ty;
    fn to_annotatable(self) -> Annotatable {
        unreachable!()
    }
    fn fragment_to_output(fragment: AstFragment) -> Self::OutputTy {
        fragment.make_ty()
    }
    fn walk(&mut self, collector: &mut InvocationCollector<'_, '_>) {
        // Save the pre-expanded name of this `ImplTrait`, so that later when defining
        // an APIT we use a name that doesn't have any placeholder fragments in it.
        if let ast::TyKind::ImplTrait(..) = self.kind {
            // HACK: pprust breaks strings with newlines when the type
            // gets too long. We don't want these to show up in compiler
            // output or built artifacts, so replace them here...
            // Perhaps we should instead format APITs more robustly.
            let name = Symbol::intern(&pprust::ty_to_string(self).replace('\n', " "));
            collector.cx.resolver.insert_impl_trait_name(self.id, name);
        }
        walk_ty(collector, self)
    }
    fn is_mac_call(&self) -> bool {
        matches!(self.kind, ast::TyKind::MacCall(..))
    }
    fn take_mac_call(self) -> (Box<ast::MacCall>, ast::AttrVec, AddSemicolon) {
        match self.kind {
            TyKind::MacCall(mac) => (mac, AttrVec::new(), AddSemicolon::No),
            _ => unreachable!(),
        }
    }
}

impl InvocationCollectorNode for ast::Pat {
    type OutputTy = Box<ast::Pat>;
    const KIND: AstFragmentKind = AstFragmentKind::Pat;
    fn to_annotatable(self) -> Annotatable {
        unreachable!()
    }
    fn fragment_to_output(fragment: AstFragment) -> Self::OutputTy {
        fragment.make_pat()
    }
    fn walk(&mut self, collector: &mut InvocationCollector<'_, '_>) {
        walk_pat(collector, self)
    }
    fn is_mac_call(&self) -> bool {
        matches!(self.kind, PatKind::MacCall(..))
    }
    fn take_mac_call(self) -> (Box<ast::MacCall>, ast::AttrVec, AddSemicolon) {
        match self.kind {
            PatKind::MacCall(mac) => (mac, AttrVec::new(), AddSemicolon::No),
            _ => unreachable!(),
        }
    }
}

impl InvocationCollectorNode for ast::Expr {
    type OutputTy = Box<ast::Expr>;
    const KIND: AstFragmentKind = AstFragmentKind::Expr;
    fn to_annotatable(self) -> Annotatable {
        Annotatable::Expr(Box::new(self))
    }
    fn fragment_to_output(fragment: AstFragment) -> Self::OutputTy {
        fragment.make_expr()
    }
    fn descr() -> &'static str {
        "an expression"
    }
    fn walk(&mut self, collector: &mut InvocationCollector<'_, '_>) {
        walk_expr(collector, self)
    }
    fn is_mac_call(&self) -> bool {
        matches!(self.kind, ExprKind::MacCall(..))
    }
    fn take_mac_call(self) -> (Box<ast::MacCall>, ast::AttrVec, AddSemicolon) {
        match self.kind {
            ExprKind::MacCall(mac) => (mac, self.attrs, AddSemicolon::No),
            _ => unreachable!(),
        }
    }
}

struct OptExprTag;
impl InvocationCollectorNode for AstNodeWrapper<Box<ast::Expr>, OptExprTag> {
    type OutputTy = Option<Box<ast::Expr>>;
    const KIND: AstFragmentKind = AstFragmentKind::OptExpr;
    fn to_annotatable(self) -> Annotatable {
        Annotatable::Expr(self.wrapped)
    }
    fn fragment_to_output(fragment: AstFragment) -> Self::OutputTy {
        fragment.make_opt_expr()
    }
    fn walk_flat_map(mut self, collector: &mut InvocationCollector<'_, '_>) -> Self::OutputTy {
        walk_expr(collector, &mut self.wrapped);
        Some(self.wrapped)
    }
    fn is_mac_call(&self) -> bool {
        matches!(self.wrapped.kind, ast::ExprKind::MacCall(..))
    }
    fn take_mac_call(self) -> (Box<ast::MacCall>, ast::AttrVec, AddSemicolon) {
        let node = self.wrapped;
        match node.kind {
            ExprKind::MacCall(mac) => (mac, node.attrs, AddSemicolon::No),
            _ => unreachable!(),
        }
    }
    fn pre_flat_map_node_collect_attr(cfg: &StripUnconfigured<'_>, attr: &ast::Attribute) {
        cfg.maybe_emit_expr_attr_err(attr);
    }
}

/// This struct is a hack to workaround unstable of `stmt_expr_attributes`.
/// It can be removed once that feature is stabilized.
struct MethodReceiverTag;

impl InvocationCollectorNode for AstNodeWrapper<ast::Expr, MethodReceiverTag> {
    type OutputTy = AstNodeWrapper<Box<ast::Expr>, MethodReceiverTag>;
    const KIND: AstFragmentKind = AstFragmentKind::MethodReceiverExpr;
    fn descr() -> &'static str {
        "an expression"
    }
    fn to_annotatable(self) -> Annotatable {
        Annotatable::Expr(Box::new(self.wrapped))
    }
    fn fragment_to_output(fragment: AstFragment) -> Self::OutputTy {
        AstNodeWrapper::new(fragment.make_method_receiver_expr(), MethodReceiverTag)
    }
    fn walk(&mut self, collector: &mut InvocationCollector<'_, '_>) {
        walk_expr(collector, &mut self.wrapped)
    }
    fn is_mac_call(&self) -> bool {
        matches!(self.wrapped.kind, ast::ExprKind::MacCall(..))
    }
    fn take_mac_call(self) -> (Box<ast::MacCall>, ast::AttrVec, AddSemicolon) {
        let node = self.wrapped;
        match node.kind {
            ExprKind::MacCall(mac) => (mac, node.attrs, AddSemicolon::No),
            _ => unreachable!(),
        }
    }
}

fn build_single_delegations<'a, Node: InvocationCollectorNode>(
    ecx: &ExtCtxt<'_>,
    deleg: &'a ast::DelegationMac,
    item: &'a ast::Item<Node::ItemKind>,
    suffixes: &'a [(Ident, Option<Ident>)],
    item_span: Span,
    from_glob: bool,
) -> impl Iterator<Item = ast::Item<Node::ItemKind>> + 'a {
    if suffixes.is_empty() {
        // Report an error for now, to avoid keeping stem for resolution and
        // stability checks.
        let kind = String::from(if from_glob { "glob" } else { "list" });
        ecx.dcx().emit_err(EmptyDelegationMac { span: item.span, kind });
    }

    suffixes.iter().map(move |&(ident, rename)| {
        let mut path = deleg.prefix.clone();
        path.segments.push(ast::PathSegment { ident, id: ast::DUMMY_NODE_ID, args: None });

        ast::Item {
            attrs: item.attrs.clone(),
            id: ast::DUMMY_NODE_ID,
            span: if from_glob { item_span } else { ident.span },
            vis: item.vis.clone(),
            kind: Node::delegation_item_kind(Box::new(ast::Delegation {
                id: ast::DUMMY_NODE_ID,
                qself: deleg.qself.clone(),
                path,
                ident: rename.unwrap_or(ident),
                rename,
                body: deleg.body.clone(),
                from_glob,
            })),
            tokens: None,
        }
    })
}



struct InvocationCollector<'a, 'b> {
    cx: &'a mut ExtCtxt<'b>,
    invocations: Vec<(Invocation, Option<Arc<SyntaxExtension>>)>, 
    monotonic: bool,
}

impl<'a, 'b> InvocationCollector<'a, 'b> {
    fn cfg(&self) -> StripUnconfigured<'_> {
        StripUnconfigured {
            sess: self.cx.sess,
            features: Some(self.cx.ecfg.features),
            config_tokens: false,
            lint_node_id: self.cx.current_expansion.lint_node_id,
        }
    }

    fn collect(&mut self, fragment_kind: AstFragmentKind, kind: InvocationKind) -> AstFragment {
        let expn_id = LocalExpnId::fresh_empty();
        if matches!(kind, InvocationKind::GlobDelegation { .. }) {
            // In resolver we need to know which invocation ids are delegations early, 
            // before their `ExpnData` is filled.
            self.cx.resolver.register_glob_delegation(expn_id);
        }
        let vis = fragment_kind.placeholder_visibility();
        self.invocations.push(( 
            Invocation {
                kind,
                fragment_kind,
                expansion_data: ExpansionData {
                    id: expn_id,
                    depth: self.cx.current_expansion.depth + 1,
                    ..self.cx.current_expansion.clone()
                },
            },
            None,
        ));
        placeholder(fragment_kind, NodeId::placeholder_from_expn_id(expn_id), vis)
    }

    fn collect_bang(&mut self, mac: Box<ast::MacCall>, kind: AstFragmentKind) -> AstFragment {
        // cache the macro call span so that it can be
        // easily adjusted for incremental compilation
        let span = mac.span();
        self.collect(kind, InvocationKind::Bang { mac, span })
    }

    fn collect_attr(
        &mut self,
        (attr, pos, derives): (ast::Attribute, usize, Vec<ast::Path>),
        item: Annotatable,
        kind: AstFragmentKind,
    ) -> AstFragment {
        self.collect(kind, InvocationKind::Attr { attr, pos, item, derives })
    }

    fn collect_glob_delegation(
        &mut self,
        item: Box<ast::AssocItem>,
        of_trait: bool,
        kind: AstFragmentKind,
    ) -> AstFragment {
        self.collect(kind, InvocationKind::GlobDelegation { item, of_trait })
    }

    /// If `item` is an attribute invocation, remove the attribute and return it together with
    /// its position and derives following it. We have to collect the derives in order to resolve
    /// legacy derive helpers (helpers written before derives that introduce them).
    fn take_first_attr(
        &self,
        item: &mut impl HasAttrs,
    ) -> Option<(ast::Attribute, usize, Vec<ast::Path>) >{
        let mut attr = None;

        let mut cfg_pos = None;
        let mut attr_pos = None;
        for (pos, attr) in item.attrs().iter().enumerate() {
            if !attr.is_doc_comment() && !self.cx.expanded_inert_attrs.is_marked(attr) {
                let name = attr.ident().map(|ident| ident.name);
                if name == Some(sym::cfg) || name == Some(sym::cfg_attr) {
                    cfg_pos = Some(pos); // a cfg attr found, no need to search anymore
                    break;
                } else if attr_pos.is_none() 
                    && !name.is_some_and(rustc_feature::is_builtin_attr_name)
                {
                    attr_pos = Some(pos); // a non-cfg attr found, still may find a cfg attr
                }
            }
        }

        item.visit_attrs(|attrs| {
            attr = Some(match (cfg_pos, attr_pos) {
                (Some(pos), _) => (attrs.remove(pos), pos, Vec::new()),
                (_, Some(pos)) => {
                    let attr = attrs.remove(pos);
                    let following_derives = attrs[pos..]
                        .iter()
                        .filter(|a| a.has_name(sym::derive))
                        .flat_map(|a| a.meta_item_list().unwrap_or_default())
                        .filter_map(|meta_item_inner| match meta_item_inner {
                            MetaItemInner::MetaItem(ast::MetaItem {
                                kind: MetaItemKind::Word,
                                path,
                                ..
                            }) => Some(path),
                            _ => None,
                        })
                        .collect();

                    (attr, pos, following_derives)
                }
                _ => return,
            });
        });

        attr
    }

    // Detect use of feature-gated or invalid attributes on macro invocations
    // since they will not be detected after macro expansion.
    fn check_attributes(&self, attrs: &[ast::Attribute], call: &ast::MacCall) {
        let features = self.cx.ecfg.features;
        let mut attrs = attrs.iter().peekable();
        let mut span: Option<Span> = None;
        while let Some(attr) = attrs.next() {
            rustc_ast_passes::feature_gate::check_attribute(attr, self.cx.sess, features);
            validate_attr::check_attr(
                &self.cx.sess.psess,
                attr,
                self.cx.current_expansion.lint_node_id,
            );
            AttributeParser::parse_limited_all(
                self.cx.sess,
                slice::from_ref(attr),
                None,
                Target::MacroCall,
                call.span(),
                self.cx.current_expansion.lint_node_id,
                Some(self.cx.ecfg.features),
                ShouldEmit::ErrorsAndLints,
            );

            let current_span = if let Some(sp) = span { sp.to(attr.span) } else { attr.span };
            span = Some(current_span);

            if attrs.peek().is_some_and(|next_attr| next_attr.doc_str().is_some()) {
                continue;
            }

            if attr.is_doc_comment() {
                self.cx.sess.psess.buffer_lint(
                    UNUSED_DOC_COMMENTS,
                    current_span,
                    self.cx.current_expansion.lint_node_id,
                    crate::errors::MacroCallUnusedDocComment { span: attr.span },
                );
            } else if rustc_attr_parsing::is_builtin_attr(attr)
                && !AttributeParser::<Early>::is_parsed_attribute(&attr.path())
            {
                let attr_name = attr.ident().unwrap().name;
                // `#[cfg]` and `#[cfg_attr]` are special - they are
                // eagerly evaluated.
                if attr_name != sym::cfg_trace && attr_name != sym::cfg_attr_trace {
                    self.cx.sess.psess.buffer_lint(
                        UNUSED_ATTRIBUTES,
                        attr.span,
                        self.cx.current_expansion.lint_node_id,
                        crate::errors::UnusedBuiltinAttribute {
                            attr_name,
                            macro_name: pprust::path_to_string(&call.path),
                            invoc_span: call.path.span,
                            attr_span: attr.span,
                        },
                    );
                }
            }
        }
    }

    fn expand_cfg_true(
        &mut self,
        node: &mut (impl HasAttrs + HasNodeId),
        attr: ast::Attribute,
        pos: usize,
    ) -> EvalConfigResult {
        let res = self.cfg().cfg_true(&attr, node.node_id(), ShouldEmit::ErrorsAndLints);
        if res.as_bool() {
            // A trace attribute left in AST in place of the original `cfg` attribute.
            // It can later be used by lints or other diagnostics.
            let trace_attr = attr_into_trace(attr, sym::cfg_trace);
            node.visit_attrs(|attrs| attrs.insert(pos, trace_attr));
        }

        res
    }

    fn expand_cfg_attr(&self, node: &mut impl HasAttrs, attr: &ast::Attribute, pos: usize) {
        node.visit_attrs(|attrs| {
            // Repeated `insert` calls is inefficient, but the number of
            // insertions is almost always 0 or 1 in practice.
            for cfg in self.cfg().expand_cfg_attr(attr, false).into_iter().rev() {
                attrs.insert(pos, cfg)
            }
        });
    }

    fn flat_map_node<Node: InvocationCollectorNode<OutputTy: Default>>( 
        &mut self,
        mut node: Node,
    ) -> Node::OutputTy {
        loop {
            return match self.take_first_attr(&mut node) {
                Some((attr, pos, derives)) => match attr.name() {
                    Some(sym::cfg) => {
                        let res = self.expand_cfg_true(&mut node, attr, pos);
                        match res {
                            EvalConfigResult::True => continue,
                            EvalConfigResult::False { reason, reason_span } => {
                                for ident in node.declared_idents() {
                                    self.cx.resolver.append_stripped_cfg_item(
                                        self.cx.current_expansion.lint_node_id,
                                        ident,
                                        reason.clone(),
                                        reason_span,
                                    )
                                }
                            }
                        }

                        Node::OutputTy::dummy()
                    }
                    Some(sym::cfg_attr) => {
                        self.expand_cfg_attr(&mut node, &attr, pos);
                        continue;
                    }
                    _ => {
                        Node::pre_flat_map_node_collect_attr(&self.cfg(), &attr);
                        self.collect_attr((attr, pos, derives), node.to_annotatable(), Node::KIND)
                            .make_ast::<Node>()
                    }
                },
                None if node.is_mac_call() => {
                    let (mac, attrs, add_semicolon) = node.take_mac_call();
                    self.check_attributes(&attrs, &mac);
                    let mut res = self.collect_bang(mac, Node::KIND).make_ast::<Node>();
                    Node::post_flat_map_node_collect_bang(&mut res, add_semicolon);
                    res
                }
                None if let Some((deleg, item)) = node.delegation() => {
                    let Some(suffixes) = &deleg.suffixes else { 
                        let traitless_qself = 
                            matches!(&deleg.qself, Some(qself) if qself.position == 0);
                        let (item, of_trait) = match node.to_annotatable() {
                            Annotatable::AssocItem(item, AssocCtxt::Impl { of_trait }) => {
                                (item, of_trait)
                            }
                            ann @ (Annotatable::Item(_) 
                            | Annotatable::AssocItem(..)
                            | Annotatable::Stmt(_)) => {
                                let span = ann.span();
                                self.cx.dcx().emit_err(GlobDelegationOutsideImpls { span });
                                return Default::default();
                            }
                            _ => unreachable!(),
                        };
                        if traitless_qself {
                            let span = item.span;
                            self.cx.dcx().emit_err(GlobDelegationTraitlessQpath { span });
                            return Default::default();
                        }
                        return self
                            .collect_glob_delegation(item, of_trait, Node::KIND)
                            .make_ast::<Node>();
                    };

                    let single_delegations = build_single_delegations::<Node>( 
                        self.cx, deleg, item, suffixes, item.span, false,
                    );
                    Node::flatten_outputs(single_delegations.map(|item| {
                        let mut item = Node::from_item(item);
                        assign_id!(self, item.node_id_mut(), || item.walk_flat_map(self))
                    }))
                }
                None => {
                    match Node::wrap_flat_map_node_walk_flat_map(node, self, |mut node, this| {
                        assign_id!(this, node.node_id_mut(), || node.walk_flat_map(this))
                    }) {
                        Ok(output) => output,
                        Err(returned_node) => {
                            node = returned_node;
                            continue;
                        }
                    }
                }
            };
        }
    }

    fn visit_node<Node: InvocationCollectorNode<OutputTy: Into<Node>> + DummyAstNode>(
        &mut self,
        node: &mut Node,
    ) {
        loop {
            return match self.take_first_attr(node) {
                Some((attr, pos, derives)) => match attr.name() {
                    Some(sym::cfg) => {
                        let span = attr.span;
                        if self.expand_cfg_true(node, attr, pos).as_bool() {
                            continue;
                        }

                        node.expand_cfg_false(self, pos, span);
                        continue;
                    }
                    Some(sym::cfg_attr) => {
                        self.expand_cfg_attr(node, &attr, pos);
                        continue;
                    }
                    _ => {
                        let n = mem::replace(node, Node::dummy());
                        *node = self
                            .collect_attr((attr, pos, derives), n.to_annotatable(), Node::KIND)
                            .make_ast::<Node>()
                            .into()
                    }
                },
                None if node.is_mac_call() => {
                    let n = mem::replace(node, Node::dummy());
                    let (mac, attrs, _) = n.take_mac_call();
                    self.check_attributes(&attrs, &mac);

                    *node = self.collect_bang(mac, Node::KIND).make_ast::<Node>().into()
                }
                None if node.delegation().is_some() => unreachable!(),
                None => {
                    assign_id!(self, node.node_id_mut(), || node.walk(self))
                }
            };
        }
    }
}

impl<'a, 'b> MutVisitor for InvocationCollector<'a, 'b> {
    fn flat_map_item(&mut self, node: Box<ast::Item>) -> SmallVec<Box<ast::Item>, 1> {
        self.flat_map_node(node)
    }

    fn flat_map_assoc_item(
        &mut self,
        node: Box<ast::AssocItem>,
        ctxt: AssocCtxt,
    ) -> SmallVec<Box<ast::AssocItem>, 1> {
        match ctxt {
            AssocCtxt::Trait => self.flat_map_node(AstNodeWrapper::new(node, TraitItemTag)),
            AssocCtxt::Impl { of_trait: false, .. } => {
                self.flat_map_node(AstNodeWrapper::new(node, ImplItemTag))
            }
            AssocCtxt::Impl { of_trait: true, .. } => {
                self.flat_map_node(AstNodeWrapper::new(node, TraitImplItemTag))
            }
        }
    }

    fn flat_map_foreign_item(
        &mut self,
        node: Box<ast::ForeignItem>,
    ) -> SmallVec<Box<ast::ForeignItem>, 1> {
        self.flat_map_node(node)
    }

    fn flat_map_variant(&mut self, node: ast::Variant) -> SmallVec<ast::Variant, 1> {
        self.flat_map_node(node)
    }

    fn flat_map_where_predicate(
        &mut self,
        node: ast::WherePredicate,
    ) -> SmallVec<ast::WherePredicate, 1> {
        self.flat_map_node(node)
    }

    fn flat_map_field_def(&mut self, node: ast::FieldDef) -> SmallVec<ast::FieldDef, 1> {
        self.flat_map_node(node)
    }

    fn flat_map_pat_field(&mut self, node: ast::PatField) -> SmallVec<ast::PatField, 1> {
        self.flat_map_node(node)
    }

    fn flat_map_expr_field(&mut self, node: ast::ExprField) -> SmallVec<ast::ExprField, 1> {
        self.flat_map_node(node)
    }

    fn flat_map_param(&mut self, node: ast::Param) -> SmallVec<ast::Param, 1> {
        self.flat_map_node(node)
    }

    fn flat_map_generic_param(
        &mut self,
        node: ast::GenericParam,
    ) -> SmallVec<ast::GenericParam, 1> {
        self.flat_map_node(node)
    }

    fn flat_map_arm(&mut self, node: ast::Arm) -> SmallVec<ast::Arm, 1> {
        self.flat_map_node(node)
    }

    fn flat_map_stmt(&mut self, node: ast::Stmt) -> SmallVec<ast::Stmt, 1> {
        // FIXME: invocations in semicolon-less expressions positions are expanded as expressions, 
        // changing that requires some compatibility measures.
        if node.is_expr() {
            // The only way that we can end up with a `MacCall` expression statement, 
            // (as opposed to a `StmtKind::MacCall`) is if we have a macro as the 
            // trailing expression in a block (e.g. `fn foo() { my_macro!() }`).
            // Record this information, so that we can report a more specific 
            // `SEMICOLON_IN_EXPRESSIONS_FROM_MACROS` lint if needed.
            // See #78991 for an investigation of treating macros in this position
            // as statements, rather than expressions, during parsing.
            return match &node.kind {
                StmtKind::Expr(expr)
                    if matches!(**expr, ast::Expr { kind: ExprKind::MacCall(..), .. }) =>
                {
                    self.cx.current_expansion.is_trailing_mac = true;
                    // Don't use `assign_id` for this statement - it may get removed
                    // entirely due to a `#[cfg]` on the contained expression
                    let res = walk_flat_map_stmt(self, node);
                    self.cx.current_expansion.is_trailing_mac = false;
                    res
                }
                _ => walk_flat_map_stmt(self, node),
            };
        }

        self.flat_map_node(node)
    }

    fn visit_crate(&mut self, node: &mut ast::Crate) {
        self.visit_node(node)
    }

    fn visit_ty(&mut self, node: &mut ast::Ty) {
        self.visit_node(node)
    }

    fn visit_pat(&mut self, node: &mut ast::Pat) {
        self.visit_node(node)
    }

    fn visit_expr(&mut self, node: &mut ast::Expr) {
        // FIXME: Feature gating is performed inconsistently between `Expr` and `OptExpr`.
        if let Some(attr) = node.attrs.first() {
            self.cfg().maybe_emit_expr_attr_err(attr);
        }
        ensure_sufficient_stack(|| self.visit_node(node))
    }

    fn visit_method_receiver_expr(&mut self, node: &mut ast::Expr) {
        self.visit_node(AstNodeWrapper::from_mut(node, MethodReceiverTag))
    }

    fn filter_map_expr(&mut self, node: Box<ast::Expr>) -> Option<Box<ast::Expr>> {
        self.flat_map_node(AstNodeWrapper::new(node, OptExprTag))
    }

    fn visit_block(&mut self, node: &mut ast::Block) {
        let orig_dir_ownership = mem::replace(
            &mut self.cx.current_expansion.dir_ownership,
            DirOwnership::UnownedViaBlock,
        );
        walk_block(self, node);
        self.cx.current_expansion.dir_ownership = orig_dir_ownership;
    }

    fn visit_id(&mut self, id: &mut NodeId) {
        // We may have already assigned a `NodeId`
        // by calling `assign_id`
        if self.monotonic && *id == ast::DUMMY_NODE_ID {
            *id = self.cx.resolver.next_node_id();
        }
    }
}

pub struct ExpansionConfig<'feat> {
    pub crate_name: Symbol,
    pub features: &'feat Features,
    pub recursion_limit: Limit,
    pub trace_mac: bool,
    /// If false, strip `#[test]` nodes
    pub should_test: bool,
    /// If true, use verbose debugging for `proc_macro::Span`
    pub span_debug: bool,
    pub proc_macro_backtrace: bool,
}

impl ExpansionConfig<'_> {
    pub fn default(crate_name: Symbol, features: &Features) -> ExpansionConfig<'_> {
        ExpansionConfig {
            crate_name,
            features,
            // FIXME should this limit be configurable?
            recursion_limit: Limit::new(1024),
            trace_mac: false,
            should_test: false,
            span_debug: false,
            proc_macro_backtrace: false,
        }
    }
}
