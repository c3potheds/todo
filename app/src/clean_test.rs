#![allow(clippy::field_reassign_with_default)]

use crate::testing::task;
use crate::testing::Fixture;
use todo_printing::Plicit::*;
use todo_printing::Status::*;

#[test]
fn clean_tasks_out_of_order() {
    let mut fix = Fixture::default();
    fix.list = serde_json::from_str(
        r#"{
            "tasks": {
                "nodes": [
                    {
                        "desc": "a",
                        "priority": 0,
                        "implicit_priority": 0,
                        "creation_time": "2000-01-01T00:00:00Z",
                        "start_date": "2000-01-01T00:00:00Z"
                    },
                    {
                        "desc": "b",
                        "priority": 1,
                        "implicit_priority": 1,
                        "creation_time": "2000-01-01T00:00:00Z",
                        "start_date": "2000-01-01T00:00:00Z"
                    },
                    {
                        "desc": "c",
                        "priority": 2,
                        "implicit_priority": 2,
                        "creation_time": "2000-01-01T00:00:00Z",
                        "start_date": "2000-01-01T00:00:00Z"
                    }
                ],
                "edge_property": "directed",
                "edges": []
            },
            "next_id": 3,
            "complete": [],
            "incomplete": {
                "layers": [
                    [
                        0,
                        1,
                        2
                    ]
                ],
                "depth": {
                    "0": 0,
                    "1": 0,
                    "2": 0
                }
            }
        }"#,
    )
    .unwrap();
    fix.test("todo")
        .validate()
        .printed_task(&task("a", 1, Incomplete))
        .printed_task(&task("b", 2, Incomplete).priority(Explicit(1)))
        .printed_task(&task("c", 3, Incomplete).priority(Explicit(2)))
        .end();
    fix.test("todo clean")
        .modified(true)
        .validate()
        .printed_task(&task("c", 1, Incomplete).priority(Explicit(2)))
        .printed_task(&task("b", 2, Incomplete).priority(Explicit(1)))
        .printed_task(&task("a", 3, Incomplete))
        .end();
}

#[test]
fn clean_tasks_in_wrong_layers() {
    let mut fix = Fixture::default();
    fix.list = serde_json::from_str(
        r#"{
            "tasks": {
                "nodes": [
                    {
                        "desc": "a",
                        "priority": 0,
                        "implicit_priority": 0,
                        "creation_time": "2000-01-01T00:00:00Z",
                        "start_date": "2000-01-01T00:00:00Z"
                    },
                    {
                        "desc": "b",
                        "priority": 1,
                        "implicit_priority": 1,
                        "creation_time": "2000-01-01T00:00:00Z",
                        "start_date": "2000-01-01T00:00:00Z"
                    },
                    {
                        "desc": "c",
                        "priority": 2,
                        "implicit_priority": 2,
                        "creation_time": "2000-01-01T00:00:00Z",
                        "start_date": "2000-01-01T00:00:00Z"
                    }
                ],
                "edge_property": "directed",
                "edges": []
            },
            "next_id": 3,
            "complete": [],
            "incomplete": {
                "layers": [
                    [
                        0,
                        1
                    ],
                    [
                        2
                    ]
                ],
                "depth": {
                    "0": 0,
                    "1": 0,
                    "2": 1
                }
            }
        }"#,
    )
    .unwrap();
    fix.test("todo")
        .validate()
        .printed_task(&task("a", 1, Incomplete))
        .printed_task(&task("b", 2, Incomplete).priority(Explicit(1)))
        .end();
    fix.test("todo clean")
        .modified(true)
        .validate()
        .printed_task(&task("c", 1, Incomplete).priority(Explicit(2)))
        .printed_task(&task("b", 2, Incomplete).priority(Explicit(1)))
        .printed_task(&task("a", 3, Incomplete))
        .end();
}
