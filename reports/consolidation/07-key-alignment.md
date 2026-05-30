# Phase 2 — Step 4 Prep (cont.): All-Packs Key Alignment Report

This report documents the slot and workspace key alignment checks across all 12 domain packs in the workspace.

---

## C1: Per-Pack Key Mappings

Below is the slot key comparison for each of the 12 domain packs:

### 1. `ob-poc.book-setup`
* **DAG Files**: [book_setup_dag.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/dag_taxonomies/book_setup_dag.yaml)
* **Constellation Files**: [cbu_workspace.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/cbu_workspace.yaml), [onboarding_workspace.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/onboarding_workspace.yaml)
* **Exact-Match Keys**: `workspace_root`
* **DAG-Only Keys**: `book`, `book_participant`, `cbu`, `client_group`, `entity`, `kyc_case`, `mandate_outline`, `structure_template`, `trading_profile`
* **Constellation-Only Keys (Rich, No Topology)**: 
  * `cbu_resource_instance`
  * `cbu_resource_instance_option_lineage`
  * `cbu_service_option_binding`
  * `onboarding_data_request`
  * `onboarding_data_request_attr`
  * `onboarding_data_request_discovery`
  * `onboarding_data_request_slice`
  * `provisioning_request`
  * `resource_owner_principal`
* **Candidate Name Mismatches**:
  * `cbu_resource_instance` <-> `cbu`
  * `cbu_resource_instance_option_lineage` <-> `cbu`
  * `cbu_service_option_binding` <-> `cbu`

### 2. `ob-poc.booking-principal`
* **DAG Files**: [booking_principal_dag.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/dag_taxonomies/booking_principal_dag.yaml)
* **Constellation Files**: [deal_workspace.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/deal_workspace.yaml)
* **Exact-Match Keys**: `workspace_root`
* **DAG-Only Keys**: `clearance`
* **Constellation-Only Keys (Rich, No Topology)**: None
* **Candidate Name Mismatches**: None

### 3. `ob-poc.catalogue`
* **DAG Files**: [catalogue_dag.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/dag_taxonomies/catalogue_dag.yaml)
* **Constellation Files**: [registry_stewardship.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/registry_stewardship.yaml)
* **Exact-Match Keys**: None
* **DAG-Only Keys**: `proposal`, `verb`, `workspace_root`
* **Constellation-Only Keys (Rich, No Topology)**: 
  * `attribute_def`
  * `changeset`
  * `derivation_spec`
  * `governance`
  * `phrase_authoring`
  * `registry`
  * `service_resource_def`
  * `service_resource_def_governance`
  * `typed_attribute`
* **Candidate Name Mismatches**: None

### 4. `ob-poc.cbu`
* **DAG Files**: [cbu_dag.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/dag_taxonomies/cbu_dag.yaml)
* **Constellation Files**: [cbu_workspace.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/cbu_workspace.yaml)
* **Exact-Match Keys**: `cbu_resource_instance_option_lineage`, `cbu_service_option_binding`, `workspace_root`
* **DAG-Only Keys**: `cbu`, `cbu_corporate_action`, `cbu_discovery_state`, `cbu_disposition`, `cbu_entity_relationship`, `cbu_entity_role`, `cbu_evidence`, `client_group`, `client_group_entity_review`, `edge`, `entity_limited_company_ubo`, `entity_proper_person`, `holding`, `investor`, `investor_kyc`, `investor_role`, `kyc_case`, `legal_entity`, `manco`, `service_consumption`, `share_class`, `temporal`, `trading_profile`
* **Constellation-Only Keys (Rich, No Topology)**: 
  * `cbu_resource_instance`
  * `onboarding_data_request`
  * `onboarding_data_request_slice`
  * `provisioning_request`
* **Candidate Name Mismatches**:
  * `cbu_resource_instance` <-> `cbu`
  * `cbu_resource_instance` <-> `cbu_corporate_action`
  * `cbu_resource_instance` <-> `cbu_discovery_state`
  * `cbu_resource_instance` <-> `cbu_disposition`
  * `cbu_resource_instance` <-> `cbu_entity_relationship`
  * `cbu_resource_instance` <-> `cbu_entity_role`
  * `cbu_resource_instance` <-> `cbu_evidence`

