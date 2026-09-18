//! Minimal fixture proving the walk and the conformance suite over it, independent of any domain.

use std::collections::{BTreeMap, BTreeSet};

use sem_os_walk::dialect::{Edge, From, To};
use sem_os_walk::expand::{Expanded, GuardKind, XEdge};
use sem_os_walk::{available, conformance, legal_entry_moves, legal_moves};

fn edge(id: &str, word: &str, roles: &[&str]) -> Edge {
    Edge {
        id: id.to_owned(),
        word: word.to_owned(),
        coord: None,
        from: From::Nodes(vec!["Open".to_owned()]),
        to: To::Node("Closed".to_owned()),
        roles: roles.iter().map(|r| (*r).to_owned()).collect(),
        guard: Vec::new(),
        pre: Vec::new(),
        se: None,
        tx: Vec::new(),
        region: None,
    }
}

fn fixture() -> Expanded {
    let e = edge("E1", "close-it", &["ops"]);
    Expanded {
        id: "T".to_owned(),
        kind: "event".to_owned(),
        parent: "T".to_owned(),
        nodes: vec!["Open".to_owned(), "Closed".to_owned()],
        terminal: vec!["Closed".to_owned()],
        outcome: vec!["Closed".to_owned()],
        clock_shape: Vec::new(),
        bind: BTreeMap::new(),
        edges: vec![XEdge {
            id: e.id.clone(),
            from: Some("Open".to_owned()),
            to: "Closed".to_owned(),
            edge: e,
            guard: vec![(GuardKind::Reached, Ok("never".to_owned()))],
        }],
    }
}

#[test]
fn a_role_permitted_edge_with_no_guard_is_available() {
    let x = fixture();
    let mut x_open = x.clone();
    x_open.edges[0].guard.clear();
    let reached: BTreeSet<String> = BTreeSet::new();
    let steps = legal_moves(&x_open, "Open", &reached, "ops");
    assert_eq!(steps.len(), 1);
    assert_eq!(steps[0].edge, "E1");
    assert_eq!(steps[0].to, "Closed");
}

#[test]
fn an_unresolved_guard_is_never_satisfied() {
    let x = fixture();
    let reached: BTreeSet<String> = BTreeSet::new();
    assert!(available(&x, "Open", &reached, "ops").is_empty());
    assert!(legal_moves(&x, "Open", &reached, "ops").is_empty());
}

#[test]
fn a_role_not_listed_never_sees_the_edge() {
    let mut x = fixture();
    x.edges[0].guard.clear();
    let reached: BTreeSet<String> = BTreeSet::new();
    assert!(legal_moves(&x, "Open", &reached, "engine").is_empty());
}

#[test]
fn entry_edges_are_the_from_none_set() {
    let x = fixture();
    let reached: BTreeSet<String> = BTreeSet::new();
    assert!(legal_entry_moves(&x, &reached, "ops").is_empty());
}

#[test]
fn the_walk_is_deterministic() {
    let mut x = fixture();
    x.edges[0].guard.clear();
    let reached: BTreeSet<String> = BTreeSet::new();
    conformance::assert_deterministic(&x, "Open", &reached, "ops");
    conformance::assert_entry_deterministic(&x, &reached, "ops");
}
