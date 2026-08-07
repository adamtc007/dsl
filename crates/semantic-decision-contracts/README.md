# semantic-decision-contracts

Stable, host-neutral contracts for semantic decision boards, inference
evidence, deterministic dispositions, and proposal workbooks.

The named crate facade also owns the reusable Semantic Gameboard v1 wire contracts:
canonical `DesignPosition` and `LegalMove` resources, explicit focus, typed attempt
receipts (including non-transition outcomes), governed explanations and feedback,
non-authoritative evidence/belief, graph-delta previews, and append-only design-turn
events. Contract representations remain private; construct values through validated
constructors and consume them through read-only accessors.

The crate validates and canonically hashes admitted values. It contains no
pack loader, model runtime, database adapter, transport, or application
policy. Existing consumers may temporarily use the compatibility re-export at
`sem_os_policy::decision_board`; new code should import this crate directly.

The gameboard kernel is deterministic over explicit inputs. It has no server, storage,
network, clock, random identity or mutable-global dependency. Its six standalone fuzz
targets decode hostile wire bytes, exercise structured operation tapes, canonicalize
every admitted value and compare attempt outcomes with a compact independent reference
model. Permanent findings and replay metadata live under `fuzz/regressions`.
