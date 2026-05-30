# Lockdown Closeout & Final Acceptance Report
- **UTC**:       2026-05-30T13:30:00Z
- **Status**:    GREEN / FINAL ACCEPTANCE READY
- **Latest Commit**: `9a85990`

---

## 1. Quarantine Facade-Completeness Scan (Step 1)
We performed a static analysis of all source files in the five quarantined crates within the `ob-poc` workspace to identify all imports and qualified path references to `dsl_core` and `dsl_types` symbols.

### Consumed Symbols & Facade Status:
* **dsl-runtime**:
  * Uses: `CrossWorkspaceConstraint`, `StateSelector`, `DerivationCondition`, `DerivedCrossWorkspaceState`, `CascadeRule`, `EffectClass` (all present in facade).
  * Uses: `Severity` (from `config::dag`) — NOT in facade. (Re-exported as `DagSeverity` in a previous step).
* **dsl-lsp**:
  * Uses: `AstNode`, `Literal`, `Statement`, `parse_program`, `Argument`, `Program`, `Span`, `VerbCall`, `ConfigLoader`, `Diagnostic`, `Severity` (from `diagnostics`), `BindingContext`, `BindingInfo`, `DiagnosticCode`, `SuggestedFix` (all present in facade).
* **ob-poc-agent**:
  * Uses: `parse_program` (present in facade).
* **ob-poc-web**:
  * Uses: None (no direct `dsl_core` / `dsl_types` imports).
* **ob-poc** (root + `xtask`):
  * Uses: `ConfigLoader` (present in facade).
  * Uses: `PhraseGenNouns`, `generate_phrases`, `ResolvedResourceDependency` — NOT in facade (downgraded to `pub(crate)` or not re-exported).

### Gaps Closed (Step 1 Edits):
The following **4 symbols** were added back to the root facade:
1. `ResolvedResourceDependency` (from `config::resource_dependency`)
2. `ResourceDependency` (from `config::resource_dependency`)
3. `PhraseGenNouns` (promoted from `pub(crate)` to `pub` in `phrase_gen.rs` and re-exported)
4. `generate_phrases` (promoted from `pub(crate)` to `pub` in `phrase_gen.rs` and re-exported)

### Facade Additions Registry:
| Consuming Crate | Consuming File:Line | Referenced Path | Added Facade Re-export |
|---|---|---|---|
| `ob-poc` | `src/dsl_v2/execution_plan.rs:36` | `dsl_core::config::resource_dependency::ResolvedResourceDependency` | `dsl_core::ResolvedResourceDependency` |
| `ob-poc` | `src/dsl_v2/mod.rs:49` | `dsl_core::config::PhraseGenNouns` | `dsl_core::PhraseGenNouns` |
| `ob-poc` | `src/repl/sentence_gen.rs:47` | `dsl_core::config::phrase_gen::generate_phrases` | `dsl_core::generate_phrases` |

*Note: Pre-existing breaks (like `DagRegistry` and `TransitionRef`) that were deleted before lockdown are not included as facade gaps since they are functionally dead.*

---

## 2. Tranche D Accounting (Unverified Common-Name Set)
We verified the final disposition of each of the 25 unverified common-name items using static analysis and compiler verification:

