# sem-os-walk

Board-walk contract for a callout-pattern DSL: a generic edge/guard/precondition AST ([`dialect`]),
one board flattened into an explicit graph ([`expand`]), and the pure guard+role legal-move walk over
it ([`walk`]). Domain-free — no field of any one domain's own piece appears here; a domain builds its
own [`expand::Expanded`] from its own board/region model and maps [`walk::Step`] into its own richer
move type at the boundary.

What a piece is blocked on: [`walk::blocked_on`] names, for every edge out of its node not enabled for
the caller, why — a coordinate the clock has not reached, another party's move, a precondition the
domain's own evaluator says fails or cannot answer, or a permanent reason ([`walk::BlockedOn`]);
[`walk::enabled_moves`] is its complement. [`conformance::assert_partition`] checks an engine's own
view of a node against the contract, edge by edge.

By when: an edge flagged `:by <coordinate>` must be played before that coordinate — its
time-criticality, and the only source of it. Every answer the walk gives about the edge carries the
flag ([`walk::Step::by`]; an unresolved one as `Err`, never dropped). The walk states the flag and
does not enforce it — a move whose coordinate is reached is still offered; what a passed deadline
means is the domain's. [`conformance::assert_by_in_shape`] checks every edge's resolved flag is its
declared one resolved, and that every flag resolves to a coordinate in the board's clock-shape.

First consumer: `corporate-actions-engine`'s `ca-fact` crate (extracted from `crates/ca-fact/src/{model,expand,walk}.rs`,
C0.3, 2026-09-18) — see that repo's `docs/eop/receipts/C0.3.md` for the extraction's own record.
