# Type System Report

This report details the identified type usages and their labeled equivalents,
focusing on the abstraction of complex generics like `DRT`.


## Detected Type Usages:


### Original: 
- **Labeled:** _INVOCATION_COLLECTOR_ALIAS_
- **File:** submodules/rust/compiler/rustc_expand/src/expand.rs
- **Line:** 36
- **Context:**
  ```rust
  /// CollectorABDRT<'a,'b,DRT> = InvocationCollector<'a,'b,DRT> (collects macro invocations)
  ```
---

### Original: pub
- **Labeled:** _INVOCATION_COLLECTOR_ALIAS_
- **File:** submodules/rust/compiler/rustc_expand/src/expand.rs
- **Line:** 37
- **Context:**
  ```rust
  pub type CollectorABDRT<'a, 'b, DRT> = InvocationCollector<'a, 'b, DRT>;
  ```
---

### Original: 
- **Labeled:** _DERIVE_RESOLUTION_ALIAS_
- **File:** submodules/rust/compiler/rustc_expand/src/expand.rs
- **Line:** 38
- **Context:**
  ```rust
  /// ResolverBound = DeriveResolution (trait bound for derive resolution)
  ```
---

### Original: pub
- **Labeled:** _DERIVE_RESOLUTION_ALIAS_
- **File:** submodules/rust/compiler/rustc_expand/src/expand.rs
- **Line:** 39
- **Context:**
  ```rust
  pub type ResolverBound = DeriveResolution;
  ```
---

### Original: 
- **Labeled:** _EXT_CTXT_ALIAS_
- **File:** submodules/rust/compiler/rustc_expand/src/expand.rs
- **Line:** 40
- **Context:**
  ```rust
  /// ExpandContext<'b, DRT> = ExtCtxt<'b, DRT> (Expansion Context)
  ```
---

### Original: pub
- **Labeled:** _EXT_CTXT_ALIAS_
- **File:** submodules/rust/compiler/rustc_expand/src/expand.rs
- **Line:** 41
- **Context:**
  ```rust
  pub type ExpandContext<'b, DRT> = ExtCtxt<'b, DRT>;
  ```
---

### Original: 
- **Labeled:** _DRT_GENERIC_PARAM_
- **File:** submodules/rust/compiler/rustc_expand/src/expand.rs
- **Line:** 42
- **Context:**
  ```rust
  /// RefAMutExpandContext_B_DRT<'a,'b,DRT> = &'a mut crate::base::ExpandContext<'b, DRT> (Mutable reference to Expansion Context)
  ```
---

### Original: pub
- **Labeled:** _DRT_GENERIC_PARAM_
- **File:** submodules/rust/compiler/rustc_expand/src/expand.rs
- **Line:** 43
- **Context:**
  ```rust
  pub type RefAMutExpandContext_B_DRT<'a, 'b, DRT> = &'a mut crate::base::ExpandContext<'b, DRT>;
  ```
---

### Original: use
- **Labeled:** _EXT_CTXT_ALIAS_
- **File:** submodules/rust/compiler/rustc_expand/src/expand.rs
- **Line:** 51
- **Context:**
  ```rust
  use crate::base::{ExtCtxt, ExpandResult, DeriveResolution, SyntaxExtensionKind, BangProcMacro, AttrProcMacro, MultiItemModifier, MacroExpanderFn, GlobDelegationExpander};
  ```
---

### Original: use
- **Labeled:** _INVOCATION_COLLECTOR_ALIAS_
- **File:** submodules/rust/compiler/rustc_expand/src/expand.rs
- **Line:** 54
- **Context:**
  ```rust
  use crate::ast_fragments_split::InvocationCollector;
  ```
---

### Original: impl<'a
- **Labeled:** _DRT_GENERIC_PARAM_
- **File:** submodules/rust/compiler/rustc_expand/src/expand.rs
- **Line:** 72
- **Context:**
  ```rust
  impl<'a, 'b, DRT: OpaqueDeriveResolution + 'static> MacroExpander<'a, 'b, DRT> {
  ```
---

### Original: pub
- **Labeled:** _EXT_CTXT_ALIAS_
- **File:** submodules/rust/compiler/rustc_expand/src/expand.rs
- **Line:** 73
- **Context:**
  ```rust
  pub fn new(cx: &'a mut crate::base::ExtCtxt<'b, DRT>, monotonic: bool) -> Self {
  ```
---

### Original: map(|DeriveResolution
- **Labeled:** _DERIVE_RESOLUTION_ALIAS_
- **File:** submodules/rust/compiler/rustc_expand/src/expand.rs
- **Line:** 180
- **Context:**
  ```rust
  .map(|DeriveResolution { path, item, exts: _, is_const }| {
  ```
---

### Original: let
- **Labeled:** _INVOCATION_COLLECTOR_ALIAS_
- **File:** submodules/rust/compiler/rustc_expand/src/expand.rs
- **Line:** 264
- **Context:**
  ```rust
  let mut collector = InvocationCollector {
  ```
---

### Original: let
- **Labeled:** _DRT_GENERIC_PARAM_
- **File:** submodules/rust/compiler/rustc_expand/src/expand.rs
- **Line:** 551
- **Context:**
  ```rust
  let expander = ext.get_macro_kind().downcast_ref::<Arc<crate::mbe::macro_rules_expander::MacroRulesMacroExpander<DRT>>>().expect("expected MacroRulesExpander");
  ```
---



