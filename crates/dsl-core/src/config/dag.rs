//! DAG taxonomy YAML loader + typed structs (R-2a, v1.3 foundation).
//!
//! Loads `rust/config/sem_os_seeds/dag_taxonomies/*.yaml` into typed Rust
//! structs. Covers the full v1.2 DAG schema (`overall_lifecycle:`,
//! `slots:`, `cross_slot_constraints:`, `product_module_gates:`,
//! `prune_cascade_rules:`, `prune_pre_validation:`) AND the v1.3
//! extensions (cross_workspace_constraints, derived_cross_workspace_state,
//! parent_slot/state_dependency, expected_lifetime, dual_lifecycle,
//! periodic_review_cadence, evidence_types, category_gated).

use anyhow::{Context, Result};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

// Re-export all relocated types from dsl_types::dag module
// Re-exports reached by external crates (facade/config re-exports)
pub use dsl_types::{
    ApprovalGateRef, CascadeRule, CrossWorkspaceConstraint, Dag, DerivationCondition,
    DerivedCrossWorkspaceState, EntryVia, FoldPreconditionsRef, LexiconManifestRef, LoadedDag,
    Phase, PredicateBinding, Severity, Slot, SlotStateMachine, StateSelector, StreamGoverned,
    StreamGovernance,
};

// Re-exports reached only inside dsl-core
pub(crate) use dsl_types::{ExpectedLifetime, PredicateBindingSourceKind, StateMachine};

// Re-exports of level-0 types reached only inside dsl-core
pub(crate) use dsl_types::{ClosureType, EligibilityConstraint};

/// Load every `*.yaml` file in the DAG taxonomies directory. Returns a
/// map keyed by `workspace` name. Malformed files surface an error —
/// we're stricter than pack_loader because DAG YAML is authoritative
/// architectural input.
pub fn load_dags_from_dir(dags_dir: &Path) -> Result<BTreeMap<String, LoadedDag>> {
    let mut out: BTreeMap<String, LoadedDag> = BTreeMap::new();
    let entries = fs::read_dir(dags_dir)
        .with_context(|| format!("cannot read DAG taxonomies dir {dags_dir:?}"))?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("yaml") {
            continue;
        }
        let raw = fs::read_to_string(&path)
            .with_context(|| format!("cannot read DAG taxonomy {path:?}"))?;
        let dag: Dag = serde_yaml::from_str(&raw)
            .with_context(|| format!("failed to parse DAG taxonomy {path:?}"))?;
        out.insert(
            dag.workspace.clone(),
            LoadedDag {
                source_path: path,
                dag,
            },
        );
    }
    Ok(out)
}

/// Load DAG taxonomies through Sem OS Domain Pack ownership.
///
/// Runtime callers should prefer this over walking `dag_taxonomies/`
/// directly. A DAG is visible to the compiler/runtime only when a domain-pack
/// manifest declares it in `owned_dags`.
pub fn load_domain_pack_owned_dags(config_root: &Path) -> Result<BTreeMap<String, LoadedDag>> {
    let domain_pack_dir = config_root.join("sem_os_seeds/domain_packs");
    let dag_dir = config_root.join("sem_os_seeds/dag_taxonomies");
    let mut out: BTreeMap<String, LoadedDag> = BTreeMap::new();

    for manifest_path in yaml_files(&domain_pack_dir)? {
        let raw = fs::read_to_string(&manifest_path)
            .with_context(|| format!("cannot read domain pack manifest {manifest_path:?}"))?;
        let manifest: serde_yaml::Value = serde_yaml::from_str(&raw)
            .with_context(|| format!("failed to parse domain pack manifest {manifest_path:?}"))?;
        let pack_id = manifest
            .get("pack_id")
            .and_then(serde_yaml::Value::as_str)
            .unwrap_or("<unknown>");

        let owned_dags = manifest
            .get("owned_dags")
            .and_then(serde_yaml::Value::as_sequence)
            .into_iter()
            .flatten()
            .filter_map(serde_yaml::Value::as_str);

        for dag_id in owned_dags {
            let path = find_dag_yaml_by_id(&dag_dir, dag_id)
                .with_context(|| format!("domain pack {pack_id} declares missing DAG {dag_id}"))?;
            let raw = fs::read_to_string(&path)
                .with_context(|| format!("cannot read DAG taxonomy {path:?}"))?;
            let dag: Dag = serde_yaml::from_str(&raw)
                .with_context(|| format!("failed to parse DAG taxonomy {path:?}"))?;
            let loaded = LoadedDag {
                source_path: path,
                dag,
            };
            if let Some(existing) = out.get(&loaded.dag.workspace) {
                if existing.dag.dag_id != loaded.dag.dag_id {
                    anyhow::bail!(
                        "domain pack DAG ownership conflict for workspace {}: {} vs {}",
                        loaded.dag.workspace,
                        existing.dag.dag_id,
                        loaded.dag.dag_id
                    );
                }
            } else {
                out.insert(loaded.dag.workspace.clone(), loaded);
            }
        }
    }

    Ok(out)
}

