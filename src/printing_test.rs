use cli::Key;
use model::TaskStatus;
use printing::*;

fn make_printing_context() -> PrintingContext {
    PrintingContext {
        max_index_digits: 3,
        width: 80,
    }
}

#[test]
fn fmt_incomplete_task() {
    let context = make_printing_context();
    let fmt = format!(
        "{}",
        PrintableTask {
            context: &context,
            desc: "a",
            number: 1,
            status: TaskStatus::Incomplete,
            action: Action::None,
        }
    );
    // The 1) is wrapped in ANSI codes painting it yellow.
    assert_eq!(fmt, "      \u{1b}[33m1)\u{1b}[0m a");
}

#[test]
fn fmt_complete_task() {
    let context = make_printing_context();
    let fmt = format!(
        "{}",
        PrintableTask {
            context: &context,
            desc: "b",
            number: 0,
            status: TaskStatus::Complete,
            action: Action::None,
        }
    );
    // The 0) is wrapped in ANSI codes painting it green.
    assert_eq!(fmt, "      \u{1b}[32m0)\u{1b}[0m b");
}

#[test]
fn fmt_blocked_task() {
    let context = make_printing_context();
    let fmt = format!(
        "{}",
        PrintableTask {
            context: &context,
            desc: "c",
            number: 2,
            status: TaskStatus::Blocked,
            action: Action::None
        }
    );
    // The 2) is wrapped in ANSI codes painting it red.
    assert_eq!(fmt, "      \u{1b}[31m2)\u{1b}[0m c");
}

#[test]
fn fmt_double_digit_number_in_max_four_digit_environment() {
    let context = PrintingContext {
        max_index_digits: 4,
        width: 80,
    };
    let fmt = format!(
        "{}",
        PrintableTask {
            context: &context,
            desc: "hello",
            number: 99,
            status: TaskStatus::Blocked,
            action: Action::None,
        }
    );
    assert_eq!(fmt, "      \u{1b}[31m99)\u{1b}[0m hello");
}

#[test]
fn fmt_triple_digit_number_in_max_four_digit_environment() {
    let context = PrintingContext {
        max_index_digits: 4,
        width: 80,
    };
    let fmt = format!(
        "{}",
        PrintableTask {
            context: &context,
            desc: "hello",
            number: 100,
            status: TaskStatus::Blocked,
            action: Action::None,
        }
    );
    assert_eq!(fmt, "     \u{1b}[31m100)\u{1b}[0m hello");
}

#[test]
fn show_check_mark_on_check_action() {
    let context = make_printing_context();
    let fmt = format!(
        "{}",
        PrintableTask {
            context: &context,
            desc: "done!",
            number: 0,
            status: TaskStatus::Complete,
            action: Action::Check,
        }
    );
    assert_eq!(fmt, "\u{1b}[32m[✓]\u{1b}[0m   \u{1b}[32m0)\u{1b}[0m done!");
}

#[test]
fn show_empty_box_on_uncheck_action() {
    let context = make_printing_context();
    let fmt = format!(
        "{}",
        PrintableTask {
            context: &context,
            desc: "oh",
            number: 1,
            status: TaskStatus::Incomplete,
            action: Action::Uncheck,
        }
    );
    assert_eq!(fmt, "\u{1b}[33m[ ]\u{1b}[0m   \u{1b}[33m1)\u{1b}[0m oh");
}

#[test]
fn display_no_match_found_warning() {
    let fmt = format!(
        "{}",
        PrintableWarning::NoMatchFoundForKey {
            requested_key: Key::ByNumber(10),
        },
    );
    assert_eq!(fmt, "\u{1b}[33mwarning\u{1b}[0m: No match found for \"10\"");
}

#[test]
fn display_cannot_check_because_already_complete_warning() {
    let fmt = format!(
        "{}",
        PrintableWarning::CannotCheckBecauseAlreadyComplete {
            cannot_check: -2
        },
    );
    assert_eq!(
        fmt,
        concat!(
            "\u{1b}[33mwarning\u{1b}[0m: ",
            "Task \u{1b}[32m-2)\u{1b}[0m is already complete"
        )
    );
}

#[test]
fn display_cannot_restore_because_already_incomplete_warning() {
    let fmt = format!(
        "{}",
        PrintableWarning::CannotRestoreBecauseAlreadyIncomplete {
            cannot_restore: 3
        },
    );
    assert_eq!(
        fmt,
        concat!(
            "\u{1b}[33mwarning\u{1b}[0m: ",
            "Task \u{1b}[33m3)\u{1b}[0m is already incomplete"
        )
    );
}

#[test]
fn display_cannot_unblock_because_task_is_not_blocked_warning() {
    let fmt = format!(
        "{}",
        PrintableWarning::CannotUnblockBecauseTaskIsNotBlocked {
            cannot_unblock: 2,
            requested_unblock_from: 1,
        },
    );
    assert_eq!(
        fmt,
        concat!(
            "\u{1b}[33mwarning\u{1b}[0m: ",
            "Task \u{1b}[33m2)\u{1b}[0m is not blocked by ",
            "\u{1b}[33m1)\u{1b}[0m"
        )
    );
}

#[test]
fn display_cannot_check_because_blocked_error() {
    let fmt = format!(
        "{}",
        PrintableError::CannotCheckBecauseBlocked {
            cannot_check: 3,
            blocked_by: vec![1, 2],
        },
    );
    assert_eq!(
        fmt,
        concat!(
            "\u{1b}[31merror\u{1b}[0m: ",
            "Cannot complete \u{1b}[31m3)\u{1b}[0m ",
            "because it is blocked by ",
            "\u{1b}[33m1)\u{1b}[0m, \u{1b}[33m2)\u{1b}[0m"
        )
    );
}

