#![allow(clippy::zero_prefixed_literal)]

use {
    super::testing::task,
    super::util::*,
    chrono::Duration,
    lookup_key::Key,
    model::{CheckOptions, NewOptions, TodoList},
    printing::{Action::*, Plicit::*, Status::*},
    testing::ymdhms,
    ::pretty_assertions::assert_eq,
};

#[test]
fn format_task_basic() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let actual = format_task(&list, a);
    let expected = task("a", 1, Incomplete);
    assert_eq!(actual, expected);
}

#[test]
fn format_task_with_action() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let actual = format_task(&list, a).action(Punt);
    let expected = task("a", 1, Incomplete).action(Punt);
    assert_eq!(actual, expected);
}

#[test]
fn format_task_with_priority() {
    let mut list = TodoList::default();
    let a = list.add(NewOptions::new().desc("a").priority(1));
    let actual = format_task(&list, a);
    let expected = task("a", 1, Incomplete).priority(Explicit(1));
    assert_eq!(actual, expected);
}

#[test]
fn format_task_with_zero_priority() {
    let mut list = TodoList::default();
    let a = list.add(NewOptions::new().desc("a").priority(0));
    let actual = format_task(&list, a);
    let expected = task("a", 1, Incomplete);
    assert_eq!(actual, expected);
}

#[test]
fn format_task_with_budget() {
    let mut list = TodoList::default();
    let now = ymdhms(2021, 04, 30, 09, 00, 00);
    let due = now + Duration::hours(5);
    let a = list.add(
        NewOptions::new()
            .desc("a")
            .budget(Duration::hours(10))
            .due_date(due),
    );
    let actual = format_task(&list, a);
    let expected = task("a", 1, Incomplete)
        .due_date(Explicit(due))
        .budget(Duration::hours(10));
    assert_eq!(actual, expected);
}

#[test]
fn format_incomplete_task_with_adep_stats() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(b).on(a).unwrap();
    list.block(c).on(b).unwrap();
    let actual = format_task(&list, a);
    let expected = task("a", 1, Incomplete).adeps_stats(1, 2);
    assert_eq!(actual, expected);
}

#[test]
fn format_incomplete_task_with_all_all_adeps_unlockable() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(b).on(a).unwrap();
    list.block(c).on(a).unwrap();
    let actual = format_task(&list, a);
    let expected = task("a", 1, Incomplete).adeps_stats(2, 2);
    assert_eq!(actual, expected);
}

#[test]
fn format_incomplete_task_with_long_adep_chain() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    let d = list.add("d");
    let e = list.add("e");
    let f = list.add("f");
    let g = list.add("g");
    let h = list.add("h");
    let i = list.add("i");
    let j = list.add("j");
    let k = list.add("k");
    let l = list.add("l");
    let m = list.add("m");
    let n = list.add("n");
    let o = list.add("o");
    let p = list.add("p");
    let q = list.add("q");
    let r = list.add("r");
    let s = list.add("s");
    let t = list.add("t");
    let u = list.add("u");
    let v = list.add("v");
    let w = list.add("w");
    let x = list.add("x");
    let y = list.add("y");
    let z = list.add("z");
    list.block(b).on(a).unwrap();
    list.block(c).on(b).unwrap();
    list.block(d).on(c).unwrap();
    list.block(e).on(d).unwrap();
    list.block(f).on(e).unwrap();
    list.block(g).on(f).unwrap();
    list.block(h).on(g).unwrap();
    list.block(i).on(h).unwrap();
    list.block(j).on(i).unwrap();
    list.block(k).on(j).unwrap();
    list.block(l).on(k).unwrap();
    list.block(m).on(l).unwrap();
    list.block(n).on(m).unwrap();
    list.block(o).on(n).unwrap();
    list.block(p).on(o).unwrap();
    list.block(q).on(p).unwrap();
    list.block(r).on(q).unwrap();
    list.block(s).on(r).unwrap();
    list.block(t).on(s).unwrap();
    list.block(u).on(t).unwrap();
    list.block(v).on(u).unwrap();
    list.block(w).on(v).unwrap();
    list.block(x).on(w).unwrap();
    list.block(y).on(x).unwrap();
    list.block(z).on(y).unwrap();
    let actual = format_task(&list, a);
    let expected = task("a", 1, Incomplete).adeps_stats(1, 25);
    assert_eq!(actual, expected);
}

#[test]
fn format_blocked_task_with_two_deps() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(c).on(a).unwrap();
    list.block(c).on(b).unwrap();
    let actual = format_task(&list, c);
    let expected = task("c", 3, Blocked).deps_stats(2, 2);
    assert_eq!(actual, expected);
}

#[test]
fn format_blocked_task_with_one_dep() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    list.block(b).on(a).unwrap();
    let actual = format_task(&list, b);
    let expected = task("b", 2, Blocked).deps_stats(1, 1);
    assert_eq!(actual, expected);
}

#[test]
fn format_blocked_task_with_complete_deps() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(c).on(a).unwrap();
    list.block(c).on(b).unwrap();
    list.check(a).unwrap();
    let actual = format_task(&list, c);
    let expected = task("c", 2, Blocked).deps_stats(1, 2);
    assert_eq!(actual, expected);
}

#[test]
fn format_blocked_task_with_blocked_deps() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(b).on(a).unwrap();
    list.block(c).on(b).unwrap();
    let actual = format_task(&list, c);
    let expected = task("c", 3, Blocked).deps_stats(1, 2);
    assert_eq!(actual, expected);
}

