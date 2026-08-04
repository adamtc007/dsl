# Decision-board contract ownership mapping

Base: `a043e7f3d40262b78b367a6c18ac4a937c7498c6`.

| Target concern | Existing shared owner | Implementation decision |
|---|---|---|
| Canonical action identity | `VerbContractBody::fqn` for operations | Add host-neutral `CanonicalCandidateId`; adapters map typed domain ids into it. |
| Arguments | `VerbArgDef` and DSL value types | Project into `ArgumentSpec`; retain typed workbook values in policy until a wider DSL extraction is justified. |
| Action/risk | `ActionClass`, `HarmClass` | Reuse without a parallel enum. |
| Phrases | `VerbContractBody::invocation_phrases` | Add typed phrase role/provenance to the model-visible projection. |
| Position/action surface | `GroundedActionSurface` | `ResolvedPosition` and `SemanticDecisionBoard` refine the closed, model-visible surface; grounding remains authoritative upstream. |
| Pack identity | `DomainPackManifest` and reload surface hash | Board stores the resolved semantic snapshot identity, not mutable loader state. |
| Board/evidence/disposition | no complete pre-existing contract | Add one canonical constructor, finite evidence, deterministic policy, and content hashes in `sem_os_policy::decision_board`. |
| Workbook state | generic DSL values existed; no resumable transition contract | Add typed slots and a closed transition API without host storage or BPMN operations. |

The shared layer deliberately contains no BPMN graph, compiler, Candle, HTTP,
database, or host event-store dependency. `scripts/check-layering.sh` enforces
the application-import boundary.