fn yaml_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir).with_context(|| format!("cannot read YAML dir {dir:?}"))? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("yaml") {
            files.push(path);
        }
    }
    files.sort();
    Ok(files)
}

fn find_dag_yaml_by_id(dag_dir: &Path, expected: &str) -> Result<PathBuf> {
    for path in yaml_files(dag_dir)? {
        let raw = fs::read_to_string(&path)
            .with_context(|| format!("cannot read DAG taxonomy {path:?}"))?;
        let yaml: serde_yaml::Value = serde_yaml::from_str(&raw)
            .with_context(|| format!("failed to parse DAG taxonomy {path:?}"))?;
        let matches = ["dag_id", "workspace"]
            .iter()
            .any(|field| yaml.get(*field).and_then(serde_yaml::Value::as_str) == Some(expected));
        if matches {
            return Ok(path);
        }
    }

    anyhow::bail!("failed to find DAG taxonomy {expected} in {dag_dir:?}")
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimal_dag_parses() {
        let yaml = r#"
version: "1.0"
workspace: example
dag_id: example_dag
"#;
        let dag: Dag = serde_yaml::from_str(yaml).expect("parse");
        assert_eq!(dag.workspace, "example");
        assert_eq!(dag.dag_id, "example_dag");
    }

    #[test]
    fn predicate_bindings_parse_on_state_machine() {
        let yaml = r#"
version: "1.0"
workspace: example
dag_id: example_dag
slots:
  - id: clearance
    state_machine:
      id: clearance_lifecycle
      source_entity: '"ob-poc".booking_principal_clearances'
      state_column: clearance_status
      predicate_bindings:
        - entity: screening_check
          source_kind: substrate
          source_entity: '"ob-poc".screenings'
          state_column: status
          id_column: screening_id
          scope: attached_to this clearance
          parent_key: clearance_id
          child_key: id
        - entity: evidence_requirement
          source_kind: substrate
          source_entity: '"ob-poc".ubo_evidence'
          state_column: verification_status
          required_universe:
            source_kind: substrate
            source_entity: '"sem_reg".evidence_requirements'
            id_column: requirement_id
            required_column: evidence_kind
            parent_key: case_id
            child_key: case_id
      states:
        - id: PENDING
"#;
        let dag: Dag = serde_yaml::from_str(yaml).expect("parse");
        let SlotStateMachine::Structured(machine) =
            dag.slots[0].state_machine.as_ref().expect("state machine")
        else {
            panic!("expected structured state machine");
        };

        assert_eq!(machine.predicate_bindings.len(), 2);
        assert_eq!(machine.predicate_bindings[0].entity, "screening_check");
        assert_eq!(
            machine.predicate_bindings[0].source_kind,
            PredicateBindingSourceKind::Substrate
        );
        assert_eq!(
            machine.predicate_bindings[0].source_entity.as_deref(),
            Some("\"ob-poc\".screenings")
        );
        assert_eq!(
            machine.predicate_bindings[1]
                .required_universe
                .as_ref()
                .map(|binding| binding.source_entity.as_str()),
            Some("\"sem_reg\".evidence_requirements")
        );
    }

    #[test]
    fn cross_workspace_constraint_parses() {
        let yaml = r#"
workspace: deal
dag_id: deal_dag
cross_workspace_constraints:
  - id: deal_contracted_requires_kyc_approved
    description: "Deal needs KYC"
    source_workspace: kyc
    source_slot: kyc_case
    source_state: APPROVED
    target_workspace: deal
    target_slot: deal
    target_transition: "KYC_CLEARANCE -> CONTRACTED"
    severity: error
"#;
        let dag: Dag = serde_yaml::from_str(yaml).expect("parse");
        assert_eq!(dag.cross_workspace_constraints.len(), 1);
        let c = &dag.cross_workspace_constraints[0];
        assert_eq!(c.source_workspace, "kyc");
        assert_eq!(c.target_workspace, "deal");
        assert_eq!(c.severity, Severity::Error);
    }

    #[test]
    fn derived_cross_workspace_state_with_tollgate_parses() {
        let yaml = r#"
workspace: cbu
dag_id: cbu_dag
derived_cross_workspace_state:
  - id: cbu_operationally_active
    description: "Tollgate"
    host_workspace: cbu
    host_slot: cbu
    host_state: operationally_active
    derivation:
      all_of:
        - { workspace: kyc, slot: kyc_case, state: APPROVED }
        - { workspace: deal, slot: deal, state: [CONTRACTED, ONBOARDING, ACTIVE] }
        - { workspace: cbu, slot: cbu_evidence, predicate: "all verified" }
    exposure:
      visible_as: first_class_state
      cacheable: true
"#;
        let dag: Dag = serde_yaml::from_str(yaml).expect("parse");
        assert_eq!(dag.derived_cross_workspace_state.len(), 1);
        let d = &dag.derived_cross_workspace_state[0];
        assert_eq!(d.host_state, "operationally_active");
        assert_eq!(d.derivation.all_of.len(), 3);
    }

    #[test]
    fn slot_with_parent_and_state_dependency() {
        let yaml = r#"
workspace: cbu
dag_id: cbu_dag
slots:
  - id: cbu
    stateless: false
    parent_slot:
      workspace: cbu
      slot: cbu
      join:
        via: cbu_entity_relationships
        parent_fk: parent_cbu_id
        child_fk: child_cbu_id
    state_dependency:
      cascade_rules:
        - parent_state: SUSPENDED
          child_allowed_states: [SUSPENDED]
          cascade_on_parent_transition: true
          default_child_state_on_cascade: SUSPENDED
      severity: error
"#;
        let dag: Dag = serde_yaml::from_str(yaml).expect("parse");
        let slot = &dag.slots[0];
        assert!(slot.parent_slot.is_some());
        let dep = slot.state_dependency.as_ref().unwrap();
        assert_eq!(dep.cascade_rules.len(), 1);
        assert_eq!(dep.cascade_rules[0].parent_state, "SUSPENDED");
    }

    #[test]
    fn slot_with_category_gated() {
        let yaml = r#"
workspace: cbu
dag_id: cbu_dag
slots:
  - id: investor
    stateless: false
    category_gated:
      category_column: cbu_category
      category_source: cbus
      activated_by: [FUND_MANDATE]
"#;
        let dag: Dag = serde_yaml::from_str(yaml).expect("parse");
        let gate = dag.slots[0].category_gated.as_ref().unwrap();
        assert_eq!(gate.activated_by, vec!["FUND_MANDATE"]);
    }

    #[test]
    fn dual_lifecycle_parses() {
        let yaml = r#"
workspace: deal
dag_id: deal_dag
slots:
  - id: deal
    stateless: false
    state_machine:
      id: deal_commercial_lifecycle
      owner: "sales+BAC"
      expected_lifetime: long_lived
      states:
        - id: PROSPECT
          entry: true
        - id: CONTRACTED
      transitions:
        - from: PROSPECT
          to: CONTRACTED
    dual_lifecycle:
      - id: deal_operational_lifecycle
        owner: ops
        junction_state_from_primary: CONTRACTED
        states:
          - id: ONBOARDING
            entry: true
          - id: OFFBOARDED
        terminal_states: [OFFBOARDED]
"#;
        let dag: Dag = serde_yaml::from_str(yaml).expect("parse");
        let slot = &dag.slots[0];
        let dual = &slot.dual_lifecycle;
        assert_eq!(dual.len(), 1);
        assert_eq!(dual[0].junction_state_from_primary, "CONTRACTED");
        assert_eq!(dual[0].owner.as_deref(), Some("ops"));
    }

    #[test]
    fn state_entry_via_parses() {
        let yaml = r#"
workspace: deal
dag_id: deal_dag
slots:
  - id: deal_rate_card
    stateless: false
    state_machine:
      id: deal_rate_card_lifecycle
      states:
        - id: DRAFT
          entry: true
          entry_via: verb
        - id: SUPERSEDED
          entry_via:
            trigger:
              name: idx_deal_rate_cards_one_agreed
        - id: CANCELLED
          entry_via:
            cascade:
              parent: deal.cancel
"#;
        let dag: Dag = serde_yaml::from_str(yaml).expect("parse");
        let Some(SlotStateMachine::Structured(sm)) = &dag.slots[0].state_machine else {
            panic!("expected structured state machine");
        };
        assert_eq!(sm.states[0].entry_via, Some(EntryVia::Verb));
        assert_eq!(
            sm.states[1].entry_via,
            Some(EntryVia::Trigger {
                name: "idx_deal_rate_cards_one_agreed".to_string()
            })
        );
        assert_eq!(
            sm.states[2].entry_via,
            Some(EntryVia::Cascade {
                parent: "deal.cancel".to_string()
            })
        );
    }

    #[test]
    fn periodic_review_cadence_parses() {
        let yaml = r#"
workspace: kyc
dag_id: kyc_dag
slots:
  - id: kyc_case
    stateless: false
    periodic_review_cadence:
      base_window: "P2Y"
      risk_tiered_overrides:
        - risk_tier: HIGH
          window: "P1Y"
      review_scope: full
evidence_types:
  - id: sanctions_screening
    validity_window: "P14D"
  - id: corporate_formation_docs
    validity_window: once
"#;
        let dag: Dag = serde_yaml::from_str(yaml).expect("parse");
        let cadence = dag.slots[0].periodic_review_cadence.as_ref().unwrap();
        assert_eq!(cadence.base_window, "P2Y");
        assert_eq!(cadence.risk_tiered_overrides[0].window, "P1Y");
        assert_eq!(dag.evidence_types.len(), 2);
    }

    #[test]
    fn suspended_exempt_parses() {
        let yaml = r#"
workspace: kyc
dag_id: kyc_dag
slots:
  - id: kyc_case
    stateless: false
    suspended_state_exempt: true
    state_machine:
      id: kyc_case_lifecycle
      expected_lifetime: long_lived
      states:
        - id: INTAKE
          entry: true
        - id: APPROVED
"#;
        let dag: Dag = serde_yaml::from_str(yaml).expect("parse");
        assert!(dag.slots[0].suspended_state_exempt);
        let sm = match dag.slots[0].state_machine.as_ref().unwrap() {
            SlotStateMachine::Structured(sm) => sm,
            _ => panic!("expected structured"),
        };
        assert_eq!(sm.expected_lifetime, Some(ExpectedLifetime::LongLived));
    }

    #[test]
    fn state_machine_reference_form_parses() {
        // "reconcile-existing" form uses a string reference
        let yaml = r#"
workspace: cbu
dag_id: cbu_dag
slots:
  - id: client_group
    stateless: false
    state_machine: "(reconcile-existing — see instrument_matrix_dag.yaml)"
"#;
        let dag: Dag = serde_yaml::from_str(yaml).expect("parse");
        let sm = dag.slots[0].state_machine.as_ref().unwrap();
        assert!(matches!(sm, SlotStateMachine::Reference(_)));
    }
}

#[cfg(test)]
mod integration_tests;
