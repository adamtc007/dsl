# semantic-pack

`semantic-pack` parses, validates and deterministically compiles host-owned YAML
semantic declarations into immutable, content-addressed artifacts. It contains
no application vocabulary or ambient filesystem/network access.

Applications supply bytes through `PackSource`, implement the named adapter
bindings in their own crates, and may install admitted artifacts in the
thread-safe in-memory registry.
