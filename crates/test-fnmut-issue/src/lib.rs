// crates/test-fnmut-issue/src/lib.rs

use std::marker::PhantomData;

// --- Mock DefId, LocalDefId, OwnerId (from trait-fixer-rustc-mock) ---
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DefId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LocalDefId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OwnerId;

impl DefId {
    pub fn as_local(self) -> Option<LocalDefId> {
        Some(LocalDefId)
    }
    pub fn to_def_id(self) -> DefId {
        self
    }
}

impl LocalDefId {
    pub fn to_def_id(self) -> DefId {
        DefId
    }
}

impl OwnerId {
    pub const DUMMY: Self = Self; // Add DUMMY for OwnerId
    pub fn to_def_id(self) -> DefId {
        DefId
    }
}

// --- Mock Span and Symbol (from trait-fixer-rustc-mock) ---
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span;

pub const DUMMY_SP: Span = Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Symbol;

impl Symbol {
    pub fn intern(_s: &str) -> Self {
        Symbol
    }
}

pub mod sym {
    use super::Symbol;
    pub const DERIVE: Symbol = Symbol;
    pub const DEBUG: Symbol = Symbol; // For lang_items().get_diagnostic_item(sym::Debug)
}

// --- Mock TyCtxt (from trait-fixer-rustc-mock) ---
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TyCtxt<'tcx>(pub PhantomData<&'tcx ()>);

impl<'tcx> TyCtxt<'tcx> {
    pub fn hir(self) -> MockHir {
        MockHir
    }
    pub fn lang_items(self) -> MockLangItems {
        MockLangItems
    }
    pub fn get_diagnostic_item(self, _symbol: Symbol) -> Option<DefId> {
        Some(DefId)
    }
    pub fn param_env(self, _def_id: DefId) -> MockParamEnv {
        MockParamEnv
    }
    pub fn infer_ctxt(self) -> MockInferCtxtBuilder {
        MockInferCtxtBuilder
    }
    pub fn mk_trait_ref(self, _def_id: DefId, _args: MockGenericArgs) -> MockPredicate {
        MockPredicate
    }
    pub fn mk_args_trait(self, _ty: MockTy<'tcx>, _substs: MockSubsts) -> MockGenericArgs {
        MockGenericArgs
    }
    pub fn typeck(self, _owner_id: OwnerId) -> MockTypeckResults {
        MockTypeckResults
    }
    pub fn type_of(self, _owner_id: OwnerId) -> MockEarlyBinder<'tcx> {
        MockEarlyBinder(PhantomData)
    }
    pub fn get_attrs(self, _def_id: DefId, _sym: Symbol) -> Vec<MockAttribute> {
        vec![MockAttribute]
    }
}

// --- Mock Hir and related types (from trait-fixer-rustc-mock) ---
pub struct MockHir;

impl MockHir {
    pub fn walk_tops(self, _f: impl FnMut(&Item)) {
        // Do nothing for mock
    }
}

