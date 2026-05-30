# Lockdown Report — Tranche 00-baseline
- UTC:       2026-05-30T09:39:00Z
- Commit(s): N/A (Read-only baseline capture)
- Status:    GREEN

## Summary
Captured baseline build and test results, generated public-api baselines, and computed the reconciled worklist deterministically.

## Edits
- git diff --stat:
  *No edits made in Tranche 0.*
- Deleted:    None (0)
- Downgraded: None (0)
- Relocated:  None (0)

## Gate Evidence (actual output tails — not summaries)
- cargo build:
```text
warning: patch `sem_os_ontology v0.1.0 (/Users/adamtc007/dev/sem-os/crates/sem_os_ontology)` was not used in the crate graph
warning: patch `sem_os_policy v0.1.0 (/Users/adamtc007/dev/sem-os/crates/sem_os_policy)` was not used in the crate graph
warning: patch `bpmn-lite-engine v0.1.0 (/Users/adamtc007/dev/bpmn-lite/bpmn-lite-engine)` was not used in the crate graph
warning: patch `bpmn-lite-ffi-grpc v0.1.0 (/Users/adamtc007/dev/bpmn-lite/bpmn-lite-ffi-grpc)` was not used in the crate graph
warning: patch `bpmn-lite-ffi-http v0.1.0 (/Users/adamtc007/dev/bpmn-lite/bpmn-lite-ffi-http)` was not used in the crate graph
warning: patch `bpmn-lite-server v0.1.0 (/Users/adamtc007/dev/bpmn-lite/bpmn-lite-server)` was not used in the crate graph
warning: patch `bpmn-lite-store v0.1.0 (/Users/adamtc007/dev/bpmn-lite/bpmn-lite-store)` was not used in the crate graph
warning: patch `dmn-lite-bridge v0.1.0 (/Users/adamtc007/dev/bpmn-lite/dmn-lite-bridge)` was not used in the crate graph
warning: patch `dsl-bus-client v0.1.0 (/Users/adamtc007/dev/bpmn-lite/dsl-bus-client)` was not used in the crate graph
warning: patch `dsl-bus-protocol v0.1.0 (/Users/adamtc007/dev/bpmn-lite/dsl-bus-protocol)` was not used in the crate graph
warning: patch `dsl-bus-server v0.1.0 (/Users/adamtc007/dev/bpmn-lite/dsl-bus-server)` was not used in the crate graph
warning: patch `dsl-bus-storage v0.1.0 (/Users/adamtc007/dev/bpmn-lite/dsl-bus-storage)` was not used in the crate graph
warning: patch `dsl-manifest v0.1.0 (/Users/adamtc007/dev/bpmn-lite/dsl-manifest)` was not used in the crate graph
warning: patch `ffi-catalogue v0.1.0 (/Users/adamtc007/dev/bpmn-lite/ffi-catalogue)` was not used in the crate graph
help: Check that the patched package version and available features are compatible
      with the dependency requirements. If the patch has a different version from
      what is locked in the Cargo.lock file, run `cargo update` to use the new
      version. This may also occur with an optional dependency that is not enabled.
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s

```
- cargo test:
```text
test v1_2_canonical_predicate_with_exists_parses ... ok
test v1_2_canonical_transition_has_transition_args ... ok
test v1_2_validator_clean_for_canonical_shapes ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/verb_flavour_catalogue.rs (target/debug/deps/verb_flavour_catalogue-98dfc91c2974995f)

running 4 tests
test tollgate_flavour_is_empty_body_only ... ok
test every_catalogue_verb_has_phase7_flavour ... ok
test discretionary_verbs_have_authority_and_audit_metadata ... ok
test phase7_flavour_lints_are_clean_for_real_catalogue ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.24s

     Running unittests src/lib.rs (target/debug/deps/dsl_types-87206f3447dfee76)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests dsl_core

running 6 tests
test crates/dsl-core/src/ast.rs - ast::count_entity_refs (line 828) ... ignored
test crates/dsl-core/src/ast.rs - ast::find_unresolved_ref_locations (line 888) ... ignored
test crates/dsl-core/src/config/mod.rs - config (line 14) ... ignored
test crates/dsl-core/src/config/phrase_gen.rs - config::phrase_gen::generate_phrases (line 115) ... ignored
test crates/dsl-core/src/viewport_parser.rs - viewport_parser (line 25) ... ignored
test crates/dsl-core/src/config/predicate/parser.rs - config::predicate::parser::parse_green_when (line 25) ... ok

test result: ok. 1 passed; 0 failed; 5 ignored; 0 measured; 0 filtered out; finished in 0.16s

   Doc-tests dsl_types

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


```
- cargo public-api:
  - `dsl_types` public API lines: 681 (saved in `/tmp/baseline_dsl_types.txt`)
  - `dsl-core` public API lines: 17662 (saved in `/tmp/baseline_dsl_core.txt`)

