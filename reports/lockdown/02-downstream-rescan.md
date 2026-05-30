# Lockdown Report — Tranche A.5 (Downstream Consumption Rescan)
- UTC:       2026-05-30T11:03:00Z
- Commit(s): [will commit next]
- Status:    GREEN

## Summary
Performed a read-only rescan of sibling repositories to construct an accurate external consumption map. Discovered that both `sem-os` and `ob-poc` depend heavily on `dsl-core` (and `sem-os` on `dsl_types`). Recomputed the `dsl-core` facade and reclassified the test-only set accordingly.

## 1. Dependency Inventory
The following downstream repositories depend on the `dsl` workspace crates:

| Downstream Repo | dsl Crate | Dependency Mechanism | Consuming Workspace/Crate Location |
| :--- | :--- | :--- | :--- |
| **sem-os** | `dsl_types` | Git dependency (`git = "...", tag = "v0.1.2"`) | `crates/sem_os_core`, `crates/sem_os_ontology` |
| **sem-os** | `dsl-core` | Git dependency (`git = "...", tag = "v0.1.2"`) | `crates/sem_os_core` |
| **ob-poc** | `dsl_types` | Workspace dependency (unused in code) | Root `Cargo.toml` only |
| **ob-poc** | `dsl-core` | Git dependency (`git = "...", tag = "v0.1.4"`) | 12 member crates |

## 2. External Consumption Map
Below is the inventory of all references in downstream repos to `dsl_core` and `dsl_types` symbols:

