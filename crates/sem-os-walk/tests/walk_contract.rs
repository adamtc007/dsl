//! The walk contract beyond determinism (L242): the clock order, the reached-sets it admits, guard
//! satisfaction, coordinate resolution, and `blocked_on` — every reason, and the partition every walk
//! engine over this contract must keep.

use std::collections::{BTreeMap, BTreeSet};

use sem_os_walk::conformance;
use sem_os_walk::dialect::{BindValue, Cmp, Coord, Edge, From, Pred, To};
use sem_os_walk::expand::{resolve_coord, Expanded, GuardKind, XEdge};
use sem_os_walk::{blocked_on, enabled_moves, legal_moves, BlockedOn, ClockOrder};

fn set(v: &[&str]) -> BTreeSet<String> {
    v.iter().map(|s| (*s).to_owned()).collect()
}

fn order() -> ClockOrder {
    ClockOrder::new(&[
        ("ex".to_owned(), "record".to_owned()),
        ("record".to_owned(), "pay".to_owned()),
    ])
}

#[test]
fn the_clock_order_is_transitively_closed() {
    let o = order();
    assert!(o.is_before("ex", "record"));
    assert!(o.is_before("record", "pay"));
    assert!(o.is_before("ex", "pay"), "closed: ex < record < pay");
    assert!(!o.is_before("pay", "ex"));
    assert!(!o.is_before("ex", "ex"), "strict");
    assert!(!o.is_before("ex", "unknown"));
}

#[test]
fn reached_sets_are_exactly_the_down_closed_sets() {
    let o = order();
    let shape: Vec<String> = ["ex", "record", "pay", "close"]
        .iter()
        .map(|s| (*s).to_owned())
        .collect();
    let sets = o.reached_sets(&shape);
    // a chain of three (4 prefixes) times an unordered fourth (in or out) = 8
    assert_eq!(sets.len(), 8, "{sets:?}");
    assert!(sets.contains(&set(&[])));
    assert!(sets.contains(&set(&["ex", "record"])));
    assert!(sets.contains(&set(&["close"])));
    assert!(sets.contains(&set(&["ex", "record", "pay", "close"])));
    assert!(
        !sets.contains(&set(&["record"])),
        "record without ex is not down-closed"
    );
    assert!(!sets.contains(&set(&["ex", "pay"])), "pay without record");
    let unique: BTreeSet<_> = sets.iter().cloned().collect();
    assert_eq!(unique.len(), sets.len(), "no set twice");
}

fn board(
    guard: Vec<(GuardKind, Result<String, String>)>,
    roles: &[&str],
    pre: Vec<Pred>,
) -> Expanded {
    let e = Edge {
        id: "E1".to_owned(),
        word: "move-it".to_owned(),
        coord: None,
        from: From::Nodes(vec!["Open".to_owned()]),
        to: To::Node("Closed".to_owned()),
        roles: roles.iter().map(|r| (*r).to_owned()).collect(),
        guard: Vec::new(),
        pre,
        se: None,
        tx: Vec::new(),
        region: None,
    };
    Expanded {
        id: "T".to_owned(),
        kind: "event".to_owned(),
        parent: "T".to_owned(),
        nodes: vec!["Open".to_owned(), "Closed".to_owned()],
        terminal: vec!["Closed".to_owned()],
        outcome: vec!["Closed".to_owned()],
        clock_shape: vec!["ex".to_owned(), "record".to_owned()],
        bind: BTreeMap::new(),
        edges: vec![XEdge {
            id: e.id.clone(),
            from: Some("Open".to_owned()),
            to: "Closed".to_owned(),
            edge: e,
            guard,
        }],
    }
}

#[test]
fn guard_satisfied_reads_each_kind_and_never_an_unresolved_binding() {
    let x = board(Vec::new(), &["ops"], Vec::new());
    let g = |k, c: Result<&str, &str>| vec![(k, c.map(str::to_owned).map_err(str::to_owned))];
    let none = set(&[]);
    let ex = set(&["ex"]);
    assert!(ClockOrder::guard_satisfied(
        &x,
        &g(GuardKind::Before, Ok("ex")),
        &none
    ));
    assert!(!ClockOrder::guard_satisfied(
        &x,
        &g(GuardKind::Before, Ok("ex")),
        &ex
    ));
    assert!(!ClockOrder::guard_satisfied(
        &x,
        &g(GuardKind::Reached, Ok("ex")),
        &none
    ));
    assert!(ClockOrder::guard_satisfied(
        &x,
        &g(GuardKind::Reached, Ok("ex")),
        &ex
    ));
    assert!(ClockOrder::guard_satisfied(
        &x,
        &g(GuardKind::Has, Ok("record")),
        &none
    ));
    assert!(!ClockOrder::guard_satisfied(
        &x,
        &g(GuardKind::Has, Ok("pay")),
        &ex
    ));
    for k in [GuardKind::Before, GuardKind::Reached, GuardKind::Has] {
        assert!(
            !ClockOrder::guard_satisfied(&x, &g(k, Err("b")), &set(&["b"])),
            "an unresolved binding is never satisfied ({k:?})"
        );
    }
    assert!(ClockOrder::guard_satisfied(&x, &[], &none), "no guard");
    let both = vec![
        (GuardKind::Reached, Ok("ex".to_owned())),
        (GuardKind::Before, Ok("record".to_owned())),
    ];
    assert!(ClockOrder::guard_satisfied(&x, &both, &ex), "a conjunction");
    assert!(!ClockOrder::guard_satisfied(
        &x,
        &both,
        &set(&["ex", "record"])
    ));
}