## Reconciled Worklist
### dsl_types DETAILED WORKLIST
1. DELETE set (3 items):
  - `fn` slot_name (constellation_map_def.rs:181)
  - `fn` min_state (constellation_map_def.rs:188)
  - `fn` verb_fqn (constellation_map_def.rs:208)

2. OVERLAP (Struck from DOWNGRADE set because they are deleted) (3 items):
  - `slot_name` (constellation_map_def.rs:181)
  - `min_state` (constellation_map_def.rs:188)
  - `verb_fqn` (constellation_map_def.rs:208)

3. DOWNGRADE set (0 items):

4. FACADE exports (13 items):
  - `ConstellationMapDefBody` (constellation_map_def.rs:10)
  - `ClosureType` (constellation_map_def.rs:22)
  - `EligibilityConstraint` (constellation_map_def.rs:31)
  - `RoleGuard` (constellation_map_def.rs:40)
  - `AuditClass` (constellation_map_def.rs:49)
  - `CompletenessAssertionConfig` (constellation_map_def.rs:53)
  - `SlotDef` (constellation_map_def.rs:66)
  - `SlotType` (constellation_map_def.rs:141)
  - `Cardinality` (constellation_map_def.rs:153)
  - `JoinDef` (constellation_map_def.rs:162)
  - `DependencyEntry` (constellation_map_def.rs:175)
  - `VerbPaletteEntry` (constellation_map_def.rs:199)
  - `StructuralFacts` (resolver_facts.rs:9)

5. TEST-RELOCATION set (0 items):

6. SERDE-CONSTRUCTED set (0 items):

7. UNVERIFIED set (1 items):
  - `to_vec` (constellation_map_def.rs:225)

==================================================

### dsl-core DETAILED WORKLIST
1. DELETE set (22 items):
  - `fn` from_byte_offset (diagnostics.rs:79)
  - `fn` with_fix (diagnostics.rs:188)
  - `fn` with_related (diagnostics.rs:194)
  - `fn` undefined_symbol_error (diagnostics.rs:237)
  - `fn` cycle_error (diagnostics.rs:249)
  - `fn` missing_arg_error (diagnostics.rs:260)
  - `fn` unknown_verb_error (diagnostics.rs:271)
  - `fn` unknown (executable_plan.rs:73)
  - `fn` from_execution_plan (executable_plan.rs:361)
  - `fn` ordering_edges (execution_dag.rs:203)
  - `fn` coordination_edges (execution_dag.rs:223)
  - `fn` from_produces (binding_context.rs:68)
  - `fn` to_llm_context (binding_context.rs:158)
  - `fn` is_entity_ref (ast.rs:438)
  - `fn` is_synthetic (ast.rs:690)
  - `fn` find_symbol_refs (ast.rs:772)
  - `fn` find_unresolved_ref_locations (ast.rs:895)
  - `fn` parse_single_verb (parser.rs:68)
  - `fn` resolve_subtype (types.rs:880)
  - `fn` resolution_tiers (types.rs:1446)
  - `fn` validate_constellation_map_dir_schema_coordination_strict (dag_validator.rs:687)
  - `struct` InstanceFrontier (mod.rs:36)

