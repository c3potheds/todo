use cli::Key;
use model::TaskStatus;
use printing::*;

fn make_printing_context() -> PrintingContext {
    PrintingContext {
        max_index_digits: 3,
        width: 80,
    }
}

fn print_task_with_context(
    context: &PrintingContext,
    task: &PrintableTask,
) -> String {
    let mut out: Vec<u8> = Vec::new();
    let mut printer = SimpleTodoPrinter {
        out: &mut out,
        context: context,
    };
    printer.print_task(task);
    String::from(std::str::from_utf8(&out).unwrap())
}

fn print_task<'a>(task: &PrintableTask) -> String {
    let context = make_printing_context();
    print_task_with_context(&context, task)
}

#[test]
fn fmt_incomplete_task() {
    let fmt = print_task(&PrintableTask {
        desc: "a",
        number: 1,
        status: TaskStatus::Incomplete,
        action: Action::None,
    });
    // The 1) is wrapped in ANSI codes painting it yellow.
    assert_eq!(fmt, "      \u{1b}[33m1)\u{1b}[0m a\n");
}

#[test]
fn fmt_complete_task() {
    let fmt = print_task(&PrintableTask {
        desc: "b",
        number: 0,
        status: TaskStatus::Complete,
        action: Action::None,
    });
    // The 0) is wrapped in ANSI codes painting it green.
    assert_eq!(fmt, "      \u{1b}[32m0)\u{1b}[0m b\n");
}

#[test]
fn fmt_blocked_task() {
    let fmt = print_task(&PrintableTask {
        desc: "c",
        number: 2,
        status: TaskStatus::Blocked,
        action: Action::None,
    });
    // The 2) is wrapped in ANSI codes painting it red.
    assert_eq!(fmt, "      \u{1b}[31m2)\u{1b}[0m c\n");
}

#[test]
fn fmt_double_digit_number_in_max_four_digit_environment() {
    let fmt = print_task_with_context(
        &PrintingContext {
            max_index_digits: 4,
            width: 80,
        },
        &PrintableTask {
            desc: "hello",
            number: 99,
            status: TaskStatus::Blocked,
            action: Action::None,
        },
    );
    assert_eq!(fmt, "      \u{1b}[31m99)\u{1b}[0m hello\n");
}

#[test]
fn fmt_triple_digit_number_in_max_four_digit_environment() {
    let fmt = print_task_with_context(
        &PrintingContext {
            max_index_digits: 4,
            width: 80,
        },
        &PrintableTask {
            desc: "hello",
            number: 100,
            status: TaskStatus::Blocked,
            action: Action::None,
        },
    );
    assert_eq!(fmt, "     \u{1b}[31m100)\u{1b}[0m hello\n");
}

#[test]
fn show_check_mark_on_check_action() {
    let fmt = print_task(&PrintableTask {
        desc: "done!",
        number: 0,
        status: TaskStatus::Complete,
        action: Action::Check,
    });
    assert_eq!(
        fmt,
        "\u{1b}[32m[✓]\u{1b}[0m   \u{1b}[32m0)\u{1b}[0m done!\n"
    );
}

#[test]
fn show_empty_box_on_uncheck_action() {
    let fmt = print_task(&PrintableTask {
        desc: "oh",
        number: 1,
        status: TaskStatus::Incomplete,
        action: Action::Uncheck,
    });
    assert_eq!(fmt, "\u{1b}[33m[ ]\u{1b}[0m   \u{1b}[33m1)\u{1b}[0m oh\n");
}