#[test]
fn format_blocked_task_with_deps_and_adeps() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(b).on(a).unwrap();
    list.block(c).on(b).unwrap();
    let actual = format_task(&list, b);
    let expected = task("b", 2, Blocked).deps_stats(1, 1);
    assert_eq!(actual, expected);
}

#[test]
fn format_complete_task_does_not_show_deps_or_adeps() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    list.block(b).on(a).unwrap();
    list.block(c).on(b).unwrap();
    list.check(a).unwrap();
    list.check(b).unwrap();
    list.check(c).unwrap();
    let actual = format_task(&list, b);
    let expected = task("b", -1, Complete);
    assert_eq!(actual, expected);
}

#[test]
fn format_incomplete_task_does_not_show_deps() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    list.block(b).on(a).unwrap();
    list.check(a).unwrap();
    let actual = format_task(&list, b);
    let expected = task("b", 1, Incomplete);
    assert_eq!(actual, expected);
}

#[test]
fn format_complete_task_with_punctuality_early() {
    let now = ymdhms(2022, 04, 13, 09, 00, 00);
    let mut list = TodoList::default();
    let a = list.add(
        NewOptions::new()
            .desc("a")
            .creation_time(now)
            .due_date(now + chrono::Duration::hours(2)),
    );
    list.check(CheckOptions { id: a, now }).unwrap();
    let actual = format_task(&list, a);
    let expected =
        task("a", 0, Complete).punctuality(-chrono::Duration::hours(2));
    assert_eq!(actual, expected);
}

#[test]
fn format_complete_task_with_punctuality_late() {
    let now = ymdhms(2022, 04, 13, 09, 00, 00);
    let mut list = TodoList::default();
    let a = list.add(
        NewOptions::new()
            .desc("a")
            .creation_time(now)
            .due_date(now - chrono::Duration::days(3)),
    );
    list.check(CheckOptions { id: a, now }).unwrap();
    let actual = format_task(&list, a);
    let expected =
        task("a", 0, Complete).punctuality(chrono::Duration::days(3));
    assert_eq!(actual, expected);
}

#[test]
fn format_tag() {
    let mut list = TodoList::default();
    let a = list.add(NewOptions::new().desc("a").as_tag());
    let actual = format_task(&list, a);
    let expected = task("a", 1, Incomplete).as_tag();
    assert_eq!(actual, expected);
}

#[test]
fn format_task_with_implicit_tags() {
    let mut list = TodoList::default();
    let a = list.add(NewOptions::new().desc("a").as_tag());
    let b = list.add(NewOptions::new().desc("b").as_tag());
    let c = list.add("c");
    list.block(a).on(c).unwrap();
    list.block(b).on(c).unwrap();
    let actual = format_task(&list, c);
    let expected = task("c", 1, Incomplete).adeps_stats(2, 2).tag("a").tag("b");
    assert_eq!(actual, expected);
}

#[test]
fn format_tag_with_implicit_tags() {
    let mut list = TodoList::default();
    let a = list.add(NewOptions::new().desc("a").as_tag());
    let b = list.add(NewOptions::new().desc("b").as_tag());
    let c = list.add(NewOptions::new().desc("c").as_tag());
    list.block(a).on(c).unwrap();
    list.block(b).on(c).unwrap();
    let actual = format_task(&list, c);
    let expected = task("c", 1, Incomplete)
        .adeps_stats(2, 2)
        .tag("a")
        .tag("b")
        .as_tag();
    assert_eq!(actual, expected);
}

#[test]
fn lookup_by_number() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    let lookup1 = lookup_tasks(&list, std::iter::once(&Key::ByNumber(1)));
    assert_eq!(lookup1.as_sorted_vec(&list), [a]);
    let lookup2 = lookup_tasks(&list, std::iter::once(&Key::ByNumber(2)));
    assert_eq!(lookup2.as_sorted_vec(&list), [b]);
    let lookup3 = lookup_tasks(&list, std::iter::once(&Key::ByNumber(3)));
    assert_eq!(lookup3.as_sorted_vec(&list), [c]);
}

#[test]
fn lookup_by_name() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    let lookup1 = lookup_tasks(&list, &[Key::ByName("a".to_string())]);
    assert_eq!(lookup1.as_sorted_vec(&list), [a]);
    let lookup2 = lookup_tasks(&list, &[Key::ByName("b".to_string())]);
    assert_eq!(lookup2.as_sorted_vec(&list), [b]);
    let lookup3 = lookup_tasks(&list, &[Key::ByName("c".to_string())]);
    assert_eq!(lookup3.as_sorted_vec(&list), [c]);
}

#[test]
fn lookup_multiple_keys() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    let lookup_all = lookup_tasks(
        &list,
        &[Key::ByNumber(1), Key::ByNumber(2), Key::ByNumber(3)],
    );
    assert_eq!(lookup_all.as_sorted_vec(&list), [a, b, c]);
}

#[test]
fn lookup_by_range() {
    let mut list = TodoList::default();
    let a = list.add("a");
    let b = list.add("b");
    let c = list.add("c");
    let lookup_1_2 = lookup_tasks(&list, &[Key::ByRange(1, 2)]);
    assert_eq!(lookup_1_2.as_sorted_vec(&list), [a, b]);
    let lookup_2_3 = lookup_tasks(&list, &[Key::ByRange(2, 3)]);
    assert_eq!(lookup_2_3.as_sorted_vec(&list), [b, c]);
    let lookup_1_3 = lookup_tasks(&list, &[Key::ByRange(1, 3)]);
    assert_eq!(lookup_1_3.as_sorted_vec(&list), [a, b, c]);
}