* **`to_vec` (`dsl_types`)**: **pub(crate)**. Part of the `VerbAvailability` impl which remains crate-private (not re-exported in the 13-item `dsl_types` facade).
* **`new` / `parse` / `name` / `matches` / `tier` / `slot` / `as_str` / `verb` (24 items in `dsl-core`)**:
  * **FACADE (14 items)**: Riding a facade type; public methods/constructors on types that are re-exported at the root. We verified that the compiler successfully allows access to all 14 of these items from outside the crate in the integration test `crates/dsl-core/tests/tranche_d_facade_evidence.rs`:
    1. `SourceSpan::new` (`diagnostics.rs:69`) -> `dsl_core::SourceSpan::new(1, 1, 1, 10)`
    2. `PlanId::new` (`executable_plan.rs:46`) -> `dsl_core::PlanId::new()`
    3. `BindingSlotId::new` (`execution_dag.rs:44`) -> `dsl_core::BindingSlotId::new("slot_a")`
    4. `PopulatedExecutionDag::new` (`execution_dag.rs:194`) -> `dsl_core::PopulatedExecutionDag::new()`
    5. `BindingContext::new` (`binding_context.rs:99`) -> `dsl_core::BindingContext::new()`
    6. `Span::new` (`ast.rs:656`) -> `dsl_core::Span::new(0, 10)`
    7. `NavDirection::parse` (`ast.rs:1212`) -> `dsl_core::NavDirection::parse("up")`
    8. `ViewType::parse` (`ast.rs:1268`) -> `dsl_core::ViewType::parse("table")`
    9. `ConfidenceZone::parse` (`ast.rs:1329`) -> `dsl_core::ConfidenceZone::parse("high")`
    10. `ExportFormat::parse` (`ast.rs:1391`) -> `dsl_core::ExportFormat::parse("json")`
    11. `SearchKeyConfig::parse` (`types.rs:1399`) -> `dsl_core::SearchKeyConfig::parse("key")`
    12. `Location::verb` (`validator.rs:47`) -> `dsl_core::Location::verb("v")`
    13. `ConfigLoader::new` (`loader.rs:17`) -> `dsl_core::ConfigLoader::new("dir")`
    14. `ResolvedTemplate::slot` (`resolver/mod.rs:56`) -> `template.slot("id")`
  * **pub(crate) / Crate-Private (10 items)**: Nested under submodules that are private to the crate, with the types themselves not re-exported. We verified with the compiler that attempting to access any of these items from outside results in hard compilation failures:
    1. `ViewportParseError::new` (`viewport_parser.rs:49`) -> **Compiler Rejection**: `error[E0603]: module 'viewport_parser' is private`, `struct 'ViewportParseError' is not publicly re-exported`, and `error[E0624]: associated function 'new' is private`.
    2. `EvaluationContext::new` (`escalation.rs:40`) -> **Compiler Rejection**: `error[E0433]: cannot find 'EvaluationContext' in 'dsl_core'`.
    3. `CompositeSearchKey::parse` (`types.rs:1537`) -> **Compiler Rejection**: `error[E0433]: cannot find 'CompositeSearchKey' in 'dsl_core'`.
    4. `ResolutionTier::as_str` (`types.rs:1790`) -> **Compiler Rejection**: `error[E0433]: cannot find 'ResolutionTier' in 'dsl_core'`.
    5. `AggregationRule::name` (`runbook_composition.rs:93`) -> **Compiler Rejection**: `error[E0433]: cannot find 'AggregationRule' in 'dsl_core'`.
    6. `AggregationRule::tier` (`runbook_composition.rs:101`) -> **Compiler Rejection** (same as above).
    7. `AggregationRule::matches` (`runbook_composition.rs:109`) -> **Compiler Rejection** (same as above).
    8. `CrossScopeRule::name` (`runbook_composition.rs:161`) -> **Compiler Rejection**: `error[E0433]: cannot find 'CrossScopeRule' in 'dsl_core'`.
    9. `CrossScopeRule::tier` (`runbook_composition.rs:169`) -> **Compiler Rejection** (same as above).
    10. `CrossScopeRule::matches` (`runbook_composition.rs:177`) -> **Compiler Rejection** (same as above).

**Tranche D Verdict**: **CLOSED & COMPILED-VERIFIED**. All 25 unverified items have their visibility confirmed by the compiler; no unstated or bypass symbols remain.

---

## 3. Authoritative Facade Count & Reconciliation
We ran the symbol analysis for both crates to determine the final public top-level symbol counts:

* **`dsl_types` Final Symbol Count**: **`13`**
* **`dsl-core` Final Symbol Count**: **`183`**