2. OVERLAP (Struck from DOWNGRADE set because they are deleted) (13 items):
  - `from_byte_offset` (diagnostics.rs:79)
  - `with_fix` (diagnostics.rs:188)
  - `with_related` (diagnostics.rs:194)
  - `unknown` (executable_plan.rs:73)
  - `from_execution_plan` (executable_plan.rs:361)
  - `ordering_edges` (execution_dag.rs:203)
  - `coordination_edges` (execution_dag.rs:223)
  - `from_produces` (binding_context.rs:68)
  - `to_llm_context` (binding_context.rs:158)
  - `is_entity_ref` (ast.rs:438)
  - `is_synthetic` (ast.rs:690)
  - `resolve_subtype` (types.rs:880)
  - `resolution_tiers` (types.rs:1446)

3. DOWNGRADE set (79 items):
  - `warning` (diagnostics.rs:146)
  - `hint` (diagnostics.rs:158)
  - `with_span` (diagnostics.rs:182)
  - `is_error` (diagnostics.rs:200)
  - `is_warning` (diagnostics.rs:205)
  - `is_hard_error` (diagnostics.rs:210)
  - `with_span` (viewport_parser.rs:56)
  - `imposes_order` (execution_dag.rs:158)
  - `ordering_pair` (execution_dag.rs:168)
  - `matches_type` (binding_context.rs:40)
  - `merge` (binding_context.rs:104)
  - `names` (binding_context.rs:121)
  - `available_types` (binding_context.rs:136)
  - `to_dsl_string` (ast.rs:53)
  - `to_user_dsl_string` (ast.rs:63)
  - `to_dsl_string` (ast.rs:81)
  - `to_user_dsl_string` (ast.rs:89)
  - `to_dsl_string` (ast.rs:110)
  - `to_user_dsl_string` (ast.rs:126)
  - `get_arg` (ast.rs:149)
  - `get_value` (ast.rs:154)
  - `to_dsl_string` (ast.rs:293)
  - `to_user_dsl_string` (ast.rs:329)
  - `integer` (ast.rs:362)
  - `resolved_entity_ref` (ast.rs:385)
  - `symbol_ref` (ast.rs:404)
  - `is_unresolved_entity_ref` (ast.rs:416)
  - `is_resolved_entity_ref` (ast.rs:427)
  - `is_symbol_ref` (ast.rs:443)
  - `is_literal` (ast.rs:448)
  - `as_string` (ast.rs:457)
  - `as_uuid` (ast.rs:466)
  - `resolved_key` (ast.rs:479)
  - `as_integer` (ast.rs:495)
  - `as_decimal` (ast.rs:503)
  - `as_boolean` (ast.rs:512)
  - `as_list` (ast.rs:520)
  - `as_map` (ast.rs:528)
  - `span` (ast.rs:536)
  - `with_resolved_key` (ast.rs:558)
  - `try_with_resolved_key` (ast.rs:568)
  - `to_dsl_string` (ast.rs:630)
  - `merge` (ast.rs:661)
  - `synthetic` (ast.rs:682)
  - `is_fully_resolved` (ast.rs:803)
  - `resolved_count` (ast.rs:808)
  - `resolution_percentage` (ast.rs:813)
  - `span` (ast.rs:1000)
  - `verb_name` (ast.rs:1014)
  - `to_dsl_string` (ast.rs:1028)
  - `span` (ast.rs:1089)
  - `to_dsl_string` (ast.rs:1103)
  - `to_dsl_string` (ast.rs:1143)
  - `span` (ast.rs:1169)
  - `to_dsl_string` (ast.rs:1178)
  - `to_dsl_string` (ast.rs:1200)
  - `to_dsl_string` (ast.rs:1255)
  - `to_dsl_string` (ast.rs:1319)
  - `min_confidence` (ast.rs:1340)
  - `from_score` (ast.rs:1350)
  - `to_dsl_string` (ast.rs:1381)
  - `mime_type` (ast.rs:1412)
  - `primary_column` (types.rs:1411)
  - `is_simple` (types.rs:1419)
  - `all_columns` (types.rs:1424)
  - `discriminators` (types.rs:1438)
  - `min_confidence` (types.rs:1465)
  - `to_sexpr` (types.rs:1473)
  - `to_sexpr` (types.rs:1616)
  - `arg_name` (types.rs:1830)
  - `entity_uuid` (resource_dependency.rs:96)
  - `entity_uuid_binding` (resource_dependency.rs:105)
  - `natural_key` (resource_dependency.rs:113)
  - `compile_resolved_entity` (resource_dependency.rs:169)
  - `binding_resolved_entity` (resource_dependency.rs:178)
  - `runtime_create_natural_key` (resource_dependency.rs:187)
  - `verb_path` (validator.rs:53)
  - `is_clean` (validator.rs:301)
  - `is_clean` (dag_validator.rs:479)