#[test]
fn display_cannot_restore_because_antidependency_is_complete_error() {
    let fmt = format!(
        "{}",
        PrintableError::CannotRestoreBecauseAntidependencyIsComplete {
            cannot_restore: -3,
            complete_antidependencies: vec![-1, 0],
        },
    );
    assert_eq!(
        fmt,
        concat!(
            "\u{1b}[31merror\u{1b}[0m: ",
            "Cannot restore \u{1b}[32m-3)\u{1b}[0m ",
            "because it blocks complete tasks ",
            "\u{1b}[32m-1)\u{1b}[0m, \u{1b}[32m0)\u{1b}[0m"
        )
    );
}

#[test]
fn display_cannot_block_because_would_cause_cycle_error() {
    let fmt = format!(
        "{}",
        PrintableError::CannotBlockBecauseWouldCauseCycle {
            cannot_block: 5,
            requested_dependency: 6,
        },
    );
    assert_eq!(
        fmt,
        concat!(
            "\u{1b}[31merror\u{1b}[0m: ",
            "Cannot block \u{1b}[33m5)\u{1b}[0m ",
            "on \u{1b}[31m6)\u{1b}[0m ",
            "because it would create a cycle"
        )
    );
}

#[test]
fn show_lock_icon_on_lock_action() {
    let context = make_printing_context();
    let fmt = format!(
        "{}",
        PrintableTask {
            context: &context,
            desc: "blocked",
            number: 5,
            status: TaskStatus::Blocked,
            action: Action::Lock,
        }
    );
    assert_eq!(
        fmt,
        " \u{1b}[31m🔒\u{1b}[0m   \u{1b}[31m5)\u{1b}[0m blocked"
    );
}

#[test]
fn show_unlock_icon_on_unlock_action() {
    let context = make_printing_context();
    let fmt = format!(
        "{}",
        PrintableTask {
            context: &context,
            desc: "unblocked",
            number: 10,
            status: TaskStatus::Incomplete,
            action: Action::Unlock,
        }
    );
    assert_eq!(
        fmt,
        " \u{1b}[32m🔓\u{1b}[0m  \u{1b}[33m10)\u{1b}[0m unblocked"
    );
}

#[test]
fn validate_single_task() {
    let mut printer = FakePrinter::new();
    let context = make_printing_context();
    printer.print_task(&PrintableTask {
        context: &context,
        desc: "a",
        number: 1,
        status: TaskStatus::Incomplete,
        action: Action::None,
    });
    printer
        .validate()
        .printed_task(&[Expect::Desc("a"), Expect::Number(1)])
        .end();
}

#[test]
fn validate_multiple_tasks() {
    let mut printer = FakePrinter::new();
    let context = make_printing_context();
    printer.print_task(&PrintableTask {
        context: &context,
        desc: "a",
        number: 1,
        status: TaskStatus::Incomplete,
        action: Action::None,
    });
    printer.print_task(&PrintableTask {
        context: &context,
        desc: "b",
        number: 2,
        status: TaskStatus::Incomplete,
        action: Action::None,
    });
    printer
        .validate()
        .printed_task(&[Expect::Desc("a"), Expect::Number(1)])
        .printed_task(&[Expect::Desc("b"), Expect::Number(2)])
        .end();
}

#[test]
#[should_panic(expected = "Missing task")]
fn fail_validation_on_missing_task() {
    let mut printer = FakePrinter::new();
    printer.validate().printed_task(&[Expect::Desc("a")]).end();
}

#[test]
#[should_panic(expected = "Unexpected description: \"a\"")]
fn fail_validation_on_incorrect_description() {
    let mut printer = FakePrinter::new();
    let context = make_printing_context();
    printer.print_task(&PrintableTask {
        context: &context,
        desc: "a",
        number: 1,
        status: TaskStatus::Incomplete,
        action: Action::None,
    });
    printer.validate().printed_task(&[Expect::Desc("b")]).end();
}

#[test]
#[should_panic(expected = "Unexpected number: 1")]
fn fail_validation_on_incorrect_number() {
    let mut printer = FakePrinter::new();
    let context = make_printing_context();
    printer.print_task(&PrintableTask {
        context: &context,
        desc: "a",
        number: 1,
        status: TaskStatus::Incomplete,
        action: Action::None,
    });
    printer.validate().printed_task(&[Expect::Number(2)]).end();
}

#[test]
#[should_panic(expected = "Extra tasks were recorded: ")]
fn fail_validation_on_extra_tasks() {
    let mut printer = FakePrinter::new();
    let context = make_printing_context();
    printer.print_task(&PrintableTask {
        context: &context,
        desc: "a",
        number: 1,
        status: TaskStatus::Incomplete,
        action: Action::None,
    });
    printer.validate().end();
}

#[test]
#[should_panic(expected = "Unexpected status")]
fn fail_validation_on_incorrect_status() {
    let mut printer = FakePrinter::new();
    let context = make_printing_context();
    printer.print_task(&PrintableTask {
        context: &context,
        desc: "a",
        number: 0,
        status: TaskStatus::Incomplete,
        action: Action::None,
    });
    printer
        .validate()
        .printed_task(&[Expect::Status(TaskStatus::Complete)])
        .end();
}

#[test]
#[should_panic(expected = "Unexpected action")]
fn fail_validation_on_incorrect_action() {
    let mut printer = FakePrinter::new();
    let context = make_printing_context();
    printer.print_task(&PrintableTask {
        context: &context,
        desc: "a",
        number: 1,
        status: TaskStatus::Incomplete,
        action: Action::New,
    });
    printer
        .validate()
        .printed_task(&[Expect::Action(Action::None)])
        .end();
}
