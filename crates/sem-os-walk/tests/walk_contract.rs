//! The walk contract beyond determinism (L242): the clock order, the reached-sets it admits, guard
//! satisfaction, coordinate resolution, and `blocked_on` — every reason, and the partition every walk
//! engine over this contract must keep.

use std::collections::{BTreeMap, BTreeSet};

use sem_os_walk::conformance;
use sem_os_walk::dialect::{BindValue, Cmp, Coord, Edge, From, Pred, To};
use sem_os_walk::expand::{resolve_coord, Expanded, GuardKind, XEdge};
use sem_os_walk::{
    blocked_on, enabled_moves, legal_moves, BlockedOn, ClockOrder, PreAnswer, PreEvaluator,
};

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
        by: None,
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
            by: None,
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

fn holds() -> Box<PreEvaluator<'static>> {
    Box::new(|_: &XEdge, _: &Pred, _: &BTreeSet<String>| PreAnswer::Holds)
}

fn answer(a: PreAnswer) -> Box<PreEvaluator<'static>> {
    Box::new(move |_: &XEdge, _: &Pred, _: &BTreeSet<String>| a)
}

#[test]
fn blocked_on_names_every_reason_once_in_declared_order() {
    let x = board(
        vec![
            (GuardKind::Reached, Ok("record".to_owned())),
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
            fact("custody-view"),
        ],
    );
    let pre = |_: &XEdge, p: &Pred, _: &BTreeSet<String>| match p {
        Pred::Fact { name, .. } if name == "attempts" => PreAnswer::Holds,
        Pred::Fact { name, .. } if name == "custody-view" => PreAnswer::Unresolved,
        _ => PreAnswer::Fails,
    };
    let b = blocked_on(&x, "Open", &set(&["ex"]), "engine", &pre);
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
            BlockedOn::PreUnresolved(fact("custody-view")),
        ],
        "the repeated `reached record` is named once"
    );
    assert!(
        b[0].is_dead(),
        "a coordinate passed: never enabled, whatever else it waits on"
    );
    assert!(enabled_moves(&x, "Open", &set(&["ex"]), "engine", &pre).is_empty());
}

#[test]
fn a_reached_guard_outside_the_shape_is_permanent_not_a_wait() {
    let x = board(
        vec![(GuardKind::Reached, Ok("pay".to_owned()))],
        &["engine"],
        Vec::new(),
    );
    let b = blocked_on(&x, "Open", &set(&["ex", "record"]), "engine", &*holds());
    assert_eq!(b[0].on, vec![BlockedOn::NotInShape("pay".to_owned())]);
    assert!(b[0].is_dead());
    assert_eq!(
        b[0].on[0].clock_coord(),
        None,
        "no mark for a coordinate the board never reaches"
    );
}

#[test]
fn a_before_guard_outside_the_shape_is_always_satisfied() {
    let x = board(
        vec![(GuardKind::Before, Ok("pay".to_owned()))],
        &["engine"],
        Vec::new(),
    );
    assert!(blocked_on(&x, "Open", &set(&["ex", "record"]), "engine", &*holds()).is_empty());
}

#[test]
fn an_unresolved_has_guard_is_unbound() {
    let x = board(
        vec![(GuardKind::Has, Err("bound".to_owned()))],
        &["engine"],
        Vec::new(),
    );
    let b = blocked_on(&x, "Open", &set(&[]), "engine", &*holds());
    assert_eq!(b[0].on, vec![BlockedOn::Unbound("bound".to_owned())]);
    assert!(b[0].is_dead());
}

#[test]
fn a_reached_precondition_is_clock_shaped_and_its_negation_is_not() {
    let x = board(
        Vec::new(),
        &["engine"],
        vec![
            Pred::Reached("record".to_owned()),
            Pred::Not(Box::new(Pred::Reached("ex".to_owned()))),
        ],
    );
    for a in [PreAnswer::Fails, PreAnswer::Unresolved] {
        let b = blocked_on(&x, "Open", &set(&[]), "engine", &*answer(a));
        assert_eq!(b[0].on.len(), 2, "{a:?}");
        assert_eq!(b[0].on[0].clock_coord(), Some("record"), "{a:?}");
        assert_eq!(b[0].on[1].clock_coord(), None, "{a:?}");
        assert!(
            !b[0].is_dead(),
            "a precondition is never permanent to the walk"
        );
    }
}