4. FACADE exports (70 items):
  - `EffectClass` (executable_plan.rs:99)
  - `TransactionPolicy` (executable_plan.rs:145)
  - `CompileStep` (compiler.rs:39)
  - `CompiledSteps` (compiler.rs:48)
  - `compile_to_steps` (compiler.rs:72)
  - `NodeId` (execution_dag.rs:31)
  - `BindingSlotId` (execution_dag.rs:41)
  - `JoinBarrierMode` (execution_dag.rs:60)
  - `DagEdge` (execution_dag.rs:78)
  - `PopulatedExecutionDag` (execution_dag.rs:189)
  - `Statement` (ast.rs:74)
  - `VerbCall` (ast.rs:99)
  - `parse_program` (parser.rs:58)
  - `EvaluationContext` (escalation.rs:29)
  - `compute_effective_tier` (escalation.rs:128)
  - `VerbsConfig` (types.rs:14)
  - `VerbFlavour` (types.rs:238)
  - `StateEffect` (types.rs:309)
  - `ExternalEffect` (types.rs:325)
  - `ConsequenceTier` (types.rs:354)
  - `SlotType` (types.rs:1218)
  - `Dag` (dag.rs:31)
  - `Phase` (dag.rs:95)
  - `SlotStateMachine` (dag.rs:273)
  - `PredicateBinding` (dag.rs:328)
  - `LoadedDag` (dag.rs:746)
  - `load_dags_from_dir` (dag.rs:755)
  - `RunbookStep` (runbook_composition.rs:39)
  - `AggregationRule` (runbook_composition.rs:67)
  - `CrossScopeRule` (runbook_composition.rs:139)
  - `compute_runbook_tier` (runbook_composition.rs:212)
  - `StructuralError` (validator.rs:72)
  - `ValidationContext` (validator.rs:317)
  - `validate_verbs_config` (validator.rs:681)
  - `DagError` (dag_validator.rs:51)
  - `DagWarning` (dag_validator.rs:385)
  - `DagValidationContext` (dag_validator.rs:489)
  - `validate_dags_with_context` (dag_validator.rs:518)
  - `validate_resolved_template_gate_metadata` (dag_validator.rs:549)
  - `validate_constellation_map_schema_coordination` (dag_validator.rs:633)
  - `harden_schema_coordination_warnings` (dag_validator.rs:697)
  - `GreenWhenExclusionReason` (green_when_coverage.rs:24)
  - `green_when_coverage_for_dags` (green_when_coverage.rs:39)
  - `green_when_coverage_for_dag` (green_when_coverage.rs:54)
  - `green_when_coverage_summary` (green_when_coverage.rs:82)
  - `ConfigLoader` (loader.rs:12)
  - `Predicate` (ast.rs:7)
  - `EntityRef` (ast.rs:63)
  - `EntitySetRef` (ast.rs:82)
  - `EntityQualifier` (ast.rs:93)
  - `RelationScope` (ast.rs:100)
  - `Validity` (ast.rs:113)
  - `CmpOp` (ast.rs:126)
  - `AttrValue` (ast.rs:143)
  - `parse_green_when` (parser.rs:35)
  - `FrontierFact` (mod.rs:14)
  - `HydrateFrontierError` (mod.rs:20)
  - `EntityRef` (mod.rs:27)
  - `GreenWhenStatus` (mod.rs:52)
  - `InvalidFactDetail` (mod.rs:76)
  - `CompletenessAssertionStatus` (mod.rs:110)
  - `VersionHash` (version.rs:5)
  - `ManifestOptions` (manifest.rs:5)
  - `ResolverManifest` (manifest.rs:26)
  - `ResolvedSource` (mod.rs:22)
  - `SlotProvenance` (mod.rs:30)
  - `ResolverProvenance` (mod.rs:35)
  - `ResolvedTemplate` (mod.rs:44)
  - `ResolvedSlot` (mod.rs:66)
  - `ResolvedTransition` (mod.rs:97)

