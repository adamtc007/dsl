# Reuse Ledger — EOP-PLAN-CA-REUSE-001 (dsl repo)

- Format: `date · tranche · surface delta · commit · status`
- Surface delta = net change in `cargo public-api --simplified` lines across the crates the tranche touches.

| Date | Tranche | Surface Delta | Commit | Status |
| :--- | :--- | :--- | :--- | :--- |
| 2026-09-09 | 01-dsl-t1-parser-relocation | +592 (four new crates) | 97ac3ae | GREEN |
| 2026-09-09 | 02-dsl-t2-domain-nouns-and-decimals | dsl-core −1 · sem_os_policy +34 | 0967d0b | GREEN |
| 2026-09-09 | 03-dsl-t3-core-crates | +572 (two new crates) · semantic-pack +147 · sem_os_policy +8 | 4060fbc | GREEN |
| 2026-09-09 | 04-dsl-t4-log-only | 0 | 5925702 | LOGGED |
| 2026-09-10 | 05-dsl-t5-parser-hang-and-equals-lexing | dsl-parser +1 (`Token::Slash`) | 9eca60e | GREEN |
| 2026-09-10 | 06-dsl-t6-append-store-id-and-contiguity | sem-os-append-store: full API break (Id-generic) | _this commit_ | GREEN |
