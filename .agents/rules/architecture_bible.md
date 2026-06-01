# Antigravity Workspace Constitution & Architecture Bible

This document serves as the immutable architectural constitution for the DSL Compiler and SemOS runtime environment. All developers and agent systems must strictly adhere to these tenets.

---

## 1. Core Principles

### 1.1 First-Principles Language Specification
Priority is always given to formal language specifications, structural AST validation rules, and compiler grammar over legacy example patterns or historical code artifacts. If a legacy pattern deviates from the grammar definition, the specification is correct, and the code must be aligned to it.

### 1.2 Target Symmetry: Rust to Java 25 Data-Oriented Programming (DOP)
We maintain a strict, 1-to-1 structural mapping from the Rust compiler's data representations to pure **Java 25 Data-Oriented Programming (DOP)** patterns.
* **Rust Enums (with payloads)** map to Java 25 **sealed interfaces** implemented by **records**.
* **Rust Structs** map to Java 25 **records** (immutable data carriers).
* **Pattern Matching** on enums maps to Java 25 **exhaustive pattern-matched switch expressions** (without defaults, ensuring compile-time completeness).

#### Reference Mapping Example
##### Rust Source
```rust
pub enum Statement {
    VerbCall(VerbCall),
    Definition(Definition),
    Composition(Composition),
}

pub struct VerbCall {
    pub domain: String,
    pub verb: String,
    pub arguments: Vec<Argument>,
    pub lens_override: Option<String>,
    pub binding: Option<String>,
}
```

##### Java 25 Target Representation
```java
// Immutable records and sealed hierarchy representing the AST
public sealed interface Statement permits VerbCall, Definition, Composition {}

public record VerbCall(
    String domain,
    String verb,
    List<Argument> arguments,
    Optional<String> lensOverride,
    Optional<String> binding
) implements Statement {}

// Exhaustive pattern-matched switch expressions
public static String formatStatement(Statement stmt) {
    return switch (stmt) {
        case VerbCall vc -> String.format("(%s.%s)", vc.domain(), vc.verb());
        case Definition d -> String.format("define %s", d.name());
        case Composition c -> String.format("compose %s", c.id());
    }; // Exhaustive: no default label allowed
}
```

---

## 2. Strict Runtime Constraints

* **Pure POJO Runtime**: Banned frameworks include **Spring Boot**, **Hibernate/ORMs**, dependency-injection containers (Guice, Spring Core), reflection, or bytecode-manipulating class loaders. 
* **Zero Boilerplate**: Use pure, lightweight Java 25 standard library APIs. State transitions, DAG evaluations, and parsing loops must run in an un-frameworked POJO context.
* **Hermetic Execution**: Execution logic must rely entirely on local state passes (immutable records and pure functions) rather than thread-local contexts or global static singleton registrations.

---

## 3. Non-Linear Strategy (Refactoring Tolerance)

* **Accept Temporary Broken States**: Multi-file compiler and parser refactoring loops will temporarily drop the workspace into a broken compiler/linter state. 
* **Strict Correctness**: The agent must **NOT** panic, abort the run, or attempt to throw global linter bypass flags (such as `#![allow(unused)]` or wildcard suppression) to short-circuit the execution. Correctness must be achieved by systematically resolving all compile-time and lint errors in order.
* **Incremental Resolution**: Follow topological dependency ordering: parse tree -> AST -> semantic resolver -> code generator.

---

## 4. SemOS Pack / DAG Context Integration

Every workbook and instruction execution must track and enforce pack context:
1. **Admissibility**: Verify that verbs are in the pack's `allowed_verbs` and slots are in `workspaces`.
2. **Lens Resolution**: Dynamically map `domain_lens_id` or statement-level `lens_override` to the target workspace/slot coordinates.
3. **Pure Execution Fallback**: When no context is provided, allow backward-compatible un-governed fallback mode (defaulting domain to workspace).
