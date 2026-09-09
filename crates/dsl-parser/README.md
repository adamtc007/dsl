# dsl-parser

S-expression lexer and tolerant recursive-descent parser for the unified DSL
atom grammar. Produces a raw, untyped `SourceFile` of `RawAtom`s; kind
classification happens in `dsl-ast` against a `dsl-atoms` catalogue.

Slots are keyword-form only (`:slot value`) outside the `flow` arrow sugar.
Every keyword slot is preserved verbatim in the raw tree, including slots no
frontend consumes.
