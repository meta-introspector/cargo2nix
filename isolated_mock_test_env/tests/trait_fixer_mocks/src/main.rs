// isolated_mock_test_env/tests/trait_fixer_mocks/src/main.rs

// Common mock types
use trait_fixer_rustc_mock::{TyCtxt, DefId, sym, Symbol, Item, OwnerId, ItemKind, Span, Ty};

// AttributeReader mocks and trait
use trait_fixer_attribute_reader_trait::AttributeReader;
use trait_fixer_attribute_reader_mock::MockAttributeReaderTyCtxt;

// CompilerHost mocks and trait
use trait_fixer_compiler_host_trait::CompilerHost;
use trait_fixer_compiler_host_mock::MockCompilerHost;

// HirInfo mocks and trait
use trait_fixer_hir_info_trait::HirInfo;

// LangItems mocks and trait
use trait_fixer_lang_items_trait::LangItems;

// QueryContext mocks and trait
use trait_fixer_query_context_trait::QueryContext;

// TraitChecker mocks and trait
use trait_fixer_trait_checker_trait::TraitChecker;


// Mock Callbacks for CompilerHost test
struct MockCompilerCallbacks; // Simple empty struct
impl MockCompilerCallbacks {
    pub fn new() -> Self { MockCompilerCallbacks }
}

fn main() {
    println!("--- Running mock attribute reader test ---");
    let mock_tcx_instance = TyCtxt(std::marker::PhantomData);
    let mock_tcx_wrapper = MockAttributeReaderTyCtxt(mock_tcx_instance);
    let mock_def_id = DefId;
    let has_derive = mock_tcx_wrapper.has_derive_attr(mock_def_id, "Debug");
    assert_eq!(has_derive, true, "Mock AttributeReader should always return true for has_derive_attr");
    println!("Mock attribute reader test passed!\n");

    println!("--- Running mock compiler host test ---");
    let compiler_host_mock = MockCompilerHost;
    let mut mock_callbacks = MockCompilerCallbacks::new();
    let args = vec!["arg1".to_string(), "arg2".to_string()];
    compiler_host_mock.run_compiler_callbacks(args, &mut mock_callbacks);
    println!("Mock compiler host test finished (output above)");
    println!("Mock compiler host test passed!\n");

    println!("--- Running mock HirInfo test ---");
    let mock_item = Item(std::marker::PhantomData);
    let owner_id = HirInfo::get_owner_id(&mock_item);
    let item_kind = HirInfo::get_item_kind(&mock_item);
    let item_span = HirInfo::get_item_span(&mock_item);
    assert_eq!(owner_id, OwnerId, "Mock HirInfo::get_owner_id failed"); // Use OwnerId directly
    println!("Mock HirInfo::get_item_kind returned: {:?}", item_kind);
    println!("Mock HirInfo::get_item_span returned: {:?}", item_span);
    println!("Mock HirInfo test passed!\n");

    println!("--- Running mock LangItems test ---");
    let mock_tcx_instance_lang = TyCtxt(std::marker::PhantomData);
    let sym_debug_mock = sym::Debug;
    let clone_def_id = LangItems::get_clone_trait_def_id(&mock_tcx_instance_lang);
    let debug_def_id = LangItems::get_debug_trait_def_id(&mock_tcx_instance_lang, sym_debug_mock);
    assert_eq!(clone_def_id, Some(DefId), "Mock LangItems::get_clone_trait_def_id failed"); // Use DefId directly
    assert_eq!(debug_def_id, Some(DefId), "Mock LangItems::get_debug_trait_def_id failed"); // Use DefId directly
    println!("Mock LangItems test passed!\n");

    println!("--- Running mock QueryContext test ---");
    let mock_tcx_instance_query = TyCtxt(std::marker::PhantomData);
    let mut call_count = 0;
    QueryContext::walk_hir_tops(&mock_tcx_instance_query, |_item: &Item| { // Use Item directly
        call_count += 1;
        println!("  Mock QueryContext::walk_hir_tops called with an item (call {})", call_count);
    });
    assert_eq!(call_count, 0, "Mock QueryContext::walk_hir_tops should not call callback in this basic mock");
    println!("Mock QueryContext test passed!\n");

    println!("--- Running mock TraitChecker test ---");
    let mock_tcx_instance_checker = TyCtxt(std::marker::PhantomData);
    let mock_adt_ty = Ty(std::marker::PhantomData);
    let mock_item_def_id = DefId;
    let mock_trait_def_id = DefId;
    let sym_intern_fn = |s: &str| Symbol::intern(s); // Corrected: Mock sym_intern function

    let trait_def_id = TraitChecker::get_trait_def_id(&mock_tcx_instance_checker, "Clone", sym_intern_fn);
    let implements_trait = TraitChecker::type_implements_trait(&mock_tcx_instance_checker, mock_adt_ty, mock_item_def_id, mock_trait_def_id);
    assert_eq!(trait_def_id, Some(DefId), "Mock TraitChecker::get_trait_def_id failed"); // Use DefId directly
    assert_eq!(implements_trait, true, "Mock TraitChecker::type_implements_trait failed");
    println!("Mock TraitChecker test passed!\n");
}