### 5. `ob-poc.deal`
* **DAG Files**: [deal_dag.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/dag_taxonomies/deal_dag.yaml)
* **Constellation Files**: [deal_lifecycle.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/deal_lifecycle.yaml), [deal_workspace.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/deal_workspace.yaml)
* **Exact-Match Keys**: `billing_profile`, `cbu`, `client_group`, `contract`, `contract_template`, `deal`, `deal_contract`, `deal_product`, `group_kyc_clearance`, `kyc_case`, `workspace_root`
* **DAG-Only Keys**: `billing_account_target`, `billing_period`, `client_principal_relationship`, `deal_document`, `deal_onboarding_request`, `deal_participant`, `deal_rate_card`, `deal_sla`, `deal_ubo_assessment`, `pricing_config`, `rate_card_line`
* **Constellation-Only Keys (Rich, No Topology)**: 
  * `onboarding_request`
  * `participant`
  * `rate_card`
* **Candidate Name Mismatches**:
  * `onboarding_request` <-> `deal_onboarding_request`
  * `participant` <-> `deal_participant`
  * `rate_card` <-> `deal_rate_card`
  * `rate_card` <-> `rate_card_line`

### 6. `ob-poc.instrument-matrix`
* **DAG Files**: [instrument_matrix_dag.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/dag_taxonomies/instrument_matrix_dag.yaml)
* **Constellation Files**: [instrument_workspace.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/instrument_workspace.yaml), [instrument_template.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/instrument_template.yaml), [trading_streetside.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/trading_streetside.yaml)
* **Exact-Match Keys**: `booking_location`, `booking_principal`, `cash_sweep`, `cbu`, `corporate_action_policy`, `custody`, `delivery`, `group`, `investment_manager_assignment`, `isda_framework`, `legal_entity`, `product`, `service_intent`, `service_resource`, `trade_gateway`, `trading_profile`, `workspace_root`
* **DAG-Only Keys**: `collateral_management`, `corporate_action_event`, `reconciliation`, `settlement_pattern_template`, `trading_activity`, `trading_profile_template`
* **Constellation-Only Keys (Rich, No Topology)**: 
  * `settlement_pattern`
* **Candidate Name Mismatches**:
  * `settlement_pattern` <-> `settlement_pattern_template`

### 7. `ob-poc.kyc`
* **DAG Files**: [kyc_dag.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/dag_taxonomies/kyc_dag.yaml)
* **Constellation Files**: [kyc_onboarding.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/kyc_onboarding.yaml), [kyc_workspace.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/kyc_workspace.yaml), [kyc_extended.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/kyc_extended.yaml)
* **Exact-Match Keys**: `board`, `client_group`, `entity_workstream`, `kyc_case`, `screening`, `tollgate`, `workspace_root`
* **DAG-Only Keys**: `capital`, `case_event`, `coverage`, `doc_request`, `entity_kyc`, `kyc_decision`, `kyc_service_agreement`, `kyc_ubo_evidence`, `kyc_ubo_registry`, `outreach_request`, `partnership`, `red_flag`, `skeleton_build`, `trust`, `ubo_evidence`, `ubo_snapshot`
* **Constellation-Only Keys (Rich, No Topology)**: 
  * `bods`
  * `cbu`
  * `entity`
  * `group_kyc_clearance`
  * `identifier`
  * `kyc_agreement`
  * `partnership_capital`
  * `request`
  * `trust_control`