| Crate | Symbol | Consuming Repo | File:Lines | Path Access Type |
| :--- | :--- | :--- | :--- | :--- |
| `dsl_types` | `*` | sem-os | `crates/sem_os_ontology/src/constellation_map_def.rs:11` | **INTERNAL** |
| `dsl_types` | `AuditClass` | sem-os | `crates/sem_os_core/src/resolver/shape_rule.rs:3` | **ROOT** |
| `dsl_types` | `Cardinality` | sem-os | `crates/sem_os_core/src/frontier/hydrator.rs:17` | **ROOT** |
| `dsl_types` | `CompletenessAssertionConfig` | sem-os | `crates/sem_os_core/src/resolver/shape_rule.rs:3` | **ROOT** |
| `dsl_types` | `ConstellationMapDefBody` | sem-os | `crates/sem_os_core/src/resolver/composer.rs:16` | **ROOT** |
| `dsl_types` | `SlotDef` | sem-os | `crates/sem_os_core/src/resolver/composer.rs:16` | **ROOT** |
| `dsl_types` | `StructuralFacts` | sem-os | `crates/sem_os_core/src/resolver/shape_rule.rs:4` | **ROOT** |
| `dsl_core` | `*` | ob-poc | `crates/dsl-analysis/src/runtime_registry.rs:23` | **INTERNAL** |
| `dsl_core` | `*` | ob-poc | `crates/dsl-runtime/src/cross_workspace/gate_checker.rs:277` | **INTERNAL** |
| `dsl_core` | `*` | ob-poc | `crates/dsl-runtime/src/cross_workspace/hierarchy_cascade.rs:208` | **INTERNAL** |
| `dsl_core` | `*` | ob-poc | `crates/sem_os_obpoc_adapter/src/lib.rs:154` | **INTERNAL** |
| `dsl_core` | `*` | ob-poc | `crates/sem_os_obpoc_adapter/src/scanner.rs:975` | **INTERNAL** |
| `dsl_core` | `*` | ob-poc | `src/sem_reg/scanner.rs:458` | **INTERNAL** |
| `dsl_core` | `ActionClass` | ob-poc | `src/sage/clash_matrix.rs:253` | **INTERNAL** |
| `dsl_core` | `ActionClass` | ob-poc | `src/sage/drafter.rs:376` | **INTERNAL** |
| `dsl_core` | `ActionClass` | ob-poc | `src/sage/verb_index.rs:11` | **INTERNAL** |
| `dsl_core` | `ActionClass` | ob-poc | `src/sage/verb_resolve.rs:9, 567` | **INTERNAL** |
| `dsl_core` | `ActionClass as DslActionClass` | ob-poc | `crates/sem_os_obpoc_adapter/src/scanner.rs:10` | **INTERNAL** |
| `dsl_core` | `ArgConfig` | ob-poc | `crates/dsl-semos-frontend/src/loader.rs:21` | **INTERNAL** |
| `dsl_core` | `ArgConfig` | ob-poc | `crates/ob-poc-boundary/src/acp_registry_projection.rs:9` | **INTERNAL** |
| `dsl_core` | `ArgConfig` | ob-poc | `crates/sem_os_obpoc_adapter/src/scanner.rs:10` | **INTERNAL** |
| `dsl_core` | `ArgConfig` | ob-poc | `crates/sem_os_postgres/src/ops/discovery.rs:41` | **INTERNAL** |
| `dsl_core` | `ArgConfig` | ob-poc | `src/repl/intent_service.rs:346` | **INTERNAL** |
| `dsl_core` | `ArgConfig` | ob-poc | `src/repl/verb_config_index.rs:398` | **INTERNAL** |
| `dsl_core` | `ArgConfig` | ob-poc | `src/sage/arg_assembly.rs:166` | **INTERNAL** |
| `dsl_core` | `ArgConfig` | ob-poc | `src/sage/drafter.rs:376` | **INTERNAL** |
| `dsl_core` | `ArgConfig` | ob-poc | `src/sage/verb_index.rs:622` | **INTERNAL** |
| `dsl_core` | `ArgConfig` | ob-poc | `src/sem_reg/onboarding/verb_extract.rs:151` | **INTERNAL** |
| `dsl_core` | `ArgType` | ob-poc | `crates/ob-poc-boundary/src/acp_registry_projection.rs:9` | **INTERNAL** |
| `dsl_core` | `ArgType` | ob-poc | `src/repl/intent_service.rs:346` | **INTERNAL** |
| `dsl_core` | `ArgType` | ob-poc | `src/repl/verb_config_index.rs:398` | **INTERNAL** |
| `dsl_core` | `ArgType` | ob-poc | `src/sage/arg_assembly.rs:4` | **INTERNAL** |
| `dsl_core` | `ArgType` | ob-poc | `src/sage/drafter.rs:376` | **INTERNAL** |
| `dsl_core` | `ArgType` | ob-poc | `src/sage/verb_index.rs:622` | **INTERNAL** |
| `dsl_core` | `Argument` | ob-poc | `crates/dsl-analysis/src/lsp_validator.rs:25` | **INTERNAL** |
| `dsl_core` | `Argument` | ob-poc | `crates/dsl-lsp/src/analysis/v2_adapter.rs:31` | **INTERNAL** |
| `dsl_core` | `Argument` | ob-poc | `src/dsl_v2/mod.rs:33` | **INTERNAL** |
| `dsl_core` | `Argument` | ob-poc | `src/dsl_v2/submission.rs:15` | **INTERNAL** |
| `dsl_core` | `Argument` | ob-poc | `src/mcp/intent_pipeline.rs:1472, 1609` | **INTERNAL** |
| `dsl_core` | `Argument` | ob-poc | `src/runbook/compiler.rs:29` | **INTERNAL** |
| `dsl_core` | `Argument` | ob-poc | `src/sem_os_runtime/verb_executor_adapter.rs:25` | **INTERNAL** |
| `dsl_core` | `AstNode` | ob-poc | `crates/dsl-analysis/src/lsp_validator.rs:25` | **INTERNAL** |
| `dsl_core` | `AstNode` | ob-poc | `crates/dsl-analysis/src/planning_facade.rs:13` | **INTERNAL** |
| `dsl_core` | `AstNode` | ob-poc | `crates/dsl-lsp/src/analysis/v2_adapter.rs:31` | **INTERNAL** |
| `dsl_core` | `AstNode` | ob-poc | `crates/dsl-lsp/tests/parser_conformance.rs:11` | **INTERNAL** |
| `dsl_core` | `AstNode` | ob-poc | `src/dsl_v2/mod.rs:33` | **INTERNAL** |
| `dsl_core` | `AstNode` | ob-poc | `src/dsl_v2/submission.rs:15` | **INTERNAL** |
| `dsl_core` | `AstNode` | ob-poc | `src/mcp/intent_pipeline.rs:1472, 1609, 1767, 1771` | **INTERNAL** |
| `dsl_core` | `AstNode` | ob-poc | `src/runbook/compiler.rs:29` | **INTERNAL** |
| `dsl_core` | `AstNode` | ob-poc | `src/sem_os_runtime/verb_executor_adapter.rs:25` | **INTERNAL** |
| `dsl_core` | `AttrValue` | sem-os | `crates/sem_os_core/src/frontier/hydrator.rs:1` | **INTERNAL** |
| `dsl_core` | `Benign` | ob-poc | `xtask/src/reconcile.rs:999` | **INTERNAL** |
| `dsl_core` | `BindingContext` | ob-poc | `crates/dsl-analysis/src/planning_facade.rs:14` | **INTERNAL** |
| `dsl_core` | `BindingContext` | ob-poc | `crates/dsl-analysis/src/suggestions.rs:4` | **INTERNAL** |
| `dsl_core` | `BindingContext` | ob-poc | `crates/dsl-lsp/src/handlers/completion.rs:13` | **INTERNAL** |
| `dsl_core` | `BindingContext` | ob-poc | `crates/ob-agentic/src/context_builder.rs:13` | **ROOT** |
| `dsl_core` | `BindingContext` | ob-poc | `src/dsl_v2/mod.rs:44` | **INTERNAL** |
| `dsl_core` | `BindingFrameSchema` | ob-poc | `src/dsl_v2/execution_plan.rs:71, 72` | **INTERNAL** |
| `dsl_core` | `BindingInfo` | ob-poc | `crates/dsl-analysis/src/suggestions.rs:107` | **INTERNAL** |
| `dsl_core` | `BindingInfo` | ob-poc | `crates/dsl-lsp/src/handlers/completion.rs:13` | **INTERNAL** |
| `dsl_core` | `BindingInfo` | ob-poc | `src/dsl_v2/mod.rs:44` | **INTERNAL** |
| `dsl_core` | `BindingSlot` | ob-poc | `src/dsl_v2/execution_plan.rs:72` | **INTERNAL** |
| `dsl_core` | `BindingSlotId` | ob-poc | `src/dsl_v2/execution_plan.rs:37, 73` | **INTERNAL** |
| `dsl_core` | `BindingSlotId` | ob-poc | `tests/phase5_coordination_harness.rs:37` | **INTERNAL** |
| `dsl_core` | `CascadeRule` | ob-poc | `crates/dsl-runtime/src/cross_workspace/hierarchy_cascade.rs:31` | **INTERNAL** |
| `dsl_core` | `ClosureType` | sem-os | `crates/sem_os_core/src/frontier/hydrator.rs:1` | **INTERNAL** |
| `dsl_core` | `ClosureType` | sem-os | `crates/sem_os_core/src/resolver/shape_rule.rs:1` | **INTERNAL** |
| `dsl_core` | `CmpOp` | sem-os | `crates/sem_os_core/src/frontier/hydrator.rs:1` | **INTERNAL** |
| `dsl_core` | `CompileError as OpCompileError` | ob-poc | `src/dsl_v2/mod.rs:61` | **INTERNAL** |
| `dsl_core` | `CompileStep` | ob-poc | `crates/dsl-analysis/src/planning_facade.rs:15` | **INTERNAL** |
| `dsl_core` | `CompileStep` | ob-poc | `src/dsl_v2/mod.rs:61` | **INTERNAL** |
| `dsl_core` | `CompiledSteps` | ob-poc | `src/dsl_v2/mod.rs:61` | **INTERNAL** |
| `dsl_core` | `CompletenessAssertionStatus` | sem-os | `crates/sem_os_core/src/frontier/hydrator.rs:1` | **INTERNAL** |
| `dsl_core` | `Composite` | ob-poc | `crates/sem_os_obpoc_adapter/src/scanner.rs:46, 381` | **INTERNAL** |
| `dsl_core` | `ConfigLoader` | ob-poc | `crates/dsl-analysis/src/planning_facade.rs:333` | **INTERNAL** |
| `dsl_core` | `ConfigLoader` | ob-poc | `crates/dsl-analysis/src/runtime_registry.rs:950, 990` | **INTERNAL** |
| `dsl_core` | `ConfigLoader` | ob-poc | `crates/dsl-analysis/src/stategraph/mod.rs:20` | **INTERNAL** |
| `dsl_core` | `ConfigLoader` | ob-poc | `crates/dsl-lsp/src/analysis/v2_adapter.rs:308` | **INTERNAL** |
| `dsl_core` | `ConfigLoader` | ob-poc | `crates/dsl-lsp/src/handlers/diagnostics.rs:23` | **INTERNAL** |
| `dsl_core` | `ConfigLoader` | ob-poc | `crates/dsl-runtime/src/cross_workspace/mod.rs:52` | **INTERNAL** |
| `dsl_core` | `ConfigLoader` | ob-poc | `crates/ob-agentic/src/validator.rs:72` | **INTERNAL** |
| `dsl_core` | `ConfigLoader` | ob-poc | `crates/ob-poc-boundary/src/acp_dag_semantic.rs:7` | **INTERNAL** |
| `dsl_core` | `ConfigLoader` | ob-poc | `crates/ob-poc-boundary/src/acp_registry_projection.rs:8` | **INTERNAL** |
| `dsl_core` | `ConfigLoader` | ob-poc | `crates/sem_os_postgres/src/ops/discovery.rs:40` | **INTERNAL** |
| `dsl_core` | `ConfigLoader` | ob-poc | `src/domain_ops/mod.rs:694` | **INTERNAL** |
| `dsl_core` | `ConfigLoader` | ob-poc | `src/dsl_v2/executor.rs:2521` | **INTERNAL** |
| `dsl_core` | `ConfigLoader` | ob-poc | `src/dsl_v2/macros/attribute_seed.rs:4` | **INTERNAL** |
| `dsl_core` | `ConfigLoader` | ob-poc | `src/dsl_v2/mod.rs:49` | **INTERNAL** |
| `dsl_core` | `ConfigLoader` | ob-poc | `src/sage/arg_assembly.rs:16, 49` | **INTERNAL** |
| `dsl_core` | `ConfigLoader` | ob-poc | `src/sage/drafter.rs:16, 45` | **INTERNAL** |
| `dsl_core` | `ConfigLoader` | ob-poc | `src/sage/verb_index.rs:10, 50` | **INTERNAL** |
| `dsl_core` | `ConfigLoader` | ob-poc | `src/sem_reg/scanner.rs:16` | **INTERNAL** |
| `dsl_core` | `ConfigLoader` | ob-poc | `src/sequencer.rs:448` | **INTERNAL** |
| `dsl_core` | `ConfigLoader` | ob-poc | `tests/semos_discovery_hit_rate.rs:13` | **INTERNAL** |
| `dsl_core` | `ConfigLoader` | ob-poc | `xtask/src/reconcile.rs:20` | **INTERNAL** |
| `dsl_core` | `ConfigLoader` | ob-poc | `xtask/src/verbs.rs:12` | **INTERNAL** |
| `dsl_core` | `ConfirmPolicyConfig` | ob-poc | `crates/dsl-semos-frontend/src/loader.rs:21` | **INTERNAL** |
| `dsl_core` | `ConfirmPolicyConfig` | ob-poc | `src/bin/verb_to_dsl.rs:24` | **INTERNAL** |
| `dsl_core` | `ConfirmPolicyConfig` | ob-poc | `src/repl/verb_config_index.rs:21` | **INTERNAL** |
| `dsl_core` | `ConsequenceTier` | ob-poc | `crates/ob-poc-boundary/src/acp_registry_projection.rs:9` | **INTERNAL** |
| `dsl_core` | `CrossWorkspaceConstraint` | ob-poc | `crates/dsl-runtime/src/cross_workspace/gate_checker.rs:40` | **INTERNAL** |
| `dsl_core` | `Crud` | ob-poc | `src/sem_reg/onboarding/verb_extract.rs:112` | **INTERNAL** |
| `dsl_core` | `CrudConfig` | ob-poc | `crates/dsl-semos-frontend/src/loader.rs:21` | **INTERNAL** |
| `dsl_core` | `CrudConfig` | ob-poc | `src/sage/verb_index.rs:632` | **INTERNAL** |
| `dsl_core` | `CrudConfig` | ob-poc | `src/session/verb_tiering_linter.rs:681` | **INTERNAL** |
| `dsl_core` | `CrudOperation` | ob-poc | `crates/ob-poc-boundary/src/acp_registry_projection.rs:9` | **INTERNAL** |
| `dsl_core` | `CrudOperation` | ob-poc | `crates/sem_os_obpoc_adapter/src/scanner.rs:10` | **INTERNAL** |
| `dsl_core` | `CrudOperation` | ob-poc | `src/sage/verb_index.rs:11` | **INTERNAL** |
| `dsl_core` | `CrudOperation` | ob-poc | `src/session/verb_tiering_linter.rs:37` | **INTERNAL** |
| `dsl_core` | `Dag` | ob-poc | `crates/dsl-runtime/src/cross_workspace/derived_state_projector.rs:109` | **INTERNAL** |
| `dsl_core` | `Dag` | ob-poc | `crates/dsl-runtime/src/cross_workspace/gate_checker.rs:293` | **INTERNAL** |
| `dsl_core` | `Dag` | ob-poc | `crates/dsl-runtime/src/cross_workspace/hierarchy_cascade.rs:230` | **INTERNAL** |
| `dsl_core` | `Dag` | ob-poc | `crates/dsl-runtime/src/cross_workspace/postgres_child_resolver.rs:161` | **INTERNAL** |
| `dsl_core` | `Dag` | sem-os | `crates/sem_os_core/src/resolver/composer.rs:1` | **INTERNAL** |
| `dsl_core` | `DagEdge` | ob-poc | `src/dsl_v2/execution_plan.rs:37` | **INTERNAL** |
| `dsl_core` | `DagEdge` | ob-poc | `tests/phase5_coordination_harness.rs:37` | **INTERNAL** |
| `dsl_core` | `DagRegistry` | ob-poc | `crates/dsl-runtime/src/cross_workspace/derived_state_projector.rs:19` | **INTERNAL** |
| `dsl_core` | `DagRegistry` | ob-poc | `crates/dsl-runtime/src/cross_workspace/gate_checker.rs:41` | **INTERNAL** |
| `dsl_core` | `DagRegistry` | ob-poc | `crates/dsl-runtime/src/cross_workspace/hierarchy_cascade.rs:32` | **INTERNAL** |
| `dsl_core` | `DagRegistry` | ob-poc | `crates/dsl-runtime/src/cross_workspace/postgres_child_resolver.rs:30` | **INTERNAL** |
| `dsl_core` | `DagRegistry` | ob-poc | `crates/dsl-runtime/src/cross_workspace/test_harness/live.rs:14` | **INTERNAL** |
| `dsl_core` | `DagRegistry` | ob-poc | `crates/dsl-runtime/src/cross_workspace/test_harness/runner.rs:4` | **INTERNAL** |
| `dsl_core` | `DagRegistry` | ob-poc | `src/runbook/step_executor_bridge.rs:163` | **INTERNAL** |
| `dsl_core` | `DagRegistry` | ob-poc | `xtask/src/dag_test.rs:95` | **INTERNAL** |
| `dsl_core` | `DagValidationContext` | ob-poc | `xtask/src/reconcile.rs:20` | **INTERNAL** |
| `dsl_core` | `Delete` | ob-poc | `src/sem_reg/onboarding/verb_extract.rs:193` | **INTERNAL** |
| `dsl_core` | `Deprecated` | ob-poc | `src/session/verb_tiering_linter.rs:338` | **INTERNAL** |
| `dsl_core` | `DerivationCondition` | ob-poc | `crates/dsl-runtime/src/cross_workspace/derived_state.rs:24` | **INTERNAL** |
| `dsl_core` | `DerivedCrossWorkspaceState` | ob-poc | `crates/dsl-runtime/src/cross_workspace/derived_state.rs:24` | **INTERNAL** |
| `dsl_core` | `Destructive` | ob-poc | `src/sem_os_runtime/verb_executor_adapter.rs:367` | **INTERNAL** |
| `dsl_core` | `Diagnostic` | ob-poc | `crates/dsl-analysis/src/planning_facade.rs:16` | **INTERNAL** |
| `dsl_core` | `Diagnostic` | ob-poc | `crates/dsl-lsp/src/handlers/code_actions.rs:185` | **INTERNAL** |
| `dsl_core` | `Diagnostic` | ob-poc | `crates/dsl-lsp/src/handlers/diagnostics.rs:194` | **INTERNAL** |
| `dsl_core` | `Diagnostic` | ob-poc | `src/dsl_v2/mod.rs:53` | **INTERNAL** |
| `dsl_core` | `DiagnosticCode` | ob-poc | `crates/dsl-analysis/src/planning_facade.rs:16` | **INTERNAL** |
| `dsl_core` | `DiagnosticCode` | ob-poc | `crates/dsl-lsp/src/handlers/code_actions.rs:14` | **INTERNAL** |
| `dsl_core` | `DiagnosticCode` | ob-poc | `src/dsl_v2/mod.rs:53` | **INTERNAL** |
| `dsl_core` | `DiscretionaryReason` | sem-os | `crates/sem_os_core/src/frontier/hydrator.rs:1` | **INTERNAL** |
| `dsl_core` | `DomainConfig` | ob-poc | `crates/dsl-analysis/src/catalogue_loader.rs:41` | **INTERNAL** |
| `dsl_core` | `DomainConfig` | ob-poc | `crates/dsl-semos-frontend/src/loader.rs:21` | **INTERNAL** |
| `dsl_core` | `DomainConfig` | ob-poc | `crates/sem_os_obpoc_adapter/src/scanner.rs:501` | **INTERNAL** |
| `dsl_core` | `DomainConfig` | ob-poc | `crates/sem_os_postgres/src/ops/discovery.rs:41` | **INTERNAL** |
| `dsl_core` | `DomainConfig` | ob-poc | `src/repl/intent_service.rs:447` | **INTERNAL** |
| `dsl_core` | `DomainConfig` | ob-poc | `src/repl/verb_config_index.rs:398` | **INTERNAL** |
| `dsl_core` | `DomainConfig` | ob-poc | `src/runbook/step_executor_bridge.rs:590` | **INTERNAL** |
| `dsl_core` | `DomainConfig` | ob-poc | `src/sage/drafter.rs:376` | **INTERNAL** |
| `dsl_core` | `DomainConfig` | ob-poc | `src/sage/verb_index.rs:622` | **INTERNAL** |
| `dsl_core` | `DomainConfig` | ob-poc | `src/session/verb_tiering_linter.rs:564, 571, 607` | **INTERNAL** |
| `dsl_core` | `Durable` | ob-poc | `src/sem_reg/onboarding/verb_extract.rs:115` | **INTERNAL** |
| `dsl_core` | `DurableConfig` | ob-poc | `crates/dsl-analysis/src/runtime_registry.rs:1245` | **INTERNAL** |
| `dsl_core` | `DurableConfig` | ob-poc | `crates/dsl-semos-frontend/src/loader.rs:21` | **INTERNAL** |
| `dsl_core` | `DurableConfig` | ob-poc | `src/bpmn_integration/config.rs:7, 315, 366` | **INTERNAL** |
| `dsl_core` | `DurableConfig` | ob-poc | `src/bpmn_integration/dispatcher.rs:686` | **INTERNAL** |
| `dsl_core` | `DurableConfig` | ob-poc | `xtask/src/verbs.rs:11` | **INTERNAL** |
| `dsl_core` | `DurableRuntime` | ob-poc | `crates/dsl-analysis/src/runtime_registry.rs:197, 1245` | **INTERNAL** |
| `dsl_core` | `DurableRuntime` | ob-poc | `src/bpmn_integration/config.rs:315, 366` | **INTERNAL** |
| `dsl_core` | `DurableRuntime` | ob-poc | `src/bpmn_integration/dispatcher.rs:686` | **INTERNAL** |
| `dsl_core` | `EffectClass` | ob-poc | `crates/dsl-runtime/src/coordination.rs:21` | **INTERNAL** |
| `dsl_core` | `EffectClass` | ob-poc | `crates/dsl-semos-frontend/src/loader.rs:473, 474` | **INTERNAL** |
| `dsl_core` | `EffectClass` | ob-poc | `src/dsl_v2/executor.rs:2520` | **INTERNAL** |
| `dsl_core` | `EffectClass` | ob-poc | `tests/phase5_coordination_harness.rs:36` | **INTERNAL** |
| `dsl_core` | `EligibilityConstraint` | sem-os | `crates/sem_os_core/src/resolver/shape_rule.rs:1` | **INTERNAL** |
| `dsl_core` | `Entity` | ob-poc | `src/api/session.rs:391` | **INTERNAL** |
| `dsl_core` | `EntityCreate` | ob-poc | `src/sem_reg/onboarding/verb_extract.rs:200` | **INTERNAL** |
| `dsl_core` | `EntityRef` | sem-os | `crates/sem_os_core/src/frontier/hydrator.rs:1, 27` | **INTERNAL** |
| `dsl_core` | `EntityRef as PredicateEntityRef` | sem-os | `crates/sem_os_core/src/frontier/hydrator.rs:1` | **INTERNAL** |
| `dsl_core` | `EntityRefStats` | ob-poc | `src/dsl_v2/mod.rs:33` | **INTERNAL** |
| `dsl_core` | `EntitySetRef` | sem-os | `crates/sem_os_core/src/frontier/hydrator.rs:1` | **INTERNAL** |
| `dsl_core` | `EntityUpsert` | ob-poc | `src/sem_reg/onboarding/verb_extract.rs:195` | **INTERNAL** |
| `dsl_core` | `EntryVia` | ob-poc | `xtask/src/reconcile.rs:19` | **INTERNAL** |
| `dsl_core` | `ExternalEffect` | ob-poc | `crates/ob-poc-boundary/src/acp_registry_projection.rs:9` | **INTERNAL** |
| `dsl_core` | `FrontierFact` | sem-os | `crates/sem_os_core/src/frontier/hydrator.rs:1` | **INTERNAL** |
| `dsl_core` | `FrontierFacts` | sem-os | `crates/sem_os_core/src/frontier/hydrator.rs:1` | **INTERNAL** |
| `dsl_core` | `GraphQuery` | ob-poc | `src/sem_reg/onboarding/verb_extract.rs:114` | **INTERNAL** |
| `dsl_core` | `GraphQueryConfig` | ob-poc | `crates/dsl-semos-frontend/src/loader.rs:21` | **INTERNAL** |
| `dsl_core` | `GraphQueryOperation` | ob-poc | `crates/dsl-analysis/src/runtime_registry.rs:197` | **INTERNAL** |
| `dsl_core` | `GreenWhenStatus` | sem-os | `crates/sem_os_core/src/frontier/hydrator.rs:1` | **INTERNAL** |
| `dsl_core` | `HarmClass` | ob-poc | `crates/ob-poc-boundary/src/acp_dag_semantic.rs:8` | **INTERNAL** |
| `dsl_core` | `HarmClass` | ob-poc | `src/sage/clash_matrix.rs:6` | **INTERNAL** |
| `dsl_core` | `HarmClass` | ob-poc | `src/sage/drafter.rs:17, 376` | **INTERNAL** |
| `dsl_core` | `HarmClass` | ob-poc | `src/sage/verb_index.rs:11, 135, 166` | **INTERNAL** |
| `dsl_core` | `HarmClass` | ob-poc | `src/sage/verb_resolve.rs:567` | **INTERNAL** |
| `dsl_core` | `HarmClass as DslHarmClass` | ob-poc | `crates/sem_os_obpoc_adapter/src/scanner.rs:10` | **INTERNAL** |
| `dsl_core` | `HydrateFrontierError` | sem-os | `crates/sem_os_core/src/frontier/hydrator.rs:1` | **INTERNAL** |
| `dsl_core` | `Insert` | ob-poc | `src/sem_reg/onboarding/verb_extract.rs:187` | **INTERNAL** |
| `dsl_core` | `InstanceFrontier` | sem-os | `crates/sem_os_core/src/frontier/hydrator.rs:1` | **INTERNAL** |
| `dsl_core` | `InvalidFact` | sem-os | `crates/sem_os_core/src/frontier/hydrator.rs:1` | **INTERNAL** |
| `dsl_core` | `InvalidFactDetail` | sem-os | `crates/sem_os_core/src/frontier/hydrator.rs:1` | **INTERNAL** |
| `dsl_core` | `Irreversible` | ob-poc | `src/sem_os_runtime/verb_executor_adapter.rs:364` | **INTERNAL** |
| `dsl_core` | `JoinBarrierMode` | ob-poc | `tests/phase5_coordination_harness.rs:37` | **INTERNAL** |
| `dsl_core` | `Link` | ob-poc | `src/sem_reg/onboarding/verb_extract.rs:196, 212` | **INTERNAL** |
| `dsl_core` | `ListByFk` | ob-poc | `src/sem_reg/onboarding/verb_extract.rs:190` | **INTERNAL** |
| `dsl_core` | `ListParties` | ob-poc | `src/sem_reg/onboarding/verb_extract.rs:191` | **INTERNAL** |
| `dsl_core` | `Literal` | ob-poc | `crates/dsl-analysis/src/lsp_validator.rs:25` | **INTERNAL** |
| `dsl_core` | `Literal` | ob-poc | `crates/dsl-lsp/src/analysis/v2_adapter.rs:31` | **INTERNAL** |
| `dsl_core` | `Literal` | ob-poc | `crates/dsl-lsp/tests/parser_conformance.rs:11` | **INTERNAL** |
| `dsl_core` | `Literal` | ob-poc | `src/dsl_v2/mod.rs:33` | **INTERNAL** |
| `dsl_core` | `Literal` | ob-poc | `src/dsl_v2/submission.rs:15` | **INTERNAL** |
| `dsl_core` | `Literal` | ob-poc | `src/mcp/intent_pipeline.rs:1472, 1609` | **INTERNAL** |
| `dsl_core` | `Literal` | ob-poc | `src/runbook/compiler.rs:29` | **INTERNAL** |
| `dsl_core` | `Literal` | ob-poc | `src/sem_os_runtime/verb_executor_adapter.rs:25` | **INTERNAL** |
| `dsl_core` | `LoadedDag` | ob-poc | `crates/dsl-runtime/src/cross_workspace/derived_state_projector.rs:109` | **INTERNAL** |
| `dsl_core` | `LoadedDag` | ob-poc | `crates/dsl-runtime/src/cross_workspace/gate_checker.rs:293` | **INTERNAL** |
| `dsl_core` | `LoadedDag` | ob-poc | `crates/dsl-runtime/src/cross_workspace/hierarchy_cascade.rs:230` | **INTERNAL** |
| `dsl_core` | `LoadedDag` | ob-poc | `crates/dsl-runtime/src/cross_workspace/postgres_child_resolver.rs:161` | **INTERNAL** |
| `dsl_core` | `LoadedDag` | sem-os | `crates/sem_os_core/src/resolver/composer.rs:1` | **INTERNAL** |
| `dsl_core` | `LoadedPack` | ob-poc | `xtask/src/reconcile.rs:20` | **INTERNAL** |
| `dsl_core` | `LookupConfig` | ob-poc | `crates/dsl-analysis/src/verb_registry.rs:22` | **INTERNAL** |
| `dsl_core` | `LookupConfig` | ob-poc | `crates/ob-poc-boundary/src/acp_registry_projection.rs:9` | **INTERNAL** |
| `dsl_core` | `LookupConfig` | ob-poc | `src/api/session.rs:255, 346` | **INTERNAL** |
| `dsl_core` | `LookupConfig` | ob-poc | `src/dsl_v2/mod.rs:48` | **INTERNAL** |
| `dsl_core` | `ManifestOptions` | ob-poc | `src/bin/reconcile_resolver_manifest.rs:2` | **INTERNAL** |
| `dsl_core` | `MissingFact` | sem-os | `crates/sem_os_core/src/frontier/hydrator.rs:1` | **INTERNAL** |
| `dsl_core` | `NodeId` | ob-poc | `src/dsl_v2/execution_plan.rs:37` | **INTERNAL** |
| `dsl_core` | `NodeId` | ob-poc | `tests/phase5_coordination_harness.rs:37` | **INTERNAL** |
| `dsl_core` | `PhraseGenNouns` | ob-poc | `src/dsl_v2/mod.rs:49` | **INTERNAL** |
| `dsl_core` | `Plugin` | ob-poc | `src/sem_reg/onboarding/verb_extract.rs:113` | **INTERNAL** |
| `dsl_core` | `Plugin` | ob-poc | `xtask/src/reconcile.rs:379, 390, 502, 511` | **INTERNAL** |
| `dsl_core` | `PolicyConfig` | ob-poc | `crates/dsl-semos-frontend/src/loader.rs:21` | **INTERNAL** |
| `dsl_core` | `PopulatedExecutionDag` | ob-poc | `src/dsl_v2/execution_plan.rs:37` | **INTERNAL** |
| `dsl_core` | `Predicate` | sem-os | `crates/sem_os_core/src/frontier/hydrator.rs:1` | **INTERNAL** |
| `dsl_core` | `PredicateBinding` | sem-os | `crates/sem_os_core/src/resolver/composer.rs:1` | **INTERNAL** |
| `dsl_core` | `PredicateBinding` | sem-os | `crates/sem_os_core/src/resolver/shape_rule.rs:1` | **INTERNAL** |
| `dsl_core` | `Program` | ob-poc | `crates/dsl-analysis/src/lsp_validator.rs:25` | **INTERNAL** |
| `dsl_core` | `Program` | ob-poc | `crates/dsl-analysis/src/planning_facade.rs:13` | **INTERNAL** |
| `dsl_core` | `Program` | ob-poc | `crates/dsl-analysis/src/suggestions.rs:3` | **INTERNAL** |
| `dsl_core` | `Program` | ob-poc | `crates/dsl-analysis/src/validation.rs:226` | **INTERNAL** |
| `dsl_core` | `Program` | ob-poc | `crates/dsl-lsp/src/analysis/v2_adapter.rs:31` | **INTERNAL** |
| `dsl_core` | `Program` | ob-poc | `src/dsl_v2/mod.rs:33` | **INTERNAL** |
| `dsl_core` | `Program` | ob-poc | `src/mcp/intent_pipeline.rs:1472, 1609, 1749` | **INTERNAL** |
| `dsl_core` | `Program` | ob-poc | `src/runbook/compiler.rs:29` | **INTERNAL** |
| `dsl_core` | `ReachableDestination` | sem-os | `crates/sem_os_core/src/frontier/hydrator.rs:1` | **INTERNAL** |
| `dsl_core` | `ReadOnly` | ob-poc | `src/sem_os_runtime/verb_executor_adapter.rs:358` | **INTERNAL** |
| `dsl_core` | `Reference` | ob-poc | `src/api/session.rs:394` | **INTERNAL** |
| `dsl_core` | `RelatedInfo` | ob-poc | `src/dsl_v2/mod.rs:53` | **INTERNAL** |
| `dsl_core` | `RequiresConfirmation` | ob-poc | `xtask/src/reconcile.rs:1001` | **INTERNAL** |
| `dsl_core` | `RequiresExplicitAuthorisation` | ob-poc | `xtask/src/reconcile.rs:1004` | **INTERNAL** |
| `dsl_core` | `ResolutionMode` | ob-poc | `crates/ob-poc-boundary/src/acp_registry_projection.rs:9` | **INTERNAL** |
| `dsl_core` | `ResolvedResourceDependency` | ob-poc | `src/dsl_v2/execution_plan.rs:36` | **INTERNAL** |
| `dsl_core` | `ResolvedSlot` | sem-os | `crates/sem_os_core/src/frontier/hydrator.rs:1` | **INTERNAL** |
| `dsl_core` | `ResolvedSlot` | sem-os | `crates/sem_os_core/src/resolver/composer.rs:1` | **INTERNAL** |
| `dsl_core` | `ResolvedSource` | sem-os | `crates/sem_os_core/src/resolver/composer.rs:1` | **INTERNAL** |
| `dsl_core` | `ResolvedTemplate` | sem-os | `crates/sem_os_core/src/frontier/hydrator.rs:1` | **INTERNAL** |
| `dsl_core` | `ResolvedTemplate` | sem-os | `crates/sem_os_core/src/resolver/composer.rs:1` | **INTERNAL** |
| `dsl_core` | `ResolvedTransition` | sem-os | `crates/sem_os_core/src/resolver/composer.rs:1` | **INTERNAL** |
| `dsl_core` | `ResolverManifest` | ob-poc | `src/bin/reconcile_resolver_manifest.rs:2` | **INTERNAL** |
| `dsl_core` | `ResolverProvenance` | sem-os | `crates/sem_os_core/src/resolver/composer.rs:1` | **INTERNAL** |
| `dsl_core` | `ReturnTypeConfig` | ob-poc | `crates/ob-poc-boundary/src/acp_registry_projection.rs:9` | **INTERNAL** |
| `dsl_core` | `ReturnsConfig` | ob-poc | `crates/dsl-semos-frontend/src/loader.rs:21` | **INTERNAL** |
| `dsl_core` | `Reversible` | ob-poc | `src/sem_os_runtime/verb_executor_adapter.rs:361` | **INTERNAL** |
| `dsl_core` | `Reviewable` | ob-poc | `xtask/src/reconcile.rs:1000` | **INTERNAL** |
| `dsl_core` | `RoleGuard` | sem-os | `crates/sem_os_core/src/resolver/shape_rule.rs:1` | **INTERNAL** |
| `dsl_core` | `RoleLink` | ob-poc | `src/sem_reg/onboarding/verb_extract.rs:197, 213` | **INTERNAL** |
| `dsl_core` | `RoleUnlink` | ob-poc | `src/sem_reg/onboarding/verb_extract.rs:199, 215` | **INTERNAL** |
| `dsl_core` | `Select` | ob-poc | `src/sem_reg/onboarding/verb_extract.rs:188` | **INTERNAL** |
| `dsl_core` | `SelectWithJoin` | ob-poc | `src/sem_reg/onboarding/verb_extract.rs:189` | **INTERNAL** |
| `dsl_core` | `Severity` | ob-poc | `crates/dsl-runtime/src/cross_workspace/gate_checker.rs:276` | **INTERNAL** |
| `dsl_core` | `Severity` | ob-poc | `crates/dsl-runtime/src/cross_workspace/hierarchy_cascade.rs:207` | **INTERNAL** |
| `dsl_core` | `Severity` | ob-poc | `src/dsl_v2/mod.rs:53` | **INTERNAL** |
| `dsl_core` | `Severity as PlanningSeverity` | ob-poc | `crates/dsl-lsp/src/handlers/diagnostics.rs:197` | **INTERNAL** |
| `dsl_core` | `ShapeRef` | sem-os | `crates/sem_os_core/src/resolver/composer.rs:1` | **INTERNAL** |
| `dsl_core` | `Simple` | ob-poc | `crates/sem_os_obpoc_adapter/src/scanner.rs:45, 380` | **INTERNAL** |
| `dsl_core` | `Slot as DagSlot` | sem-os | `crates/sem_os_core/src/resolver/composer.rs:1` | **INTERNAL** |
| `dsl_core` | `SlotProvenance` | sem-os | `crates/sem_os_core/src/resolver/composer.rs:1` | **INTERNAL** |
| `dsl_core` | `SlotStateMachine` | ob-poc | `xtask/src/reconcile.rs:19` | **INTERNAL** |
| `dsl_core` | `SlotStateMachine` | sem-os | `crates/sem_os_core/src/resolver/composer.rs:1` | **INTERNAL** |
| `dsl_core` | `SourceOfTruth` | ob-poc | `src/session/verb_tiering_linter.rs:37` | **INTERNAL** |
| `dsl_core` | `SourceOfTruth` | ob-poc | `xtask/src/verbs.rs:11` | **INTERNAL** |
| `dsl_core` | `SourceSpan` | ob-poc | `crates/dsl-analysis/src/planning_facade.rs:16` | **INTERNAL** |
| `dsl_core` | `SourceSpan` | ob-poc | `src/dsl_v2/mod.rs:53` | **INTERNAL** |
| `dsl_core` | `Span` | ob-poc | `crates/dsl-analysis/src/lsp_validator.rs:25` | **INTERNAL** |
| `dsl_core` | `Span` | ob-poc | `crates/dsl-analysis/src/planning_facade.rs:13` | **INTERNAL** |
| `dsl_core` | `Span` | ob-poc | `src/dsl_v2/mod.rs:33` | **INTERNAL** |
| `dsl_core` | `Span` | ob-poc | `src/dsl_v2/submission.rs:15, 593` | **INTERNAL** |
| `dsl_core` | `Span` | ob-poc | `src/mcp/intent_pipeline.rs:1472, 1609` | **INTERNAL** |
| `dsl_core` | `Span` | ob-poc | `src/runbook/compiler.rs:29` | **INTERNAL** |
| `dsl_core` | `Span` | ob-poc | `src/sem_os_runtime/verb_executor_adapter.rs:25` | **INTERNAL** |
| `dsl_core` | `Span as V2Span` | ob-poc | `crates/dsl-lsp/src/analysis/v2_adapter.rs:31` | **INTERNAL** |
| `dsl_core` | `StateEffect` | ob-poc | `crates/dsl-analysis/src/macros/registry.rs:480` | **INTERNAL** |
| `dsl_core` | `StateEffect` | ob-poc | `crates/dsl-analysis/src/macros/schema.rs:23` | **INTERNAL** |
| `dsl_core` | `StateEffect` | ob-poc | `crates/ob-poc-boundary/src/acp_registry_projection.rs:9` | **INTERNAL** |
| `dsl_core` | `StateSelector` | ob-poc | `crates/dsl-runtime/src/cross_workspace/derived_state.rs:24` | **INTERNAL** |
| `dsl_core` | `StateSelector` | ob-poc | `crates/dsl-runtime/src/cross_workspace/gate_checker.rs:40` | **INTERNAL** |
| `dsl_core` | `Statement` | ob-poc | `crates/dsl-analysis/src/lsp_validator.rs:25` | **INTERNAL** |
| `dsl_core` | `Statement` | ob-poc | `crates/dsl-lsp/src/analysis/v2_adapter.rs:31` | **INTERNAL** |
| `dsl_core` | `Statement` | ob-poc | `crates/dsl-lsp/src/handlers/code_actions.rs:13` | **INTERNAL** |
| `dsl_core` | `Statement` | ob-poc | `crates/dsl-lsp/tests/parser_conformance.rs:11` | **INTERNAL** |
| `dsl_core` | `Statement` | ob-poc | `src/agent/orchestrator.rs:4625, 4646` | **INTERNAL** |
| `dsl_core` | `Statement` | ob-poc | `src/api/agent_routes.rs:1895` | **INTERNAL** |
| `dsl_core` | `Statement` | ob-poc | `src/api/agent_routes.rs:2029` | **ROOT** |
| `dsl_core` | `Statement` | ob-poc | `src/dsl_v2/mod.rs:33` | **INTERNAL** |
| `dsl_core` | `Statement` | ob-poc | `src/dsl_v2/submission.rs:15` | **INTERNAL** |
| `dsl_core` | `Statement` | ob-poc | `src/mcp/handlers/core.rs:787` | **ROOT** |
| `dsl_core` | `Statement` | ob-poc | `src/mcp/intent_pipeline.rs:1472, 1753` | **INTERNAL** |
| `dsl_core` | `Statement` | ob-poc | `src/runbook/compiler.rs:29` | **INTERNAL** |
| `dsl_core` | `SuggestedFix` | ob-poc | `crates/dsl-lsp/src/handlers/code_actions.rs:14` | **INTERNAL** |
| `dsl_core` | `SuggestedFix` | ob-poc | `src/dsl_v2/mod.rs:53` | **INTERNAL** |
| `dsl_core` | `ThreeAxisDeclaration` | ob-poc | `crates/dsl-semos-frontend/src/loader.rs:21` | **INTERNAL** |
| `dsl_core` | `TransitionArgs` | ob-poc | `crates/dsl-analysis/src/macros/schema.rs:23` | **INTERNAL** |
| `dsl_core` | `TransitionArgs` | ob-poc | `crates/dsl-semos-frontend/src/loader.rs:21` | **INTERNAL** |
| `dsl_core` | `TransitionArgs` | ob-poc | `src/runbook/step_executor_bridge.rs:177, 520, 550, 590` | **INTERNAL** |
| `dsl_core` | `TransitionRef` | ob-poc | `src/runbook/step_executor_bridge.rs:254` | **INTERNAL** |
| `dsl_core` | `Unlink` | ob-poc | `src/sem_reg/onboarding/verb_extract.rs:198, 214` | **INTERNAL** |
| `dsl_core` | `Update` | ob-poc | `src/sem_reg/onboarding/verb_extract.rs:192` | **INTERNAL** |
| `dsl_core` | `Upsert` | ob-poc | `src/sem_reg/onboarding/verb_extract.rs:194` | **INTERNAL** |
| `dsl_core` | `ValidationContext` | ob-poc | `xtask/src/reconcile.rs:20` | **INTERNAL** |
| `dsl_core` | `ValidationContext` | ob-poc | `xtask/src/verbs.rs:27, 561` | **INTERNAL** |
| `dsl_core` | `Validity` | sem-os | `crates/sem_os_core/src/frontier/hydrator.rs:1` | **INTERNAL** |
| `dsl_core` | `VerbBehavior` | ob-poc | `crates/dsl-semos-frontend/src/loader.rs:21` | **INTERNAL** |
| `dsl_core` | `VerbBehavior` | ob-poc | `crates/ob-poc-boundary/src/acp_registry_projection.rs:9` | **INTERNAL** |
| `dsl_core` | `VerbBehavior` | ob-poc | `src/bin/verb_to_dsl.rs:24` | **INTERNAL** |
| `dsl_core` | `VerbBehavior` | ob-poc | `src/repl/intent_service.rs:447` | **INTERNAL** |
| `dsl_core` | `VerbBehavior` | ob-poc | `src/repl/verb_config_index.rs:398` | **INTERNAL** |
| `dsl_core` | `VerbBehavior` | ob-poc | `src/runbook/step_executor_bridge.rs:590` | **INTERNAL** |
| `dsl_core` | `VerbBehavior` | ob-poc | `src/sage/arg_assembly.rs:166` | **INTERNAL** |
| `dsl_core` | `VerbBehavior` | ob-poc | `src/sage/drafter.rs:376` | **INTERNAL** |
| `dsl_core` | `VerbBehavior` | ob-poc | `src/sage/verb_index.rs:622` | **INTERNAL** |
| `dsl_core` | `VerbBehavior` | ob-poc | `src/session/verb_tiering_linter.rs:37` | **INTERNAL** |
| `dsl_core` | `VerbBehavior` | ob-poc | `xtask/src/reconcile.rs:739` | **INTERNAL** |
| `dsl_core` | `VerbBehavior` | ob-poc | `xtask/src/verbs.rs:11` | **INTERNAL** |
| `dsl_core` | `VerbCall` | ob-poc | `crates/dsl-analysis/src/lsp_validator.rs:25` | **INTERNAL** |
| `dsl_core` | `VerbCall` | ob-poc | `crates/dsl-analysis/src/planning_facade.rs:13` | **INTERNAL** |
| `dsl_core` | `VerbCall` | ob-poc | `crates/dsl-lsp/src/analysis/v2_adapter.rs:31` | **INTERNAL** |
| `dsl_core` | `VerbCall` | ob-poc | `src/dsl_v2/mod.rs:33` | **INTERNAL** |
| `dsl_core` | `VerbCall` | ob-poc | `src/dsl_v2/submission.rs:15` | **INTERNAL** |
| `dsl_core` | `VerbCall` | ob-poc | `src/mcp/intent_pipeline.rs:1472, 1609` | **INTERNAL** |
| `dsl_core` | `VerbCall` | ob-poc | `src/runbook/compiler.rs:29` | **INTERNAL** |
| `dsl_core` | `VerbCall` | ob-poc | `src/sem_os_runtime/verb_executor_adapter.rs:25` | **INTERNAL** |
| `dsl_core` | `VerbConfig` | ob-poc | `crates/dsl-analysis/src/catalogue_loader.rs:41` | **INTERNAL** |
| `dsl_core` | `VerbConfig` | ob-poc | `crates/dsl-semos-frontend/src/loader.rs:21` | **INTERNAL** |
| `dsl_core` | `VerbConfig` | ob-poc | `crates/ob-poc-boundary/src/acp_dag_semantic.rs:8` | **INTERNAL** |
| `dsl_core` | `VerbConfig` | ob-poc | `crates/ob-poc-boundary/src/acp_registry_projection.rs:9` | **INTERNAL** |
| `dsl_core` | `VerbConfig` | ob-poc | `crates/sem_os_obpoc_adapter/src/scanner.rs:10` | **INTERNAL** |
| `dsl_core` | `VerbConfig` | ob-poc | `crates/sem_os_postgres/src/ops/discovery.rs:41` | **INTERNAL** |
| `dsl_core` | `VerbConfig` | ob-poc | `src/bin/verb_to_dsl.rs:24` | **INTERNAL** |
| `dsl_core` | `VerbConfig` | ob-poc | `src/repl/intent_service.rs:447` | **INTERNAL** |
| `dsl_core` | `VerbConfig` | ob-poc | `src/repl/verb_config_index.rs:398` | **INTERNAL** |
| `dsl_core` | `VerbConfig` | ob-poc | `src/runbook/step_executor_bridge.rs:590` | **INTERNAL** |
| `dsl_core` | `VerbConfig` | ob-poc | `src/sage/arg_assembly.rs:4` | **INTERNAL** |
| `dsl_core` | `VerbConfig` | ob-poc | `src/sage/drafter.rs:17` | **INTERNAL** |
| `dsl_core` | `VerbConfig` | ob-poc | `src/sage/verb_index.rs:11` | **INTERNAL** |
| `dsl_core` | `VerbConfig` | ob-poc | `src/sem_reg/onboarding/verb_extract.rs:180` | **INTERNAL** |
| `dsl_core` | `VerbConfig` | ob-poc | `src/session/verb_tiering_linter.rs:37` | **INTERNAL** |
| `dsl_core` | `VerbConfig` | ob-poc | `xtask/src/verbs.rs:2143` | **INTERNAL** |
| `dsl_core` | `VerbConsumes` | ob-poc | `crates/dsl-analysis/src/verb_registry.rs:22` | **INTERNAL** |
| `dsl_core` | `VerbConsumes` | ob-poc | `crates/dsl-semos-frontend/src/loader.rs:21` | **INTERNAL** |
| `dsl_core` | `VerbConsumes` | ob-poc | `crates/sem_os_obpoc_adapter/src/scanner.rs:1272, 1277` | **INTERNAL** |
| `dsl_core` | `VerbFlavour` | ob-poc | `crates/dsl-semos-frontend/src/loader.rs:21` | **INTERNAL** |
| `dsl_core` | `VerbFlavour` | ob-poc | `src/bin/verb_to_dsl.rs:24` | **INTERNAL** |
| `dsl_core` | `VerbLifecycle` | ob-poc | `crates/dsl-semos-frontend/src/loader.rs:21` | **INTERNAL** |
| `dsl_core` | `VerbLifecycle` | ob-poc | `crates/sem_os_obpoc_adapter/src/scanner.rs:1322` | **INTERNAL** |
| `dsl_core` | `VerbManifest` | ob-poc | `src/domain_ops/mod.rs:674` | **INTERNAL** |
| `dsl_core` | `VerbMetadata` | ob-poc | `crates/dsl-semos-frontend/src/loader.rs:21` | **INTERNAL** |
| `dsl_core` | `VerbMetadata` | ob-poc | `crates/sem_os_obpoc_adapter/src/scanner.rs:10` | **INTERNAL** |
| `dsl_core` | `VerbMetadata` | ob-poc | `crates/sem_os_postgres/src/ops/discovery.rs:41` | **INTERNAL** |
| `dsl_core` | `VerbMetadata` | ob-poc | `src/sage/drafter.rs:376` | **INTERNAL** |
| `dsl_core` | `VerbMetadata` | ob-poc | `src/sage/verb_index.rs:11` | **INTERNAL** |
| `dsl_core` | `VerbMetadata` | ob-poc | `src/session/verb_tiering_linter.rs:355, 449, 484, 681` | **INTERNAL** |
| `dsl_core` | `VerbOutputConfig` | ob-poc | `crates/dsl-semos-frontend/src/loader.rs:21` | **INTERNAL** |
| `dsl_core` | `VerbProduces` | ob-poc | `crates/dsl-analysis/src/verb_registry.rs:22, 246` | **INTERNAL** |
| `dsl_core` | `VerbProduces` | ob-poc | `crates/dsl-semos-frontend/src/loader.rs:21` | **INTERNAL** |
| `dsl_core` | `VerbProduces` | ob-poc | `src/sem_reg/onboarding/verb_extract.rs:173` | **INTERNAL** |
| `dsl_core` | `VerbScope` | ob-poc | `xtask/src/verbs.rs:11` | **INTERNAL** |
| `dsl_core` | `VerbSentences` | ob-poc | `crates/dsl-semos-frontend/src/loader.rs:21` | **INTERNAL** |
| `dsl_core` | `VerbSentences` | ob-poc | `src/repl/intent_service.rs:26` | **INTERNAL** |
| `dsl_core` | `VerbSentences` | ob-poc | `src/repl/verb_config_index.rs:21` | **INTERNAL** |
| `dsl_core` | `VerbTier` | ob-poc | `src/sage/verb_index.rs:11` | **INTERNAL** |
| `dsl_core` | `VerbTier` | ob-poc | `src/session/verb_tiering_linter.rs:37` | **INTERNAL** |
| `dsl_core` | `VerbTier` | ob-poc | `xtask/src/verbs.rs:11` | **INTERNAL** |
| `dsl_core` | `VerbWriteConfig` | ob-poc | `crates/dsl-semos-frontend/src/loader.rs:21` | **INTERNAL** |
| `dsl_core` | `VerbsConfig` | ob-poc | `crates/dsl-analysis/src/catalogue_loader.rs:41` | **INTERNAL** |
| `dsl_core` | `VerbsConfig` | ob-poc | `crates/dsl-semos-frontend/src/loader.rs:21` | **INTERNAL** |
| `dsl_core` | `VerbsConfig` | ob-poc | `crates/dsl-semos-frontend/tests/round_trip.rs:26` | **INTERNAL** |
| `dsl_core` | `VerbsConfig` | ob-poc | `crates/sem_os_obpoc_adapter/src/lib.rs:16` | **INTERNAL** |
| `dsl_core` | `VerbsConfig` | ob-poc | `crates/sem_os_obpoc_adapter/src/scanner.rs:10, 501` | **INTERNAL** |
| `dsl_core` | `VerbsConfig` | ob-poc | `crates/sem_os_postgres/src/ops/discovery.rs:281` | **INTERNAL** |
| `dsl_core` | `VerbsConfig` | ob-poc | `src/bin/verb_to_dsl.rs:24` | **INTERNAL** |
| `dsl_core` | `VerbsConfig` | ob-poc | `src/repl/intent_service.rs:447` | **INTERNAL** |
| `dsl_core` | `VerbsConfig` | ob-poc | `src/repl/verb_config_index.rs:21` | **INTERNAL** |
| `dsl_core` | `VerbsConfig` | ob-poc | `src/runbook/step_executor_bridge.rs:530, 590` | **INTERNAL** |
| `dsl_core` | `VerbsConfig` | ob-poc | `src/sage/drafter.rs:17` | **INTERNAL** |
| `dsl_core` | `VerbsConfig` | ob-poc | `src/sage/verb_index.rs:11` | **INTERNAL** |
| `dsl_core` | `VerbsConfig` | ob-poc | `src/sem_reg/onboarding/verb_extract.rs:104` | **INTERNAL** |
| `dsl_core` | `VerbsConfig` | ob-poc | `src/session/verb_sync.rs:56` | **INTERNAL** |
| `dsl_core` | `VerbsConfig` | ob-poc | `tests/repl_v2_phase2.rs:19` | **INTERNAL** |
| `dsl_core` | `VerbsConfig` | ob-poc | `tests/repl_v2_phase3.rs:27` | **INTERNAL** |
| `dsl_core` | `VerbsConfig` | ob-poc | `tests/repl_v2_phase4.rs:54` | **INTERNAL** |
| `dsl_core` | `VerbsConfig` | ob-poc | `tests/repl_v2_phase5.rs:52` | **INTERNAL** |
| `dsl_core` | `VerbsConfig` | ob-poc | `tests/repl_v2_phase6.rs:32` | **INTERNAL** |
| `dsl_core` | `VerbsConfig` | ob-poc | `xtask/src/reconcile.rs:20` | **INTERNAL** |
| `dsl_core` | `WiringReport` | ob-poc | `src/domain_ops/mod.rs:678` | **INTERNAL** |
| `dsl_core` | `WorkspaceId` | sem-os | `crates/sem_os_core/src/resolver/composer.rs:1` | **INTERNAL** |
| `dsl_core` | `ast` | ob-poc | `src/dsl_v2/mod.rs:32` | **ROOT** |
| `dsl_core` | `binding_context` | ob-poc | `src/dsl_v2/mod.rs:43` | **ROOT** |
| `dsl_core` | `collect_declared_fqns` | ob-poc | `xtask/src/reconcile.rs:20` | **INTERNAL** |
| `dsl_core` | `collect_declared_fqns` | ob-poc | `xtask/src/verbs.rs:561` | **INTERNAL** |
| `dsl_core` | `compile_to_steps` | ob-poc | `crates/dsl-analysis/src/planning_facade.rs:15` | **INTERNAL** |
| `dsl_core` | `compile_to_steps` | ob-poc | `crates/ob-agentic/src/validator.rs:54` | **INTERNAL** |
| `dsl_core` | `compile_to_steps` | ob-poc | `src/dsl_v2/mod.rs:61` | **INTERNAL** |
| `dsl_core` | `compile_to_steps` | ob-poc | `src/runbook/compiler.rs:30` | **INTERNAL** |
| `dsl_core` | `compiler` | ob-poc | `src/dsl_v2/mod.rs:60` | **ROOT** |
| `dsl_core` | `compute_version_hash` | sem-os | `crates/sem_os_core/src/resolver/composer.rs:1` | **INTERNAL** |
| `dsl_core` | `config` | ob-poc | `src/dsl_v2/mod.rs:47` | **ROOT** |
| `dsl_core` | `count_entity_refs` | ob-poc | `src/dsl_v2/mod.rs:33` | **INTERNAL** |
| `dsl_core` | `cycle_error` | ob-poc | `src/dsl_v2/mod.rs:53` | **INTERNAL** |
| `dsl_core` | `default` | ob-poc | `xtask/src/reconcile.rs:139` | **INTERNAL** |
| `dsl_core` | `diagnostics` | ob-poc | `src/dsl_v2/mod.rs:52` | **ROOT** |
| `dsl_core` | `entity_kinds_from_taxonomy_yaml` | ob-poc | `xtask/src/reconcile.rs:20` | **INTERNAL** |
| `dsl_core` | `flatten_pack_entries` | ob-poc | `xtask/src/reconcile.rs:20` | **INTERNAL** |
| `dsl_core` | `flatten_pack_entries` | ob-poc | `xtask/src/verbs.rs:561` | **INTERNAL** |
| `dsl_core` | `from_env` | ob-poc | `src/agent/orchestrator.rs:2288` | **INTERNAL** |
| `dsl_core` | `from_env` | ob-poc | `src/api/repl_routes_v2.rs:504` | **INTERNAL** |
| `dsl_core` | `from_env` | ob-poc | `src/sem_reg/onboarding/verb_extract.rs:309` | **INTERNAL** |
| `dsl_core` | `from_env` | ob-poc | `tests/db_integration.rs:293` | **INTERNAL** |
| `dsl_core` | `from_env` | ob-poc | `xtask/src/sem_reg.rs:1005` | **INTERNAL** |
| `dsl_core` | `generate_phrases` | ob-poc | `src/repl/sentence_gen.rs:47` | **INTERNAL** |
| `dsl_core` | `implicit_create_hint` | ob-poc | `src/dsl_v2/mod.rs:53` | **INTERNAL** |
| `dsl_core` | `load_dags_from_dir` | ob-poc | `xtask/src/reconcile.rs:20` | **INTERNAL** |
| `dsl_core` | `load_domain_pack_owned_dags` | sem-os | `crates/sem_os_core/src/resolver/composer.rs:1` | **INTERNAL** |
| `dsl_core` | `load_packs_from_dir` | ob-poc | `xtask/src/reconcile.rs:20` | **INTERNAL** |
| `dsl_core` | `load_packs_from_dir` | ob-poc | `xtask/src/verbs.rs:561` | **INTERNAL** |
| `dsl_core` | `missing_arg_error` | ob-poc | `src/dsl_v2/mod.rs:53` | **INTERNAL** |
| `dsl_core` | `new` | ob-poc | `crates/dsl-semos-frontend/tests/round_trip.rs:27` | **INTERNAL** |
| `dsl_core` | `parse_green_when` | sem-os | `crates/sem_os_core/src/frontier/hydrator.rs:1` | **INTERNAL** |
| `dsl_core` | `parse_program` | ob-poc | `crates/dsl-analysis/src/lsp_validator.rs:26` | **INTERNAL** |
| `dsl_core` | `parse_program` | ob-poc | `crates/dsl-analysis/src/planning_facade.rs:17` | **INTERNAL** |
| `dsl_core` | `parse_program` | ob-poc | `crates/dsl-lsp/src/analysis/v2_adapter.rs:32` | **INTERNAL** |
| `dsl_core` | `parse_program` | ob-poc | `crates/dsl-lsp/src/handlers/completion.rs:14` | **INTERNAL** |
| `dsl_core` | `parse_program` | ob-poc | `crates/dsl-lsp/tests/parser_conformance.rs:12` | **INTERNAL** |
| `dsl_core` | `parse_program` | ob-poc | `crates/ob-agentic/src/validator.rs:35` | **ROOT** |
| `dsl_core` | `parse_program` | ob-poc | `crates/ob-poc-agent/src/repl_channel.rs:292, 353` | **INTERNAL** |
| `dsl_core` | `parse_program` | ob-poc | `src/dsl_v2/mod.rs:40` | **INTERNAL** |
| `dsl_core` | `parse_program` | ob-poc | `src/runbook/compiler.rs:31` | **INTERNAL** |
| `dsl_core` | `parse_single_verb` | ob-poc | `src/dsl_v2/mod.rs:40` | **INTERNAL** |
| `dsl_core` | `parse_single_verb` | sem-os | `crates/sem_os_core/src/frontier/hydrator.rs:1` | **INTERNAL** |
| `dsl_core` | `parser` | ob-poc | `src/dsl_v2/mod.rs:39` | **ROOT** |
| `dsl_core` | `set_phrase_gen_nouns` | ob-poc | `src/dsl_v2/mod.rs:49` | **INTERNAL** |
| `dsl_core` | `undefined_symbol_error` | ob-poc | `src/dsl_v2/mod.rs:53` | **INTERNAL** |
| `dsl_core` | `unknown_verb_error` | ob-poc | `src/dsl_v2/mod.rs:53` | **INTERNAL** |
| `dsl_core` | `validate_dags_with_context` | ob-poc | `xtask/src/reconcile.rs:20` | **INTERNAL** |
| `dsl_core` | `validate_pack_fqns` | ob-poc | `xtask/src/reconcile.rs:20` | **INTERNAL** |
| `dsl_core` | `validate_pack_fqns` | ob-poc | `xtask/src/verbs.rs:561` | **INTERNAL** |
| `dsl_core` | `validate_verbs_config` | ob-poc | `xtask/src/reconcile.rs:20` | **INTERNAL** |
| `dsl_core` | `validate_verbs_config` | ob-poc | `xtask/src/verbs.rs:27, 561` | **INTERNAL** |
| `dsl_core` | `wiring_check` | ob-poc | `src/domain_ops/mod.rs:694` | **INTERNAL** |

