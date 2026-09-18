# sem-os-walk

Board-walk contract for a callout-pattern DSL: a generic edge/guard/precondition AST ([`dialect`]),
one board flattened into an explicit graph ([`expand`]), and the pure guard+role legal-move walk over
it ([`walk`]). Domain-free — no field of any one domain's own piece appears here; a domain builds its
own [`expand::Expanded`] from its own board/region model and maps [`walk::Step`] into its own richer
move type at the boundary.

First consumer: `corporate-actions-engine`'s `ca-fact` crate (extracted from `crates/ca-fact/src/{model,expand,walk}.rs`,
C0.3, 2026-09-18) — see that repo's `docs/eop/receipts/C0.3.md` for the extraction's own record.
