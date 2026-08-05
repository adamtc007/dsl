# semantic-decision-contracts

Stable, host-neutral contracts for semantic decision boards, inference
evidence, deterministic dispositions, and proposal workbooks.

The crate validates and canonically hashes admitted values. It contains no
pack loader, model runtime, database adapter, transport, or application
policy. Existing consumers may temporarily use the compatibility re-export at
`sem_os_policy::decision_board`; new code should import this crate directly.
