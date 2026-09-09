//! Keyword slots the parser must carry verbatim, including slots that no
//! shared frontend consumes: `:guard`, `:pre`, `:roles`, `:se`, `:bind`,
//! `:reads-as`.

use dsl_parser::{parse, RawAtom, RawValue, SourceFile};

const SOURCE: &str = r#"
(node settle
  :guard (guard :deadline record-date :on-expiry settle-passed)
  :pre (pre :all [entitled elected])
  :roles [custodian issuer-agent]
  :se (se :steps [(seq :of [book-cash book-stock]) (par :of [notify-holder notify-agent])])
  :bind [(bind :abstract place :concrete book-stock) (bind :abstract remove :concrete nil)]
  :reads-as "settle {event} for {holder}")
"#;

/// Minimal source printer, local to this test: parse → print → parse must be
/// the identity on the raw tree.
fn print_value(value: &RawValue) -> String {
    match value {
        RawValue::Atom(atom) => print_atom(atom),
        RawValue::List(items) => {
            let inner: Vec<String> = items.iter().map(print_value).collect();
            format!("[{}]", inner.join(" "))
        }
        RawValue::Map(pairs) => {
            let inner: Vec<String> = pairs
                .iter()
                .map(|(k, v)| format!(":{k} {}", print_value(v)))
                .collect();
            format!("{{{}}}", inner.join(" "))
        }
        RawValue::Symbol(s) => s.clone(),
        RawValue::QualifiedName { pack, atom } => format!("{pack}/{atom}"),
        RawValue::StringLit(s) => format!("{:?}", s),
        RawValue::IntLit(i) => i.to_string(),
        RawValue::FloatLit(f) => format!("{f:?}"),
        RawValue::BoolLit(b) => b.to_string(),
        RawValue::SlotRef(s) => format!("@{s}"),
        RawValue::TemplateSubst(s) => format!(",{s}"),
        RawValue::TemplateSplice(s) => format!(",@{s}"),
        RawValue::InsertionMarker(s) => format!("${s}"),
        RawValue::ForEach {
            var,
            list_param,
            body,
        } => {
            let inner: Vec<String> = body.iter().map(print_value).collect();
            format!("(for-each :var {var} :in {list_param} {})", inner.join(" "))
        }
    }
}

fn print_atom(atom: &RawAtom) -> String {
    let mut out = format!("({}", atom.kind);
    if let Some(name) = &atom.name {
        out.push(' ');
        out.push_str(name);
    }
    for (slot, value) in &atom.slots {
        out.push_str(&format!(" :{slot} {}", print_value(value)));
    }
    out.push(')');
    out
}

fn print_file(file: &SourceFile) -> String {
    file.atoms
        .iter()
        .map(print_atom)
        .collect::<Vec<_>>()
        .join("\n")
}

fn parse_ok(src: &str) -> SourceFile {
    let (file, diagnostics) = parse(src);
    assert!(
        !diagnostics.has_errors(),
        "parse errors: {:?}",
        diagnostics
            .errors()
            .map(|d| d.message.as_str())
            .collect::<Vec<_>>()
    );
    file
}

#[test]
fn six_unconsumed_slots_are_preserved_verbatim() {
    let file = parse_ok(SOURCE);
    assert_eq!(file.atoms.len(), 1);
    let atom = &file.atoms[0];
    assert_eq!(atom.kind, "node");
    assert_eq!(atom.name.as_deref(), Some("settle"));

    let slot_names: Vec<&str> = atom.slots.iter().map(|(k, _)| k.as_str()).collect();
    assert_eq!(
        slot_names,
        ["guard", "pre", "roles", "se", "bind", "reads-as"],
        "slots must be kept in source order with their source names"
    );

    let slot = |name: &str| -> &RawValue {
        &atom
            .slots
            .iter()
            .find(|(k, _)| k == name)
            .unwrap_or_else(|| panic!("slot :{name} missing"))
            .1
    };

    match slot("guard") {
        RawValue::Atom(guard) => {
            assert_eq!(guard.kind, "guard");
            assert_eq!(
                guard.slots,
                vec![
                    (
                        "deadline".to_owned(),
                        RawValue::Symbol("record-date".to_owned())
                    ),
                    (
                        "on-expiry".to_owned(),
                        RawValue::Symbol("settle-passed".to_owned())
                    ),
                ]
            );
        }
        other => panic!("expected nested guard atom, got {other:?}"),
    }
    match slot("pre") {
        RawValue::Atom(pre) => assert_eq!(pre.kind, "pre"),
        other => panic!("expected nested pre atom, got {other:?}"),
    }
    assert_eq!(
        slot("roles"),
        &RawValue::List(vec![
            RawValue::Symbol("custodian".to_owned()),
            RawValue::Symbol("issuer-agent".to_owned()),
        ])
    );
    match slot("se") {
        RawValue::Atom(se) => {
            assert_eq!(se.kind, "se");
            match &se.slots[0].1 {
                RawValue::List(steps) => {
                    assert_eq!(steps.len(), 2);
                    assert!(matches!(&steps[0], RawValue::Atom(a) if a.kind == "seq"));
                    assert!(matches!(&steps[1], RawValue::Atom(a) if a.kind == "par"));
                }
                other => panic!("expected :steps list, got {other:?}"),
            }
        }
        other => panic!("expected nested se atom, got {other:?}"),
    }
    match slot("bind") {
        RawValue::List(binds) => {
            assert_eq!(binds.len(), 2);
            // `nil` is an ordinary symbol; the parser has no null literal.
            assert!(matches!(
                &binds[1],
                RawValue::Atom(b) if b.slots[1].1 == RawValue::Symbol("nil".to_owned())
            ));
        }
        other => panic!("expected bind list, got {other:?}"),
    }
    assert_eq!(
        slot("reads-as"),
        &RawValue::StringLit("settle {event} for {holder}".to_owned()),
        "template braces inside strings are opaque to the parser"
    );
}

#[test]
fn six_slots_round_trip_through_print_and_serde() {
    let file = parse_ok(SOURCE);

    let printed = print_file(&file);
    let reparsed = parse_ok(&printed);
    assert_eq!(reparsed, file, "parse → print → parse must be the identity");

    let json = serde_json::to_string(&file).unwrap();
    let decoded: SourceFile = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded, file, "serde round trip must preserve every slot");
}

#[test]
fn positional_value_outside_flow_is_a_parse_error() {
    let (_, diagnostics) = parse("(bind place book-stock)");
    assert!(diagnostics.has_errors());
    assert!(diagnostics.errors().any(|d| d
        .message
        .contains("Positional value in non-flow atom context")));
}

#[test]
fn flow_arrow_sugar_still_synthesises_source_and_target() {
    let file = parse_ok("(flow a -> b :condition ok)");
    let atom = &file.atoms[0];
    assert_eq!(
        atom.slots[0],
        ("source".to_owned(), RawValue::Symbol("a".to_owned()))
    );
    assert_eq!(
        atom.slots[1],
        ("target".to_owned(), RawValue::Symbol("b".to_owned()))
    );
    assert_eq!(atom.slots[2].0, "condition");
}