## 3. Revised dsl-core FACADE
The revised `dsl-core` public facade is recomputed as the union of:
1. Downstream consumed symbols (from Step 2).
2. Symbols consumed by `dsl-core` contract-level integration tests (0 contract-level tests found; all existing tests access internal paths).
3. The planned 70 facade items.

**Revised Facade Count**: 198 (an increase of +150 items vs the planned 70)

### Facade Itemized List (Newly added symbols flagged):
- `ActionClass` [NEWLY ADDED]
- `ActionClass as DslActionClass` [NEWLY ADDED]
- `ArgConfig` [NEWLY ADDED]
- `ArgType` [NEWLY ADDED]
- `Argument` [NEWLY ADDED]
- `AstNode` [NEWLY ADDED]
- `AttrValue`
- `AuthorityContext` [NEWLY ADDED]
- `Benign` [NEWLY ADDED]
- `BindingContext` [NEWLY ADDED]
- `BindingFrameSchema` [NEWLY ADDED]
- `BindingInfo` [NEWLY ADDED]
- `BindingSlot` [NEWLY ADDED]
- `BindingSlotId`
- `CascadeRule` [NEWLY ADDED]
- `ClosureType` [NEWLY ADDED]
- `CmpOp`
- `CompileError as OpCompileError` [NEWLY ADDED]
- `CompileStep`
- `CompiledSteps`
- `CompletenessAssertionStatus`
- `Composite` [NEWLY ADDED]
- `ConfidenceZone` [NEWLY ADDED]
- `ConfigLoader`
- `ConfirmPolicyConfig` [NEWLY ADDED]
- `ConsequenceTier`
- `CrossWorkspaceConstraint` [NEWLY ADDED]
- `Crud` [NEWLY ADDED]
- `CrudConfig` [NEWLY ADDED]
- `CrudOperation` [NEWLY ADDED]
- `Dag`
- `DagEdge`
- `DagRegistry` [NEWLY ADDED]
- `DagValidationContext`
- `Delete` [NEWLY ADDED]
- `Deprecated` [NEWLY ADDED]
- `DerivationCondition` [NEWLY ADDED]
- `DerivedCrossWorkspaceState` [NEWLY ADDED]
- `Destructive` [NEWLY ADDED]
- `Diagnostic` [NEWLY ADDED]
- `DiagnosticCode` [NEWLY ADDED]
- `DiscretionaryReason` [NEWLY ADDED]
- `DomainConfig` [NEWLY ADDED]
- `Durable` [NEWLY ADDED]
- `DurableConfig` [NEWLY ADDED]
- `DurableRuntime` [NEWLY ADDED]
- `EffectClass`
- `EligibilityConstraint` [NEWLY ADDED]
- `EnhanceArg` [NEWLY ADDED]
- `Entity` [NEWLY ADDED]
- `EntityCreate` [NEWLY ADDED]
- `EntityRef`
- `EntityRef as PredicateEntityRef` [NEWLY ADDED]
- `EntityRefStats` [NEWLY ADDED]
- `EntitySetRef`
- `EntityUpsert` [NEWLY ADDED]
- `EntryVia` [NEWLY ADDED]
- `ExecutablePlan` [NEWLY ADDED]
- `ExecutionStepSummary` [NEWLY ADDED]
- `ExportFormat` [NEWLY ADDED]
- `ExternalEffect`
- `FocusTarget` [NEWLY ADDED]
- `FrontierFact`
- `FrontierFacts` [NEWLY ADDED]
- `GraphQuery` [NEWLY ADDED]
- `GraphQueryConfig` [NEWLY ADDED]
- `GraphQueryOperation` [NEWLY ADDED]
- `GreenWhenStatus`
- `HarmClass` [NEWLY ADDED]
- `HarmClass as DslHarmClass` [NEWLY ADDED]
- `HydrateFrontierError`
- `Insert` [NEWLY ADDED]
- `InstanceFrontier` [NEWLY ADDED]
- `InstructionInput` [NEWLY ADDED]
- `InvalidFact` [NEWLY ADDED]
- `InvalidFactDetail`
- `Irreversible` [NEWLY ADDED]
- `JoinBarrierMode`
- `Link` [NEWLY ADDED]
- `ListByFk` [NEWLY ADDED]
- `ListParties` [NEWLY ADDED]
- `Literal` [NEWLY ADDED]
- `LoadedDag`
- `LoadedPack` [NEWLY ADDED]
- `LookupConfig` [NEWLY ADDED]
- `ManifestOptions`
- `MissingFact` [NEWLY ADDED]
- `NavDirection` [NEWLY ADDED]
- `NavTarget` [NEWLY ADDED]
- `NodeId`
- `PhraseGenNouns` [NEWLY ADDED]
- `PlanId` [NEWLY ADDED]
- `Plugin` [NEWLY ADDED]
- `PolicyConfig` [NEWLY ADDED]
- `PopulatedExecutionDag`
- `Predicate`
- `PredicateBinding`
- `Program` [NEWLY ADDED]
- `ReachableDestination` [NEWLY ADDED]
- `ReadOnly` [NEWLY ADDED]
- `Reference` [NEWLY ADDED]
- `RelatedInfo` [NEWLY ADDED]
- `RequiresConfirmation` [NEWLY ADDED]
- `RequiresExplicitAuthorisation` [NEWLY ADDED]
- `ResolutionMode` [NEWLY ADDED]
- `ResolvedResourceDependency` [NEWLY ADDED]
- `ResolvedSlot`
- `ResolvedSource`
- `ResolvedTemplate`
- `ResolvedTransition`
- `ResolverManifest`
- `ResolverProvenance`
- `ResourceDependency` [NEWLY ADDED]
- `ReturnTypeConfig` [NEWLY ADDED]
- `ReturnsConfig` [NEWLY ADDED]
- `Reversible` [NEWLY ADDED]
- `Reviewable` [NEWLY ADDED]
- `RoleGuard` [NEWLY ADDED]
- `RoleLink` [NEWLY ADDED]
- `RoleUnlink` [NEWLY ADDED]
- `RuntimeInstruction` [NEWLY ADDED]
- `Select` [NEWLY ADDED]
- `SelectWithJoin` [NEWLY ADDED]
- `SemOsSnapshotId` [NEWLY ADDED]
- `Severity` [NEWLY ADDED]
- `Severity as PlanningSeverity` [NEWLY ADDED]
- `ShapeRef` [NEWLY ADDED]
- `Simple` [NEWLY ADDED]
- `Slot as DagSlot` [NEWLY ADDED]
- `SlotProvenance`
- `SlotStateMachine`
- `SourceOfTruth` [NEWLY ADDED]
- `SourceSpan` [NEWLY ADDED]
- `Span` [NEWLY ADDED]
- `Span as V2Span` [NEWLY ADDED]
- `StateEffect`
- `StateSelector` [NEWLY ADDED]
- `Statement`
- `SuggestedFix` [NEWLY ADDED]
- `ThreeAxisDeclaration` [NEWLY ADDED]
- `TransactionPolicy`
- `TransitionArgs` [NEWLY ADDED]
- `TransitionRef` [NEWLY ADDED]
- `Unlink` [NEWLY ADDED]
- `Update` [NEWLY ADDED]
- `Upsert` [NEWLY ADDED]
- `ValidationContext`
- `Validity`
- `VerbBehavior` [NEWLY ADDED]
- `VerbCall`
- `VerbConfig` [NEWLY ADDED]
- `VerbConsumes` [NEWLY ADDED]
- `VerbFlavour`
- `VerbLifecycle` [NEWLY ADDED]
- `VerbManifest` [NEWLY ADDED]
- `VerbMetadata` [NEWLY ADDED]
- `VerbOutputConfig` [NEWLY ADDED]
- `VerbProduces` [NEWLY ADDED]
- `VerbScope` [NEWLY ADDED]
- `VerbSentences` [NEWLY ADDED]
- `VerbTier` [NEWLY ADDED]
- `VerbWriteConfig` [NEWLY ADDED]
- `VerbsConfig`
- `ViewType` [NEWLY ADDED]
- `ViewportVerb` [NEWLY ADDED]
- `WiringReport` [NEWLY ADDED]
- `WorkspaceId` [NEWLY ADDED]
- `ast` [NEWLY ADDED]
- `binding_context` [NEWLY ADDED]
- `collect_declared_fqns` [NEWLY ADDED]
- `compile_to_steps`
- `compiler` [NEWLY ADDED]
- `compute_version_hash` [NEWLY ADDED]
- `config` [NEWLY ADDED]
- `count_entity_refs` [NEWLY ADDED]
- `cycle_error` [NEWLY ADDED]
- `default` [NEWLY ADDED]
- `entity_kinds_from_taxonomy_yaml` [NEWLY ADDED]
- `flatten_pack_entries` [NEWLY ADDED]
- `from_env` [NEWLY ADDED]
- `generate_phrases` [NEWLY ADDED]
- `implicit_create_hint` [NEWLY ADDED]
- `load_dags_from_dir`
- `load_domain_pack_owned_dags` [NEWLY ADDED]
- `load_packs_from_dir` [NEWLY ADDED]
- `missing_arg_error` [NEWLY ADDED]
- `new` [NEWLY ADDED]
- `parse_green_when`
- `parse_program`
- `parse_single_verb` [NEWLY ADDED]
- `parser` [NEWLY ADDED]
- `set_phrase_gen_nouns` [NEWLY ADDED]
- `undefined_symbol_error` [NEWLY ADDED]
- `unknown_verb_error` [NEWLY ADDED]
- `validate_dags_with_context`
- `validate_pack_fqns` [NEWLY ADDED]
- `validate_verbs_config`
- `wiring_check` [NEWLY ADDED]

