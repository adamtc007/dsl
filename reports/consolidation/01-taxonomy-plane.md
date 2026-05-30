# Phase 2 — Step 1: Taxonomy Plane Trace Report

This report documents the static consumer trace of `sem_os_taxonomy` to determine its architectural plane placement.

---

## 1. Consumed Public Surface of `sem_os_taxonomy`

The public surface of the `sem_os_taxonomy` crate consists of:

### A. Functions
* **`build_taxonomy_tree(snapshots: &[SnapshotRow], taxonomy_fqn: &str) -> Option<TaxonomyTree>`**
  * Core constructor to project a single taxonomy FQN.
* **`build_all_taxonomy_trees(snapshots: &[SnapshotRow]) -> Vec<TaxonomyTree>`**
  * Top-level constructor to project all taxonomies present in a snapshot slice.

### B. Output Types & Structs
* **`TaxonomyTree`** (struct)
  * Top-level projected model wrapper containing `meta` and `root` node.
  * Methods: `to_yaml() -> Result<String, serde_yaml::Error>`, `to_json() -> Result<String, serde_json::Error>`.
* **`TaxonomyMeta`** (struct)
  * Represents taxonomy header metadata (`fqn`, `name`, `description`, `domain`, `classification_axis`, `max_depth`).
* **`TaxonomyTreeNode`** (struct)
  * Recursive representation of nodes, holding attributes, labels, classified members, and children nodes.
* **`MemberSummary`** (struct)
  * Summary of a registry object membership mapping (`fqn`, `object_type`, `kind`).
* **`MembershipKind`** (enum, re-exported from `sem_os_ontology`)
  * Enum representing type of mapping (`Direct` or `Inherited`).

---

## 2. Full Consumer Trace

We statically traced all references to `sem_os_taxonomy` across the entire codebase graph, including the `dsl` workspace, the historical separate `sem-os` workspace, and the `ob-poc` workspace (including quarantined crates).

### A. `dsl` Consolidated Workspace
* **[dsl/Cargo.toml](file:///Users/adamtc007/Dev/dsl/Cargo.toml#L9)**:
  ```toml
  "crates/sem_os_taxonomy",
  ```
  *(Workspace member registration)*
* **[dsl/Cargo.toml](file:///Users/adamtc007/Dev/dsl/Cargo.toml#L20)**:
  ```toml
  sem_os_taxonomy = { path = "crates/sem_os_taxonomy" }
  ```
  *(Workspace dependency declaration)*
* **Source Files (`.rs`)**: **0 references**. No crate or module imports, no calls to its public functions or types.

### B. `sem-os` Workspace (Historical Separate Repository)
* **[sem-os/Cargo.toml](file:///Users/adamtc007/Dev/sem-os/Cargo.toml#L7)**:
  ```toml
  "crates/sem_os_taxonomy",
  ```
  *(Workspace member registration)*
* **[sem-os/Cargo.toml](file:///Users/adamtc007/Dev/sem-os/Cargo.toml#L17)**:
  ```toml
  sem_os_taxonomy = { path = "crates/sem_os_taxonomy" }
  ```
  *(Workspace dependency declaration)*
* **Source Files (`.rs`)**: **0 references**. No imports or usages.

### C. `ob-poc` Workspace (Including Quarantined Crates)
* **[ob-poc/rust/Cargo.toml](file:///Users/adamtc007/Developer/ob-poc/rust/Cargo.toml#L377)**:
  ```toml
  sem_os_taxonomy = { git = "https://github.com/adamtc007/dsl",    tag = "v0.1.4" }
  ```
  *(Workspace dependency declaration)*
* **Source Files (`.rs`)**: **0 references**. No imports or references inside quarantined or non-quarantined crates.
* **Cargo Lockfile (`Cargo.lock`)**:
  ```toml
  [[patch.unused]]
  name = "sem_os_taxonomy"
  version = "0.1.0"
  ```
  *(Confirms that Cargo marks the patched crate as unused in the actual compiled dependency graph)*

*All findings above are measured via literal grep searches and cargo dependency analysis.*

---

## 3. Per-Consumer Compile-vs-Runtime Classification

Because there are **0 external consumers** of the `sem_os_taxonomy` crate in the entire system, there are no invocations to classify as COMPILE-PHASE or RUNTIME-LIVE.

The only active usages of the public surface of the crate are its own unit tests defined inside [builder.rs](file:///Users/adamtc007/Dev/dsl/crates/sem_os_taxonomy/src/builder.rs).

---

## 4. Architectural Plane Decision

### Does a non-compiler live runtime consumer of `sem_os_taxonomy` exist?
**NO**.

### Plane Classification
Based on the static trace evidence, `sem_os_taxonomy` is classified as a **compiler-plane capability crate**.
* **Reasoning**:
  1. It performs static projection (building a visual tree of categorizations from database state snapshots) for use by high-level presentation layers (like UI representations).
  2. It has no live state queries, does not run as a background service/daemon, and is not involved in runtime access decisions (ABAC), policy evaluation, or service routing.
  3. Since it is entirely unused by any runtime layer, classifying it on the compiler-plane is the most cohesive and low-risk architecture.

---

## 5. Placements & Layering Contradictions

* **Dependency Direction Check**:
  * Dependencies of `sem_os_taxonomy`: `sem_os_types` and `sem_os_ontology`.
  * Both of these direct dependencies are settled on the **compiler-plane** (schema definitions and AST/ontology metadata).
  * There are no dependency cycles or edges from `sem_os_taxonomy` to the runtime-plane (`sem_os_policy`'s active service layers).
* **Placements Verification**:
  * No contradictions exist with the settled placements of compiler-plane (`sem_os_types`, `sem_os_core`, `sem_os_ontology`, `sem_os_policy::domain_pack`) or runtime-plane (`sem_os_policy`'s ABAC/service) crates.

---

## "WHAT I DID NOT DO" Ledger

We strictly adhered to the read-only, leashed guidelines of Phase 2 — Step 1:
1. **No edits to source files**: No files inside any crate were modified, refactored, or written to.
2. **No cargo mutations**: No modifications were made to dependencies or Cargo profiles.
3. **No relocation of crates**: Did not move `sem_os_taxonomy` or change its package structure.
4. **No other Phase 2 steps**: Restricted execution entirely to static source code tracing and listing of metadata.
5. **No test executions**: No cargo tests were executed for this step.

---
Report compiled by Antigravity on 2026-05-30.