pub type ItemId = DefId; // Mock ItemId as DefId

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Item<'tcx>(PhantomData<&'tcx ()>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ItemKind<'tcx> {
    Struct(PhantomData<&'tcx ()>),
    Enum(PhantomData<&'tcx ()>),
    Union(PhantomData<&'tcx ()>),
    // Add other ItemKind variants as needed
}

// --- Mock LangItems (from trait-fixer-rustc-mock) ---
pub struct MockLangItems;

impl MockLangItems {
    pub fn clone_trait(self) -> Option<DefId> {
        Some(DefId)
    }
}

// --- Mock Inference Context (from trait-fixer-rustc-mock) ---
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MockParamEnv;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MockPredicate;

pub struct MockInferCtxtBuilder;

impl MockInferCtxtBuilder {
    pub fn build(self, _typing_mode: MockTypingMode) -> MockInferCtxt {
        MockInferCtxt
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MockTypingMode {
    NonBodyAnalysis,
    // Add other variants as needed
}

impl Default for MockTypingMode {
    fn default() -> Self {
        MockTypingMode::NonBodyAnalysis
    }
}

pub struct MockInferCtxt;

impl MockInferCtxt {
    pub fn probe(self, f: impl FnOnce(&MockInferCtxt) -> bool) -> bool {
        f(&self) // Call the closure directly with self
    }
    pub fn at(self, _cause: &MockObligationCause, _param_env: MockParamEnv) -> MockInferCtxtAt {
        MockInferCtxtAt
    }
}

pub struct MockInferCtxtAt;

impl MockInferCtxtAt {
    pub fn predicate_may_hold(self, _predicate: &MockPredicate) -> bool {
        true // Always succeed for mock
    }
}

// --- Mock Attributes (from trait-fixer-rustc-mock) ---
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MockAttribute;

impl MockAttribute {
    pub fn meta_item_list(self) -> Vec<MockMetaItem> {
        vec![MockMetaItem] // Return a dummy meta item
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MockMetaItem;

impl MockMetaItem {
    pub fn has_name(self, _symbol: Symbol) -> bool {
        true // Always has name for mock
    }
}

// --- Mock Types and Substitutions (from trait-fixer-rustc-mock) ---
pub struct MockTypeckResults;

pub struct MockEarlyBinder<'tcx>(PhantomData<&'tcx ()>);

impl<'tcx> MockEarlyBinder<'tcx> {
    pub fn instantiate(self, _tcx: TyCtxt<'tcx>, _substs: MockSubsts) -> MockTy<'tcx> {
        MockTy(PhantomData)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MockTy<'tcx>(PhantomData<&'tcx ()>);

pub struct MockSubsts;

impl MockSubsts {
    pub fn empty() -> Self {
        MockSubsts
    }
}

pub struct MockGenericArgs; // Placeholder for rustc_middle::ty::GenericArgs

// --- Mock Obligation and Predicates (from trait-fixer-rustc-mock) ---
pub struct MockObligationCause;

pub enum MockObligationCauseCode {
    Misc,
    // Add other variants as needed
}

impl MockObligationCause {
    pub fn new(_span: Span, _body_id: LocalDefId, _code: MockObligationCauseCode) -> Self {
        MockObligationCause
    }
}

pub struct MockPredicateObligation;

impl MockPredicateObligation {
    pub fn new(
        _tcx: TyCtxt,
        _cause: MockObligationCause,
        _param_env: MockParamEnv,
        _predicate: MockPredicate,
    ) -> Self {
        MockPredicateObligation
    }
}

pub struct MockBinder; // Placeholder for rustc_middle::ty::Binder

impl MockBinder {
    pub fn dummy<T>(_value: T) -> MockBinder {
        MockBinder
    }
}

// --- QueryContext Trait (from trait-fixer-query-context-trait) ---
pub trait QueryContext<'tcx> {
    fn walk_hir_tops(&self, f: impl FnMut(&'tcx Item<'tcx>));
}

// --- MockTyCtxt implementation of QueryContext (from trait-fixer-query-context-mock) ---
pub struct QueryContextMockTyCtxt<'tcx>(pub TyCtxt<'tcx>); // Renamed to avoid conflict

// Implementation for QueryContextMockTyCtxt
impl<'tcx> QueryContext<'tcx> for QueryContextMockTyCtxt<'tcx> {
    fn walk_hir_tops(&self, _f: impl FnMut(&'tcx Item<'tcx>)) {
        println!("Mock QueryContextMockTyCtxt::walk_hir_tops called");
    }
}

pub fn trigger_issue<'tcx>(mock_tcx: QueryContextMockTyCtxt<'tcx>) {
    let _item = Item(std::marker::PhantomData);
    mock_tcx.walk_hir_tops(|_item_ref| {
        // do nothing
    });
}