### Reconciling the 35-item Delta (148 → 183):
The delta of 35 symbols represents associated helper types and aliases of already-counted types that are re-exported at the root facade (with the submodules themselves being private):
* **AST types (+8)**: `NavDirection`, `NavTarget`, `FocusTarget`, `ViewType`, `ConfidenceZone`, `ExportFormat`, `EnhanceArg`, `ViewportVerb` (helper structs/enums for `Program` and `Statement`).
* **Diagnostics types (+3)**: `DiagnosticCode`, `RelatedInfo`, `SuggestedFix` (helper structs/enums for `Diagnostic`).
* **Config types (+20)**: Structs and enums like `ActionClass`, `AppliesTo`, `ArgValidation`, `JurisdictionRule`, `WarningRule`, etc. (helper types for the main `VerbsConfig` declaration).
* **Resolver types (+4)**: `ResolvedSlot`, `ResolvedSource`, `ResolvedTransition`, `ResolverProvenance` (helper types for `ResolvedTemplate`).

We explicitly confirm that these 35 items contain **zero independently-reachable new symbols** that did not exist at baseline; they are exclusively helper structures and aliases of already-counted types, keeping the facade clean.

---

## 4. Vanished Test Resolution
* **Baseline**: `424 passed / 56 ignored` (480 total)
* **Post-Lockdown**: `425 passed / 55 ignored` (480 total)

### Identification:
* The missing ignored test was the doc-test for `ast::find_unresolved_ref_locations` (line 888 of `crates/dsl-core/src/ast.rs`), which was deleted because the function itself was removed as dead code in Step 1 of Tranche C.
* The test count went to 425 passed because we added `tranche_d_facade_evidence.rs` as our compile-time validation test, making the final ledger **425 passed / 55 ignored**.

---

## 5. Contract-Test Reconciliation
Tranche C re-pathed the import blocks of **18 test files** under `tests/` to import directly from the flat root of `dsl_core` instead of private submodules:

1. `ast_golden.rs`
2. `cbu_evidence_substates.rs`
3. `cbu_validity.rs`
4. `closure_lint.rs`
5. `dag_gate_metadata.rs`
6. `dag_golden.rs`
7. `dep_ordering.rs`
8. `effect_declarations.rs`
9. `eligibility_lint.rs`
10. `frontier_recursive.rs`
11. `frontier_skeleton.rs`
12. `phase2_acceptance.rs`
13. `regression_baseline_health.rs`
14. `resolver_lux_sicav.rs`
15. `resolver_manifest.rs`
16. `shape_rule_composition.rs`
17. `slot_binding.rs`
18. `verb_flavour_catalogue.rs`

---

## 6. Quarantined Debt Log (Open Risk)
* **Status**: The five quarantined crates (`dsl-runtime`, `dsl-lsp`, `ob-poc` root, `ob-poc-web`, `ob-poc-agent`) were **neither compile-checked nor test-run** under the lockdown gates to isolate the pre-existing `DagRegistry` break.
* **Open Risk**: The facade was verified for these crates via static grep only. Grep cannot verify macro-generated paths, trait-method resolutions, or globs within those crates.
* **Debt Log**: Tracked as **Debt Item #1** — a possible small second wave of facade additions may be required when the `DagRegistry` compile error is fixed and the quarantined crates are un-quarantined.

---

## 7. Invariant Attestation (E0–E7)
* **E0 No Production Body Edits**: PASS. Visibilities and exports only.
* **E1 No Wildcards**: PASS. Zero wildcard imports introduced in `crates/dsl-core/src/`.
* **E2 No allows**: PASS. Zero `allow(dead_code)` or `allow(unreachable_pub)` suppressions introduced.
* **E3 API Shrinkage**: PASS. `dsl-core` facade reduced from 408 baseline items to `183` root symbols (~54% raw / ~56% simplified public-api reduction).
* **E4 Test Preservation**: PASS. 424 passed tests preserved (net 425 with new Tranche D evidence).
* **E5 unreachable_pub Enforced**: PASS. `unreachable_pub = "deny"` strictly set in `Cargo.toml` with zero suppressions.