#[test]
fn a_move_no_role_may_make_is_permanent() {
    let x = board(Vec::new(), &[], Vec::new());
    let b = blocked_on(&x, "Open", &set(&[]), "engine", &*holds());
    assert_eq!(b[0].on, vec![BlockedOn::Party(Vec::new())]);
    assert!(b[0].is_dead());
}

#[test]
fn edges_that_render_as_one_step_partition_by_position() {
    // three edges out of Open: E1 awaits `ex`; T is custody's; a second E1 (same id, word, target and
    // roles — the same step) with no guard. The contract classifies each by position.
    let mut x = board(
        vec![(GuardKind::Reached, Ok("ex".to_owned()))],
        &["engine"],
        Vec::new(),
    );
    let mut t = x.edges[0].clone();
    t.id = "T".to_owned();
    t.edge.id = "T".to_owned();
    t.to = "Other".to_owned();
    t.edge.roles = vec!["inbound".to_owned()];
    t.guard.clear();
    let mut twin = x.edges[0].clone();
    twin.guard.clear();
    x.edges.push(t);
    x.edges.push(twin);
    let reached = set(&[]);
    let pre = holds();
    let enabled = enabled_moves(&x, "Open", &reached, "engine", &*pre);
    let blocked = blocked_on(&x, "Open", &reached, "engine", &*pre);
    assert_eq!(enabled.len(), 1);
    assert_eq!(blocked.len(), 2);
    conformance::assert_partition(&x, "Open", &reached, "engine", &*pre, &enabled, &blocked);
}

#[test]
#[should_panic(expected = "blocked edges out of Open out of board order")]
fn the_partition_check_rejects_an_engine_that_swaps_its_blocked_edges() {
    let mut x = board(
        vec![(GuardKind::Reached, Ok("record".to_owned()))],
        &["engine"],
        Vec::new(),
    );
    let mut t = x.edges[0].clone();
    t.id = "T".to_owned();
    t.edge.id = "T".to_owned();
    t.edge.roles = vec!["inbound".to_owned()];
    t.guard.clear();
    x.edges.push(t);
    let reached = set(&[]);
    let pre = holds();
    let enabled = enabled_moves(&x, "Open", &reached, "engine", &*pre);
    let mut blocked = blocked_on(&x, "Open", &reached, "engine", &*pre);
    blocked.reverse();
    conformance::assert_partition(&x, "Open", &reached, "engine", &*pre, &enabled, &blocked);
}

#[test]
#[should_panic(expected = "are not the contract's")]
fn the_partition_check_rejects_an_engine_that_enables_a_blocked_edge() {
    let x = board(
        vec![(GuardKind::Reached, Ok("record".to_owned()))],
        &["engine"],
        Vec::new(),
    );
    let reached = set(&[]);
    let pre = holds();
    let enabled = legal_moves(&x, "Open", &set(&["ex", "record"]), "engine");
    conformance::assert_partition(&x, "Open", &reached, "engine", &*pre, &enabled, &[]);
}

#[test]
fn a_clock_shaped_block_clears_when_its_coordinate_is_reached() {
    let x = board(
        vec![(GuardKind::Reached, Ok("record".to_owned()))],
        &["engine"],
        Vec::new(),
    );
    let yes = holds();
    let b = blocked_on(&x, "Open", &set(&["ex"]), "engine", &*yes);
    assert_eq!(b[0].on, vec![BlockedOn::Awaits("record".to_owned())]);
    assert_eq!(b[0].on[0].clock_coord(), Some("record"));
    assert!(!b[0].is_dead());
    assert!(blocked_on(&x, "Open", &set(&["ex", "record"]), "engine", &*yes).is_empty());
    assert_eq!(
        enabled_moves(&x, "Open", &set(&["ex", "record"]), "engine", &*yes),
        legal_moves(&x, "Open", &set(&["ex", "record"]), "engine")
    );
}