#[test]
fn text_wrapping() {
    let context = PrintingContext {
        max_index_digits: 3,
        width: 24,
    };
    let fmt = print_task_with_context(
        &context,
        &PrintableTask {
            desc: "this task has a long description, much longer than 24 chars",
            number: 1,
            status: TaskStatus::Incomplete,
            action: Action::None,
        },
    );
    assert_eq!(
        fmt,
        "      \u{1b}[33m1)\u{1b}[0m this task\n         \
                                     has a long\n         \
                                     description,\n         \
                                     much longer\n         \
                                     than 24 chars\n"
    );
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
fn display_no_match_found_for_name_warning() {
    let fmt = format!(
        "{}",
        PrintableWarning::NoMatchFoundForKey {
            requested_key: Key::ByName("blah".to_string()),
        }
    );
    assert_eq!(
        fmt,
        "\u{1b}[33mwarning\u{1b}[0m: No match found for \"blah\""
    );
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
    let fmt = print_task(&PrintableTask {
        desc: "blocked",
        number: 5,
        status: TaskStatus::Blocked,
        action: Action::Lock,
    });
    assert_eq!(
        fmt,
        " \u{1b}[31m🔒\u{1b}[0m   \u{1b}[31m5)\u{1b}[0m blocked\n"
    );
}

#[test]
fn show_unlock_icon_on_unlock_action() {
    let fmt = print_task(&PrintableTask {
        desc: "unblocked",
        number: 10,
        status: TaskStatus::Incomplete,
        action: Action::Unlock,
    });
    assert_eq!(
        fmt,
        " \u{1b}[32m🔓\u{1b}[0m  \u{1b}[33m10)\u{1b}[0m unblocked\n"
    );
}

#[test]
fn show_punt_icon_on_punt_action() {
    let fmt = print_task(&PrintableTask {
        desc: "punt this",
        number: 5,
        status: TaskStatus::Incomplete,
        action: Action::Punt,
    });
    assert_eq!(fmt, " ⏎    \u{1b}[33m5)\u{1b}[0m punt this\n");
}

#[test]
fn validate_single_task() {
    let mut printer = FakePrinter::new();
    printer.print_task(&PrintableTask {
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
    printer.print_task(&PrintableTask {
        desc: "a",
        number: 1,
        status: TaskStatus::Incomplete,
        action: Action::None,
    });
    printer.print_task(&PrintableTask {
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
fn validate_warning() {
    let mut printer = FakePrinter::new();
    let warning = PrintableWarning::NoMatchFoundForKey {
        requested_key: Key::ByName("a".to_string()),
    };
    printer.print_warning(&warning);
    printer.validate().printed_warning(&warning).end();
}

#[test]
fn validate_error() {
    let mut printer = FakePrinter::new();
    let error = PrintableError::CannotBlockBecauseWouldCauseCycle {
        cannot_block: 1,
        requested_dependency: 1,
    };
    printer.print_error(&error);
    printer.validate().printed_error(&error).end();
}
#[test]
#[should_panic(expected = "Missing item")]
fn fail_validation_on_missing_task() {
    let mut printer = FakePrinter::new();
    printer.validate().printed_task(&[Expect::Desc("a")]).end();
}

#[test]
#[should_panic(expected = "Unexpected description: \"a\"")]
fn fail_validation_on_incorrect_description() {
    let mut printer = FakePrinter::new();
    printer.print_task(&PrintableTask {
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
    printer.print_task(&PrintableTask {
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
    printer.print_task(&PrintableTask {
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
    printer.print_task(&PrintableTask {
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
    printer.print_task(&PrintableTask {
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

#[test]
#[should_panic(expected = "Unexpected warning")]
fn fail_validation_on_wrong_warning() {
    let mut printer = FakePrinter::new();
    let warning1 = PrintableWarning::NoMatchFoundForKey {
        requested_key: Key::ByNumber(1),
    };
    let warning2 = PrintableWarning::NoMatchFoundForKey {
        requested_key: Key::ByNumber(2),
    };
    printer.print_warning(&warning1);
    printer.validate().printed_warning(&warning2).end();
}

#[test]
#[should_panic(expected = "Expected")]
fn fail_validation_on_task_instead_of_warning() {
    let mut printer = FakePrinter::new();
    let warning =
        PrintableWarning::CannotPuntBecauseComplete { cannot_punt: 0 };
    printer.print_task(&PrintableTask {
        desc: "a",
        number: 1,
        status: TaskStatus::Incomplete,
        action: Action::None,
    });
    printer.validate().printed_warning(&warning).end();
}

#[test]
#[should_panic(expected = "Unexpected error")]
fn fail_validation_on_wrong_error() {
    let mut printer = FakePrinter::new();
    let error1 = PrintableError::CannotCheckBecauseBlocked {
        cannot_check: 3,
        blocked_by: vec![2],
    };
    let error2 = PrintableError::CannotCheckBecauseBlocked {
        cannot_check: 2,
        blocked_by: vec![1],
    };
    printer.print_error(&error1);
    printer.validate().printed_error(&error2).end();
}

#[test]
#[should_panic(expected = "Expected")]
fn fail_validation_on_task_instead_of_error() {
    let mut printer = FakePrinter::new();
    let error = PrintableError::CannotCheckBecauseBlocked {
        cannot_check: 3,
        blocked_by: vec![2],
    };
    printer.print_task(&PrintableTask {
        desc: "a",
        number: 1,
        status: TaskStatus::Incomplete,
        action: Action::None,
    });
    printer.validate().printed_error(&error).end();
}