5. TEST-RELOCATION set (117 items):
  - `error` (diagnostics.rs:134)
  - `info` (diagnostics.rs:170)
  - `EffectClass` (executable_plan.rs:99)
  - `TransactionPolicy` (executable_plan.rs:145)
  - `from_effect_classes` (executable_plan.rs:166)
  - `CompileStep` (compiler.rs:39)
  - `CompiledSteps` (compiler.rs:48)
  - `is_ok` (compiler.rs:58)
  - `compile_to_steps` (compiler.rs:72)
  - `ast` (lib.rs:15)
  - `compiler` (lib.rs:17)
  - `config` (lib.rs:18)
  - `execution_dag` (lib.rs:21)
  - `frontier` (lib.rs:22)
  - `parser` (lib.rs:23)
  - `resolver` (lib.rs:24)
  - `NodeId` (execution_dag.rs:31)
  - `BindingSlotId` (execution_dag.rs:41)
  - `JoinBarrierMode` (execution_dag.rs:60)
  - `DagEdge` (execution_dag.rs:78)
  - `PopulatedExecutionDag` (execution_dag.rs:189)
  - `add_edge` (execution_dag.rs:198)
  - `ordering_pairs` (execution_dag.rs:211)
  - `display` (binding_context.rs:79)
  - `get` (binding_context.rs:111)
  - `contains` (binding_context.rs:116)
  - `all` (binding_context.rs:126)
  - `insert` (binding_context.rs:131)
  - `is_empty` (binding_context.rs:148)
  - `len` (binding_context.rs:153)
  - `Statement` (ast.rs:74)
  - `VerbCall` (ast.rs:99)
  - `full_name` (ast.rs:144)
  - `string` (ast.rs:357)
  - `entity_ref` (ast.rs:367)
  - `as_symbol` (ast.rs:487)
  - `len` (ast.rs:669)
  - `is_empty` (ast.rs:674)
  - `all` (ast.rs:1282)
  - `extension` (ast.rs:1402)
  - `parse_program` (parser.rs:58)
  - `EvaluationContext` (escalation.rs:29)
  - `with_arg` (escalation.rs:44)
  - `with_entity_attr` (escalation.rs:49)
  - `with_flag` (escalation.rs:62)
  - `compute_effective_tier` (escalation.rs:128)
  - `VerbsConfig` (types.rs:14)
  - `VerbFlavour` (types.rs:238)
  - `StateEffect` (types.rs:309)
  - `ExternalEffect` (types.rs:325)
  - `ConsequenceTier` (types.rs:354)
  - `Dag` (dag.rs:31)
  - `Phase` (dag.rs:95)
  - `SlotStateMachine` (dag.rs:273)
  - `PredicateBinding` (dag.rs:328)
  - `LoadedDag` (dag.rs:746)
  - `load_dags_from_dir` (dag.rs:755)
  - `RunbookStep` (runbook_composition.rs:39)
  - `AggregationRule` (runbook_composition.rs:67)
  - `CrossScopeRule` (runbook_composition.rs:139)
  - `compute_runbook_tier` (runbook_composition.rs:212)
  - `StructuralError` (validator.rs:72)
  - `error_count` (validator.rs:304)
  - `ValidationContext` (validator.rs:317)
  - `validate_verbs_config` (validator.rs:681)
  - `DagError` (dag_validator.rs:51)
  - `DagWarning` (dag_validator.rs:385)
  - `error_count` (dag_validator.rs:482)
  - `DagValidationContext` (dag_validator.rs:489)
  - `validate_dags_with_context` (dag_validator.rs:518)
  - `validate_resolved_template_gate_metadata` (dag_validator.rs:549)
  - `validate_constellation_map_schema_coordination` (dag_validator.rs:633)
  - `harden_schema_coordination_warnings` (dag_validator.rs:697)
  - `dag` (mod.rs:21)
  - `dag_validator` (mod.rs:22)
  - `effect_class` (mod.rs:23)
  - `loader` (mod.rs:26)
  - `types` (mod.rs:31)
  - `GreenWhenExclusionReason` (green_when_coverage.rs:24)
  - `green_when_coverage_for_dags` (green_when_coverage.rs:39)
  - `green_when_coverage_for_dag` (green_when_coverage.rs:54)
  - `green_when_coverage_summary` (green_when_coverage.rs:82)
  - `ConfigLoader` (loader.rs:12)
  - `from_env` (loader.rs:31)
  - `config_dir` (loader.rs:97)
  - `load_verbs` (loader.rs:109)
  - `ast` (mod.rs:3)
  - `parser` (mod.rs:4)
  - `Predicate` (ast.rs:7)
  - `EntityRef` (ast.rs:63)
  - `EntitySetRef` (ast.rs:82)
  - `EntityQualifier` (ast.rs:93)
  - `RelationScope` (ast.rs:100)
  - `Validity` (ast.rs:113)
  - `CmpOp` (ast.rs:126)
  - `AttrValue` (ast.rs:143)
  - `parse_green_when` (parser.rs:35)
  - `FrontierFact` (mod.rs:14)
  - `HydrateFrontierError` (mod.rs:20)
  - `EntityRef` (mod.rs:27)
  - `GreenWhenStatus` (mod.rs:52)
  - `InvalidFactDetail` (mod.rs:76)
  - `CompletenessAssertionStatus` (mod.rs:110)
  - `VersionHash` (version.rs:5)
  - `ManifestOptions` (manifest.rs:5)
  - `with_required_slots` (manifest.rs:10)
  - `ResolverManifest` (manifest.rs:26)
  - `from_template` (manifest.rs:38)
  - `to_text` (manifest.rs:66)
  - `manifest` (mod.rs:3)
  - `ResolvedSource` (mod.rs:22)
  - `SlotProvenance` (mod.rs:30)
  - `ResolverProvenance` (mod.rs:35)
  - `ResolvedTemplate` (mod.rs:44)
  - `slot_mut` (mod.rs:60)
  - `ResolvedSlot` (mod.rs:66)
  - `ResolvedTransition` (mod.rs:97)

