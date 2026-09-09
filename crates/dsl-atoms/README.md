# dsl-atoms

Data-driven atom kind catalogue for the unified DSL atom grammar. Kinds are
registered by data (`KindCatalogue::register`), parsed from DSL source
(`KindCatalogue::from_source`), or seeded from the built-in set
(`KindCatalogue::builtin`). Classification against a catalogue never falls
back to a default: an unregistered kind is a typed `UnknownKind` error that
names the catalogue.