#[test]
fn a_party_shaped_block_is_another_roles_move() {
    let x = board(Vec::new(), &["inbound"], Vec::new());
    let b = blocked_on(&x, "Open", &set(&[]), "engine", &*holds());
    assert_eq!(b[0].on, vec![BlockedOn::Party(vec!["inbound".to_owned()])]);
    assert!(!b[0].is_dead());
    assert!(
        blocked_on(&x, "Open", &set(&[]), "inbound", &*holds()).is_empty(),
        "custody's own view"
    );
}

#[test]
fn a_node_with_no_edges_out_is_blocked_on_nothing() {
    let x = board(Vec::new(), &["engine"], Vec::new());
    assert!(blocked_on(&x, "Closed", &set(&[]), "engine", &*holds()).is_empty());
    assert!(enabled_moves(&x, "Closed", &set(&[]), "engine", &*holds()).is_empty());
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
            for a in [PreAnswer::Holds, PreAnswer::Fails, PreAnswer::Unresolved] {
                let pre = answer(a);
                let enabled = enabled_moves(&x, "Open", &reached, role, &*pre);
                let blocked = blocked_on(&x, "Open", &reached, role, &*pre);
                conformance::assert_partition(
                    &x, "Open", &reached, role, &*pre, &enabled, &blocked,
                );
                conformance::assert_blocked_deterministic(&x, "Open", &reached, role, &*pre);
            }
        }
    }
}

#[test]
#[should_panic(expected = "too many")]
fn reached_sets_refuses_a_shape_it_cannot_enumerate() {
    let shape: Vec<String> = (0..32).map(|i| format!("c{i}")).collect();
    let _ = order().reached_sets(&shape);
}

/// `board`, its one edge flagged `:by` (resolved or not).
fn flagged(by: Result<&str, &str>, roles: &[&str]) -> Expanded {
    let mut x = board(Vec::new(), roles, Vec::new());
    x.edges[0].by = Some(by.map(str::to_owned).map_err(str::to_owned));
    x.edges[0].edge.by = Some(match by {
        Ok(c) => sem_os_walk::dialect::Coord::Named(c.to_owned()),
        Err(b) => sem_os_walk::dialect::Coord::Bound(b.to_owned()),
    });
    x
}

#[test]
fn every_answer_about_a_flagged_edge_carries_its_by() {
    let x = flagged(Ok("record"), &["ops"]);
    let none = set(&[]);
    let holds = |_: &XEdge, _: &Pred, _: &BTreeSet<String>| PreAnswer::Holds;
    let legal = legal_moves(&x, "Open", &none, "ops");
    assert_eq!(legal.len(), 1);
    assert_eq!(legal[0].by.as_deref(), Some("record"));
    let enabled = enabled_moves(&x, "Open", &none, "ops", &holds);
    assert_eq!(enabled[0].by.as_deref(), Some("record"));
    // another party's move: blocked, and still flagged — whose move it is and by when
    let blocked = blocked_on(&x, "Open", &none, "engine", &holds);
    assert_eq!(blocked.len(), 1);
    assert_eq!(blocked[0].step.by.as_deref(), Some("record"));
    conformance::assert_by_in_shape(&x);
    // an unflagged edge carries none
    let plain = board(Vec::new(), &["ops"], Vec::new());
    assert_eq!(legal_moves(&plain, "Open", &none, "ops")[0].by, None);
    conformance::assert_by_in_shape(&plain);
}

#[test]
fn an_unresolved_by_is_never_guessed() {
    let x = flagged(Err("cutoff"), &["ops"]);
    assert_eq!(legal_moves(&x, "Open", &set(&[]), "ops")[0].by, None);
    let unresolved = std::panic::catch_unwind(|| conformance::assert_by_in_shape(&x));
    assert!(unresolved.is_err(), "an unresolved :by passes conformance");
}

#[test]
fn a_by_outside_the_clock_shape_fails_conformance() {
    let x = flagged(Ok("pay"), &["ops"]);
    let outside = std::panic::catch_unwind(|| conformance::assert_by_in_shape(&x));
    assert!(outside.is_err(), "a :by the board never reaches passes conformance");
}