6. SERDE-CONSTRUCTED set (1 items):
  - `CsgRulesConfig` (types.rs:26)

7. UNVERIFIED set (24 items):
  - `new` (diagnostics.rs:69)
  - `new` (executable_plan.rs:46)
  - `new` (viewport_parser.rs:49)
  - `new` (execution_dag.rs:44)
  - `new` (execution_dag.rs:194)
  - `new` (binding_context.rs:99)
  - `new` (ast.rs:656)
  - `parse` (ast.rs:1212)
  - `parse` (ast.rs:1268)
  - `parse` (ast.rs:1329)
  - `parse` (ast.rs:1391)
  - `new` (escalation.rs:40)
  - `parse` (types.rs:1399)
  - `parse` (types.rs:1537)
  - `as_str` (types.rs:1790)
  - `name` (runbook_composition.rs:93)
  - `tier` (runbook_composition.rs:101)
  - `matches` (runbook_composition.rs:109)
  - `name` (runbook_composition.rs:161)
  - `tier` (runbook_composition.rs:169)
  - `matches` (runbook_composition.rs:177)
  - `verb` (validator.rs:47)
  - `new` (loader.rs:17)
  - `slot` (mod.rs:56)


## Ledgers
- Deleted tests:     None (Tranche B)
- Dead-code harvest: None (no suppression)
- Unverified:        None (Tranche D)

## Deviations & Decisions
- **viewport_parser**: Resolved conflict where the third-pass report categorized `viewport_parser` as both DEAD and used internally. Kept as `pub(crate) mod` and removed from `DELETE` set.
- **binding_context** & **executable_plan**: Reconciled modules so their mod entries in `lib.rs` are `pub(crate) mod` to support their root re-exports.

## Invariant attestation
- E0 no live-body edits: PASS — diff = visibility + deletions + re-exports + test moves only (No changes made)
- E1 no globs introduced:  PASS — zero globs introduced
- E2 no allow(dead_code):  PASS — zero allow(dead_code) introduced

## Next
- Next tranche: Tranche A (dsl_types lockdown)
- Entry preconditions: Workspace is clean, baseline captured, and Tranche 0 is approved.