* **Candidate Name Mismatches**:
  * `entity` <-> `entity_kyc`
  * `group_kyc_clearance` <-> `entity_kyc`
  * `group_kyc_clearance` <-> `kyc_decision`
  * `group_kyc_clearance` <-> `kyc_service_agreement`
  * `group_kyc_clearance` <-> `kyc_ubo_evidence`
  * `group_kyc_clearance` <-> `kyc_ubo_registry`
  * `kyc_agreement` <-> `entity_kyc`
  * `kyc_agreement` <-> `kyc_decision`
  * `kyc_agreement` <-> `kyc_service_agreement`
  * `kyc_agreement` <-> `kyc_ubo_evidence`
  * `kyc_agreement` <-> `kyc_ubo_registry`
  * `partnership_capital` <-> `capital`
  * `partnership_capital` <-> `partnership`
  * `request` <-> `doc_request`
  * `request` <-> `outreach_request`
  * `trust_control` <-> `trust`

### 8. `ob-poc.lifecycle-resources`
* **DAG Files**: [lifecycle_resources_dag.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/dag_taxonomies/lifecycle_resources_dag.yaml)
* **Constellation Files**: [lifecycle_resources.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/lifecycle_resources.yaml)
* **Exact-Match Keys**: `application_instance`, `capability_binding`, `workspace_root`
* **DAG-Only Keys**: `application`
* **Constellation-Only Keys (Rich, No Topology)**: None
* **Candidate Name Mismatches**: None

### 9. `ob-poc.onboarding-request`
* **DAG Files**: [onboarding_request_dag.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/dag_taxonomies/onboarding_request_dag.yaml)
* **Constellation Files**: [onboarding_workspace.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/onboarding_workspace.yaml)
* **Exact-Match Keys**: `cbu_resource_instance`, `onboarding_data_request`, `onboarding_data_request_attr`, `onboarding_data_request_discovery`, `onboarding_data_request_slice`, `provisioning_request`, `resource_owner_principal`, `workspace_root`
* **DAG-Only Keys**: `cbu`, `contract`, `deal`, `onboarding_request`
* **Constellation-Only Keys (Rich, No Topology)**: None
* **Candidate Name Mismatches**: None

### 10. `ob-poc.product-service-taxonomy`
* **DAG Files**: [product_service_taxonomy_dag.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/dag_taxonomies/product_service_taxonomy_dag.yaml)
* **Constellation Files**: [product_service_taxonomy.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/product_service_taxonomy.yaml), [product_workspace.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/product_workspace.yaml)
* **Exact-Match Keys**: `product`, `product_service_option_override`, `resource_owner_principal`, `service`, `service_option`, `service_resource`, `service_resource_fanout_rule`, `service_resource_option_constraint`, `workspace_root`
* **DAG-Only Keys**: `attribute`, `product_service_condition`, `service_version`
* **Constellation-Only Keys (Rich, No Topology)**: 
  * `resource_dictionary`
* **Candidate Name Mismatches**: None

### 11. `ob-poc.semos-maintenance`
* **DAG Files**: [semos_maintenance_dag.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/dag_taxonomies/semos_maintenance_dag.yaml)
* **Constellation Files**: [semos_workspace.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/semos_workspace.yaml), [registry_stewardship.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/registry_stewardship.yaml), [governance_compliance.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/governance_compliance.yaml)
* **Exact-Match Keys**: `attribute_def`, `changeset`, `derivation_spec`, `governance`, `phrase_authoring`, `registry`, `service_resource_def`, `typed_attribute`, `workspace_root`
* **DAG-Only Keys**: `remediation_event`, `shared_atom`
* **Constellation-Only Keys (Rich, No Topology)**: 
  * `access_review`
  * `delegation`
  * `group`
  * `regulatory`
  * `rule`
  * `rule_field`
  * `ruleset`
  * `service_resource_def_governance`
  * `sla`
  * `team`
* **Candidate Name Mismatches**: None