#[test]
fn resolve_coord_names_the_literal_the_bound_symbol_or_the_unresolved_binding() {
    let mut bind = BTreeMap::new();
    bind.insert("posting".to_owned(), BindValue::Symbol("pay".to_owned()));
    bind.insert("listy".to_owned(), BindValue::Nil);
    assert_eq!(
        resolve_coord(&bind, &Coord::Named("ex".to_owned())),
        Ok("ex".to_owned())
    );
    assert_eq!(
        resolve_coord(&bind, &Coord::Bound("posting".to_owned())),
        Ok("pay".to_owned())
    );
    assert_eq!(
        resolve_coord(&bind, &Coord::Bound("missing".to_owned())),
        Err("missing".to_owned())
    );
    assert_eq!(
        resolve_coord(&bind, &Coord::Bound("listy".to_owned())),
        Err("listy".to_owned()),
        "a binding that is not a symbol does not resolve to a coordinate"
    );
}

fn fact(name: &str) -> Pred {
    Pred::Fact {
        name: name.to_owned(),
        cmp: None,
    }
}

#[test]
fn blocked_on_names_every_reason_in_declared_order() {
    let x = board(
        vec![
            (GuardKind::Reached, Ok("record".to_owned())),
            (GuardKind::Before, Ok("ex".to_owned())),
            (GuardKind::Has, Ok("pay".to_owned())),
            (GuardKind::Reached, Err("bound".to_owned())),
        ],
        &["inbound"],
        vec![
            fact("acknowledgement"),
            Pred::Fact {
                name: "attempts".to_owned(),
                cmp: Some((Cmp::Below, "budget".to_owned())),
            },
        ],
    );
    let only_attempts =
        |_: &XEdge, p: &Pred| matches!(p, Pred::Fact { name, .. } if name == "attempts");
    let b = blocked_on(&x, "Open", &set(&["ex"]), "engine", &only_attempts);
    assert_eq!(b.len(), 1);
    assert_eq!(b[0].step.edge, "E1");
    assert_eq!(
        b[0].on,
        vec![
            BlockedOn::Awaits("record".to_owned()),
            BlockedOn::Passed("ex".to_owned()),
            BlockedOn::NotInShape("pay".to_owned()),
            BlockedOn::Unbound("bound".to_owned()),
            BlockedOn::Party(vec!["inbound".to_owned()]),
            BlockedOn::Pre(fact("acknowledgement")),
        ]
    );
    assert!(enabled_moves(&x, "Open", &set(&["ex"]), "engine", &only_attempts).is_empty());
}

#[test]
fn a_clock_shaped_block_clears_when_its_coordinate_is_reached() {
    let x = board(
        vec![(GuardKind::Reached, Ok("record".to_owned()))],
        &["engine"],
        Vec::new(),
    );
    let yes = |_: &XEdge, _: &Pred| true;
    assert_eq!(
        blocked_on(&x, "Open", &set(&["ex"]), "engine", &yes)[0].on,
        vec![BlockedOn::Awaits("record".to_owned())]
    );
    assert!(blocked_on(&x, "Open", &set(&["ex", "record"]), "engine", &yes).is_empty());
    assert_eq!(
        enabled_moves(&x, "Open", &set(&["ex", "record"]), "engine", &yes),
        legal_moves(&x, "Open", &set(&["ex", "record"]), "engine")
    );
}

#[test]
fn a_party_shaped_block_is_another_roles_move() {
    let x = board(Vec::new(), &["inbound"], Vec::new());
    let yes = |_: &XEdge, _: &Pred| true;
    let b = blocked_on(&x, "Open", &set(&[]), "engine", &yes);
    assert_eq!(b[0].on, vec![BlockedOn::Party(vec!["inbound".to_owned()])]);
    assert!(
        blocked_on(&x, "Open", &set(&[]), "inbound", &yes).is_empty(),
        "custody's own view"
    );
}

#[test]
fn a_node_with_no_edges_out_is_blocked_on_nothing() {
    let x = board(Vec::new(), &["engine"], Vec::new());
    let yes = |_: &XEdge, _: &Pred| true;
    assert!(blocked_on(&x, "Closed", &set(&[]), "engine", &yes).is_empty());
    assert!(enabled_moves(&x, "Closed", &set(&[]), "engine", &yes).is_empty());
}

#[test]
fn the_conformance_suite_holds_over_every_reached_set_role_and_answer() {
    let mut x = board(
        vec![(GuardKind::Reached, Ok("record".to_owned()))],
        &["engine"],
        vec![fact("f")],
    );
    // a second edge out of Open: a party's move, no guard
    let mut e2 = x.edges[0].clone();
    e2.id = "E2".to_owned();
    e2.edge.id = "E2".to_owned();
    e2.edge.roles = vec!["inbound".to_owned()];
    e2.guard.clear();
    e2.edge.pre.clear();
    x.edges.push(e2);
    let shape = x.clock_shape.clone();
    for reached in ClockOrder::new(&[("ex".to_owned(), "record".to_owned())]).reached_sets(&shape) {
        for role in ["engine", "inbound", "ops"] {
            for answer in [true, false] {
                let holds = move |_: &XEdge, _: &Pred| answer;
                conformance::assert_partition(&x, "Open", &reached, role, &holds);
                conformance::assert_blocked_deterministic(&x, "Open", &reached, role, &holds);
            }
        }
    }
}
