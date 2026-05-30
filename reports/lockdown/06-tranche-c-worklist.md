# Lockdown Report — Tranche C Worklist & Projection
- UTC:       2026-05-30T12:25:00Z
- Status:    PLANNING

## 1. Tranche C Inputs & Counts
* **True DELETE Set**: **16** items (down from 22)
* **True DOWNGRADE Set**: **73** items
  * *Original Downgrades*: **49** items (inherent/trait methods internal to crate)
  * *Test-Only Facade Downgrades*: **24** items (facade symbols with no downstream prod consumption)
  * *Overlap Confirmation*: **0** overlap between the 24 test-only facade items and the 49 original downgrades (re-verified via static analysis).
* **Final FACADE Set**: **134** items (exact count: 139 with aliases, 136 unique symbols)

---

## 2. Public-API Projections (post-Tranche C)
Once the 10 submodules in `crates/dsl-core/src/lib.rs` are changed from `pub mod` to `pub(crate) mod`, and only the facade exports remain public:
* **Raw `cargo public-api -p dsl-core` Count**:
  * *Baseline*: `17,662` lines
  * *Projected Target*: **`14,813`** lines (reduction of 2,849 lines)
* **Simplified `cargo public-api -p dsl-core -sss` Count**:
  * *Baseline*: `2,983` lines
  * *Projected Target*: **`2,575`** lines (reduction of 408 lines)

---

## 3. Consolidated Worklist

### A. DELETE Worklist (16 items to delete)
Change to private/delete from source:
1. `from_byte_offset` (`diagnostics.rs:79`)
2. `with_fix` (`diagnostics.rs:188`)
3. `with_related` (`diagnostics.rs:194`)
4. `unknown` (`executable_plan.rs:73`)
5. `from_execution_plan` (`executable_plan.rs:361`)
6. `ordering_edges` (`execution_dag.rs:203`)
7. `coordination_edges` (`execution_dag.rs:223`)
8. `from_produces` (`binding_context.rs:68`)
9. `to_llm_context` (`binding_context.rs:158`)
10. `is_entity_ref` (`ast.rs:438`)
11. `is_synthetic` (`ast.rs:690`)
12. `find_symbol_refs` (`ast.rs:772`)
13. `find_unresolved_ref_locations` (`ast.rs:895`)
14. `validate_constellation_map_dir_schema_coordination_strict` (`dag_validator.rs:687`)
15. `resolve_subtype` (`types.rs:880`)
16. `resolution_tiers` (`types.rs:1446`)

### B. DOWNGRADE Worklist (73 items)
Change visibility from `pub` to `pub(crate)` or remove re-exports:
1. **24 Test-Only Facade Items** (remove from `lib.rs` root exports, make `pub(crate)` in submodules):
   * `AggregationRule`, `CrossScopeRule`, `DagWarning`, `EntityQualifier`, `EvaluationContext`, `GreenWhenExclusionReason`, `RelationScope`, `RunbookStep`, `StructuralError`, `TransactionPolicy`, `VersionHash`, `compute_effective_tier`, `compute_runbook_tier`, `dag_validator`, `from_effect_classes`, `green_when_coverage_for_dag`, `green_when_coverage_for_dags`, `green_when_coverage_summary`, `harden_schema_coordination_warnings`, `ordering_pairs`, `slot_mut`, `validate_constellation_map_schema_coordination`, `validate_resolved_template_gate_metadata`, `with_entity_attr`
2. **49 Original Crate-Internal Downgrades** (change definition visibility to `pub(crate)` in source):
   * Methods: `warning`, `hint`, `with_span`, `is_error`, `is_warning`, `is_hard_error`, `imposes_order`, `ordering_pair`, `matches_type`, `merge`, `names`, `available_types`, `to_dsl_string`, `to_user_dsl_string`, `get_arg`, `get_value`, `integer`, `resolved_entity_ref`, `symbol_ref`, `is_unresolved_entity_ref`, `is_resolved_entity_ref`, `is_symbol_ref`, `is_literal`, `as_string`, `as_uuid`, `resolved_key`, `as_integer`, `as_decimal`, `as_boolean`, `as_list`, `as_map`, `span`, `with_resolved_key`, `try_with_resolved_key`, `synthetic`, `is_fully_resolved`, `resolved_count`, `resolution_percentage`, `verb_name`, `min_confidence`, `from_score`, `mime_type`, `primary_column`, `is_simple`, `all_columns`, `discriminators`, `to_sexpr`, `arg_name`, `entity_uuid`, `entity_uuid_binding`, `natural_key`, `compile_resolved_entity`, `binding_resolved_entity`, `runtime_create_natural_key`, `verb_path`, `is_clean`.

### C. FACADE Worklist (134 items)
Retain as public and re-export in `lib.rs`:
* [Exact list of the 134/139 symbols listed in Tranche B report]

### D. Module Privatization Worklist
Change visibility of submodules in `lib.rs` from `pub mod` to `pub(crate) mod`:
1. `ast`
2. `binding_context`
3. `compiler`
4. `config`
5. `diagnostics`
6. `executable_plan`
7. `execution_dag`
8. `frontier`
9. `parser`
10. `resolver`