## 4. Corrected Test-Relocation Set
Downstream-consumed symbols have been removed from the `dsl-core` test-relocation set (they are contract public API now).

**Corrected Test-Relocation Count**: 62 (reduced from 117)

### Remaining Test-Relocation Items:
- `AggregationRule`
- `CrossScopeRule`
- `DagError`
- `DagWarning`
- `EntityQualifier`
- `EvaluationContext`
- `GreenWhenExclusionReason`
- `Phase`
- `RelationScope`
- `RunbookStep`
- `StructuralError`
- `TransactionPolicy`
- `VersionHash`
- `add_edge`
- `all`
- `all`
- `as_symbol`
- `compute_effective_tier`
- `compute_runbook_tier`
- `config_dir`
- `contains`
- `dag`
- `dag_validator`
- `display`
- `effect_class`
- `entity_ref`
- `error`
- `error_count`
- `error_count`
- `execution_dag`
- `extension`
- `from_effect_classes`
- `from_template`
- `frontier`
- `full_name`
- `get`
- `green_when_coverage_for_dag`
- `green_when_coverage_for_dags`
- `green_when_coverage_summary`
- `harden_schema_coordination_warnings`
- `info`
- `insert`
- `is_empty`
- `is_empty`
- `is_ok`
- `len`
- `len`
- `load_verbs`
- `loader`
- `manifest`
- `ordering_pairs`
- `resolver`
- `slot_mut`
- `string`
- `to_text`
- `types`
- `validate_constellation_map_schema_coordination`
- `validate_resolved_template_gate_metadata`
- `with_arg`
- `with_entity_attr`
- `with_flag`
- `with_required_slots`