### 12. `ob-poc.session-bootstrap`
* **DAG Files**: [session_bootstrap_dag.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/dag_taxonomies/session_bootstrap_dag.yaml)
* **Constellation Files**: [cbu_workspace.yaml](file:///Users/adamtc007/Developer/ob-poc/rust/config/sem_os_seeds/constellation_maps/cbu_workspace.yaml)
* **Exact-Match Keys**: `workspace_root`
* **DAG-Only Keys**: `client_group`, `session_scope`
* **Constellation-Only Keys (Rich, No Topology)**: 
  * `cbu_resource_instance`
  * `cbu_resource_instance_option_lineage`
  * `cbu_service_option_binding`
  * `onboarding_data_request`
  * `onboarding_data_request_slice`
  * `provisioning_request`
* **Candidate Name Mismatches**: None

---

## C2: Candidate Mismatches Worklist

Below is the aggregated and deduplicated worklist of potential name mismatches (aliases) for review by the architect:

* `cbu_resource_instance` <-> `cbu` (Packs: `ob-poc.book-setup`, `ob-poc.cbu`)
* `cbu_resource_instance` <-> `cbu_corporate_action` (Packs: `ob-poc.cbu`)
* `cbu_resource_instance` <-> `cbu_discovery_state` (Packs: `ob-poc.cbu`)
* `cbu_resource_instance` <-> `cbu_disposition` (Packs: `ob-poc.cbu`)
* `cbu_resource_instance` <-> `cbu_entity_relationship` (Packs: `ob-poc.cbu`)
* `cbu_resource_instance` <-> `cbu_entity_role` (Packs: `ob-poc.cbu`)
* `cbu_resource_instance` <-> `cbu_evidence` (Packs: `ob-poc.cbu`)
* `cbu_resource_instance_option_lineage` <-> `cbu` (Packs: `ob-poc.book-setup`)
* `cbu_service_option_binding` <-> `cbu` (Packs: `ob-poc.book-setup`)
* `entity` <-> `entity_kyc` (Packs: `ob-poc.kyc`)
* `group_kyc_clearance` <-> `entity_kyc` (Packs: `ob-poc.kyc`)
* `group_kyc_clearance` <-> `kyc_decision` (Packs: `ob-poc.kyc`)
* `group_kyc_clearance` <-> `kyc_service_agreement` (Packs: `ob-poc.kyc`)
* `group_kyc_clearance` <-> `kyc_ubo_evidence` (Packs: `ob-poc.kyc`)
* `group_kyc_clearance` <-> `kyc_ubo_registry` (Packs: `ob-poc.kyc`)
* `kyc_agreement` <-> `entity_kyc` (Packs: `ob-poc.kyc`)
* `kyc_agreement` <-> `kyc_decision` (Packs: `ob-poc.kyc`)
* `kyc_agreement` <-> `kyc_service_agreement` (Packs: `ob-poc.kyc`)
* `kyc_agreement` <-> `kyc_ubo_evidence` (Packs: `ob-poc.kyc`)
* `kyc_agreement` <-> `kyc_ubo_registry` (Packs: `ob-poc.kyc`)
* `onboarding_request` <-> `deal_onboarding_request` (Packs: `ob-poc.deal`)
* `participant` <-> `deal_participant` (Packs: `ob-poc.deal`)
* `partnership_capital` <-> `capital` (Packs: `ob-poc.kyc`)
* `partnership_capital` <-> `partnership` (Packs: `ob-poc.kyc`)
* `rate_card` <-> `deal_rate_card` (Packs: `ob-poc.deal`)
* `rate_card` <-> `rate_card_line` (Packs: `ob-poc.deal`)
* `request` <-> `doc_request` (Packs: `ob-poc.kyc`)
* `request` <-> `outreach_request` (Packs: `ob-poc.kyc`)
* `settlement_pattern` <-> `settlement_pattern_template` (Packs: `ob-poc.instrument-matrix`)
* `trust_control` <-> `trust` (Packs: `ob-poc.kyc`)

---

## "WHAT I DID NOT DO" Ledger

In strict compliance with Phase 2 — Step 4 Prep (cont.) read-only constraints:
1. **No Source Edits**: Did not modify any code or configuration file in either repository.
2. **No Dependency Changes**: Did not touch `Cargo.toml` or `Cargo.lock` files.
3. **No Key Unification**: Did not rename slots or workspaces to align their names.
4. **No Git State Mutation (Except Report Commit)**: Staged and committed only this report file.
5. **No System Alterations**: Ran no service deployments or custom validations.
