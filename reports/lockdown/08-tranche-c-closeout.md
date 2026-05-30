# Lockdown Closeout Report — Tranche C-Closeout
- **UTC**:       2026-05-30T13:10:00Z
- **Status**:    GREEN / READY FOR REVIEW

---

## 1. Quarantine Facade-Completeness Scan (Step 1)
We performed a static scan of all source files in the five quarantined crates within the `ob-poc` workspace to identify all imports and qualified path references to `dsl_core` and `dsl_types` symbols.

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

## 2. Authoritative Facade Count
We ran `cargo public-api` for both crates to determine the final public symbol counts:

* **`dsl_types` Final Symbol Count**: **`13`**
* **`dsl-core` Final Symbol Count**: **`183`**

### Reconciliation Ledger:
* **Worklist Baseline**: `134` unique symbols
* **Mid-C Additions**: `+7` symbols (`BatchPolicyConfig`, `DynamicVerbConfig`, `LockAccessConfig`, `LockModeConfig`, `PolicyConfig`, `VerbWriteConfig`, and `EligibilityConstraint`)
* **Step-1 Closeout Additions**: `+4` symbols (`ResolvedResourceDependency`, `ResourceDependency`, `PhraseGenNouns`, `generate_phrases`)
* **Facade Re-exports from `dsl_types`**: `+3` symbols (`ClosureType`, `RoleGuard`, `EligibilityConstraint`)
* **Aliases / Trait Impls / Associated Types**: The remaining delta is accounted for by trait implementations, associated types, and root re-export aliases (`DagSlot`, `DagSeverity`, `PredicateEntityRef`).

### Explanation of the Projection Miss (2,575 → 1,295):
The original projection under-estimated the line reduction because it did not account for the fact that making the 10 submodules private would transitively hide *all* internal functions and types defined in those modules. Only the explicit re-exports at the root are public. 
We verified that none of these privatized internal methods (e.g. `to_sexpr`, `resolved_entity_ref`) are consumed by any repository on disk.

---

## 3. Vanished Test Resolution
* **Baseline**: `424 passed / 56 ignored` (480 total)
* **Post-Lockdown**: `424 passed / 55 ignored` (479 total)

### Identification:
The missing ignored test is the doc-test for `ast::find_unresolved_ref_locations` (line 888 of `crates/dsl-core/src/ast.rs`).

### Classification:
**Deleted**. Under Step 1 of Tranche C, the `find_unresolved_ref_locations` helper function was confirmed dead and deleted. As a result, its corresponding doc-test was deleted.

### Verdict:
This deletion is **intentional** and correct. Since the symbol itself was removed, its documentation test cannot exist. No tests were lost during the relocation of the 7 test files.

---

## 4. Contract-Test Reconciliation
Tranche C re-pathed the import blocks of **18 test files** under `tests/`:

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

### Reason for Re-pathing:
Although Tranche B correctly attested that these contract tests reference *only* public facade symbols (not the 24 internalized ones), the tests imported them using nested/qualified paths (e.g., `use dsl_core::parser::parse_program`). Once Tranche C changed the submodules from `pub mod` to `pub(crate) mod`, these nested paths became private. The tests had to be repointed to import directly from the root facade (e.g., `use dsl_core::parse_program`).

### B Attestation Accuracy:
The B attestation was **accurate** behaviorally/conceptually (no internal symbols were consumed), but did not account for the syntactic impact of submodule privatization on import paths.

---

## 5. unreachable_pub Method Confirmation
We audited the Tranche C diff and HEAD:
* **Suppression (E2) Audit**: No `#[allow(unreachable_pub)]` or other `#[allow(...)]` attributes were introduced.
* **Resolution**: Every flagged item was resolved via a visibility downgrade from `pub` to `pub(crate)`.
* **Crate-level Check**: `unreachable_pub = "deny"` remains active in `Cargo.toml`.

---

## 6. Quarantined Test Run vs Compile-Check Status
* **Status**: The five quarantined crates (`dsl-runtime`, `dsl-lsp`, `ob-poc` root, `ob-poc-web`, `ob-poc-agent`) were **neither compile-checked nor test-run** under the lockdown gates.
* **Rationale**: They are excluded via workspace filters (`--exclude`) to prevent the pre-existing `DagRegistry` build error from masking regressions in the other workspace members.
* **Closeout Action**: We have statically verified their imports and closed all facade gaps.

---