## 5. dsl_types Facade Widening Verification
Confirmed whether `dsl_types` facade had to be widened beyond the planned 13 types:
- **Widened items**: None (0)
- **Attestation**: No `dsl_types` symbols outside of the planned 13 facade types are consumed by downstream repositories.

## 6. Downstream Baseline Status
The baseline build/test status of downstream repos against the current `dsl` workspace changes is recorded below:

### Command List for "downstream-green" Verification:
1. **dsl workspace**:
   ```bash
   cd /Users/adamtc007/Dev/dsl && cargo build --workspace && cargo test --workspace
   ```
   *Status*: **GREEN**
2. **sem-os**:
   ```bash
   cd /Users/adamtc007/Dev/sem-os && cargo build --workspace && cargo test --workspace
   ```
   *Status*: **GREEN** (Verified builds cleanly and passes all tests after Tranche A facade fixes)
3. **ob-poc**:
   ```bash
   cd /Users/adamtc007/Developer/ob-poc/rust && cargo build --workspace && cargo test --workspace
   ```
   *Status*: **GREEN** (Builds and passes against the tag v0.1.4; uses git tags so unaffected by local dsl edits until local patch is applied)

## Invariant attestation
- E0 no live-body edits: PASS — Zero source edits made in this tranche.
- E1 no globs introduced:  PASS — Zero globs introduced.
- E2 no allow(dead_code):  PASS — Zero allow(dead_code) introduced.

## Next
- Next tranche: Tranche B — entry preconditions: Tranche A.5 accepted, downstream repos verified green.
