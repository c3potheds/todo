use app::testing::ymdhms;
use chrono::DateTime;
use chrono::Utc;
use cli::Key;
use printing::Status::*;
use printing::*;

fn make_printing_context() -> PrintingContext {
    PrintingContext {
        max_index_digits: 3,
        width: 80,
        now: Utc::now(),
    }
}

fn now_context(now: DateTime<Utc>) -> PrintingContext {
    PrintingContext {
        max_index_digits: 3,
        width: 80,
        now: now,
    }
}

fn print_task_with_context(
    context: PrintingContext,
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
    print_task_with_context(context, task)
}

#[test]
fn fmt_incomplete_task() {
    let fmt = print_task(&PrintableTask::new("a", 1, Incomplete));
    // The 1) is wrapped in ANSI codes painting it yellow.
    assert_eq!(fmt, "      \u{1b}[33m1)\u{1b}[0m a\n");
}

#[test]
fn fmt_complete_task() {
    let fmt = print_task(&PrintableTask::new("b", 0, Complete));
    // The 0) is wrapped in ANSI codes painting it green.
    assert_eq!(fmt, "      \u{1b}[32m0)\u{1b}[0m b\n");
}

#[test]
fn fmt_blocked_task() {
    let fmt = print_task(&PrintableTask::new("c", 2, Blocked));
    // The 2) is wrapped in ANSI codes painting it red.
    assert_eq!(fmt, "      \u{1b}[31m2)\u{1b}[0m c\n");
}

#[test]
fn fmt_double_digit_number_in_max_four_digit_environment() {
    let fmt = print_task_with_context(
        PrintingContext {
            max_index_digits: 4,
            width: 80,
            now: Utc::now(),
        },
        &PrintableTask::new("hello", 99, Blocked),
    );
    assert_eq!(fmt, "      \u{1b}[31m99)\u{1b}[0m hello\n");
}

#[test]
fn fmt_triple_digit_number_in_max_four_digit_environment() {
    let fmt = print_task_with_context(
        PrintingContext {
            max_index_digits: 4,
            width: 80,
            now: Utc::now(),
        },
        &PrintableTask::new("hello", 100, Blocked),
    );
    assert_eq!(fmt, "     \u{1b}[31m100)\u{1b}[0m hello\n");
}

#[test]
fn show_check_mark_on_check_action() {
    let fmt = print_task(
        &PrintableTask::new("done!", 0, Complete).action(Action::Check),
    );
    assert_eq!(
        fmt,
        "\u{1b}[32m[✓]\u{1b}[0m   \u{1b}[32m0)\u{1b}[0m done!\n"
    );
}

#[test]
fn show_empty_box_on_uncheck_action() {
    let fmt = print_task(
        &PrintableTask::new("oh", 1, Incomplete).action(Action::Uncheck),
    );
    assert_eq!(fmt, "\u{1b}[33m[ ]\u{1b}[0m   \u{1b}[33m1)\u{1b}[0m oh\n");
}

#[test]
fn text_wrapping() {
    let context = PrintingContext {
        max_index_digits: 3,
        width: 24,
        now: Utc::now(),
    };
    let fmt = print_task_with_context(
        context,
        &PrintableTask::new(
            "this task has a long description, much longer than 24 chars",
            1,
            Incomplete,
        ),
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
fn text_wrapping_with_log_date() {
    let context = PrintingContext {
        max_index_digits: 3,
        width: 34,
        now: Utc::now(),
    };
    let fmt = print_task_with_context(
        context,
        &PrintableTask::new(
            "what a long description, it needs multiple lines",
            0,
            Complete,
        )
        .log_date(LogDate::YearMonthDay(2020, 03, 15)),
    );
    assert_eq!(
        fmt,
        concat!(
            "2020-03-15       \u{1b}[32m0)\u{1b}[0m what a long\n",
            "                    description,\n",
            "                    it needs\n",
            "                    multiple lines\n"
        )
    );
}

#[test]
fn visible_log_date() {
    let fmt = print_task(
        &PrintableTask::new(
            "yeah babi babi babi babi babi babi babi babiru",
            0,
            Complete,
        )
        .log_date(LogDate::ymd(2021, 02, 28).unwrap()),
    );
    assert_eq!(
        fmt,
        concat!(
            "2021-02-28       \u{1b}[32m0)\u{1b}[0m ",
            "yeah babi babi babi babi babi babi babi babiru\n"
        )
    );
}

#[test]
fn invisible_log_date() {
    let fmt = print_task(
        &PrintableTask::new(
            "yeah babi babi babi babi babi babi babi babiru",
            0,
            Complete,
        )
        .log_date(LogDate::Invisible),
    );
    assert_eq!(
        fmt,
        concat!(
            "                 \u{1b}[32m0)\u{1b}[0m ",
            "yeah babi babi babi babi babi babi babi babiru\n"
        )
    );
}

#[test]
fn show_priority_on_task() {
    let fmt = print_task(&PrintableTask::new("a", 1, Incomplete).priority(1));
    assert_eq!(
        fmt,
        "      \u{1b}[33m1)\u{1b}[0m \u{1b}[1;35mP1\u{1b}[0m a\n"
    );
}

#[test]
fn show_meh_due_date_on_task() {
    let now = ymdhms(2021, 04, 15, 10, 00, 00);
    let task = PrintableTask::new("a", 1, Incomplete)
        .due_date(now + chrono::Duration::days(2));
    let fmt = print_task_with_context(now_context(now), &task);
    assert_eq!(
        fmt,
        "      \u{1b}[33m1)\u{1b}[0m \u{1b}[1;2;37mDue in 2 days\u{1b}[0m a\n"
    );
}

#[test]
fn show_moderate_due_date_on_task() {
    let now = ymdhms(2021, 04, 15, 10, 00, 00);
    let task = PrintableTask::new("a", 1, Incomplete)
        .due_date(now + chrono::Duration::hours(9));
    let fmt = print_task_with_context(now_context(now), &task);
    assert_eq!(
        fmt,
        "      \u{1b}[33m1)\u{1b}[0m \u{1b}[1;33mDue in 9 hours\u{1b}[0m a\n"
    );
}

#[test]
fn show_urgent_due_date_on_task() {
    let now = ymdhms(2021, 04, 15, 10, 00, 00);
    let task = PrintableTask::new("a", 1, Incomplete)
        .due_date(now - chrono::Duration::days(1));
    let fmt = print_task_with_context(now_context(now), &task);
    assert_eq!(
        fmt,
        "      \u{1b}[33m1)\u{1b}[0m \u{1b}[1;31mDue 1 day ago\u{1b}[0m a\n"
    );
}

#[test]
fn show_priority_and_due_date_together() {
    let now = ymdhms(2021, 04, 15, 10, 00, 00);
    let task = PrintableTask::new("a", 1, Incomplete)
        .priority(1)
        .due_date(now - chrono::Duration::days(1));
    let fmt = print_task_with_context(now_context(now), &task);
    assert_eq!(
        fmt,
        concat!(
            "      \u{1b}[33m1)\u{1b}[0m ",
            "\u{1b}[1;35mP1\u{1b}[0m ",
            "\u{1b}[1;31mDue 1 day ago\u{1b}[0m ",
            "a\n"
        ),
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
    assert_eq!(
        fmt,
        "\u{1b}[1;33mwarning\u{1b}[0m: No match found for \"10\""
    );
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
        "\u{1b}[1;33mwarning\u{1b}[0m: No match found for \"blah\""
    );
}

#[test]
fn display_no_match_found_for_range_warning() {
    let fmt = format!(
        "{}",
        PrintableWarning::NoMatchFoundForKey {
            requested_key: Key::ByRange(10, 20),
        }
    );
    assert_eq!(
        fmt,
        "\u{1b}[1;33mwarning\u{1b}[0m: No match found for range(10..20)"
    );
}

#[test]
fn display_cannot_check_because_already_complete_warning() {
    let fmt = format!(
        "{}",
        PrintableWarning::CannotCheckBecauseAlreadyComplete {
            cannot_check: BriefPrintableTask::new(-2, Complete)
        },
    );
    assert_eq!(
        fmt,
        concat!(
            "\u{1b}[1;33mwarning\u{1b}[0m: ",
            "Task \u{1b}[32m-2)\u{1b}[0m is already complete"
        )
    );
}

#[test]
fn display_cannot_restore_because_already_incomplete_warning() {
    let fmt = format!(
        "{}",
        PrintableWarning::CannotRestoreBecauseAlreadyIncomplete {
            cannot_restore: BriefPrintableTask::new(3, Incomplete),
        },
    );
    assert_eq!(
        fmt,
        concat!(
            "\u{1b}[1;33mwarning\u{1b}[0m: ",
            "Task \u{1b}[33m3)\u{1b}[0m is already incomplete"
        )
    );
}

#[test]
fn display_cannot_unblock_because_task_is_not_blocked_warning() {
    let fmt = format!(
        "{}",
        PrintableWarning::CannotUnblockBecauseTaskIsNotBlocked {
            cannot_unblock: BriefPrintableTask::new(2, Incomplete),
            requested_unblock_from: BriefPrintableTask::new(1, Incomplete),
        },
    );
    assert_eq!(
        fmt,
        concat!(
            "\u{1b}[1;33mwarning\u{1b}[0m: ",
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
            cannot_check: BriefPrintableTask::new(3, Blocked),
            blocked_by: vec![
                BriefPrintableTask::new(1, Incomplete),
                BriefPrintableTask::new(2, Incomplete)
            ],
        },
    );
    assert_eq!(
        fmt,
        concat!(
            "\u{1b}[1;31merror\u{1b}[0m: ",
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
            cannot_restore: BriefPrintableTask::new(-3, Complete),
            complete_antidependencies: vec![
                BriefPrintableTask::new(-1, Complete),
                BriefPrintableTask::new(0, Complete)
            ],
        },
    );
    assert_eq!(
        fmt,
        concat!(
            "\u{1b}[1;31merror\u{1b}[0m: ",
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
            cannot_block: BriefPrintableTask::new(5, Incomplete),
            requested_dependency: BriefPrintableTask::new(6, Blocked),
        },
    );
    assert_eq!(
        fmt,
        concat!(
            "\u{1b}[1;31merror\u{1b}[0m: ",
            "Cannot block \u{1b}[33m5)\u{1b}[0m ",
            "on \u{1b}[31m6)\u{1b}[0m ",
            "because it would create a cycle"
        )
    );
}

#[test]
fn display_cannot_edit_because_unexpected_number_error() {
    let fmt = format!(
        "{}",
        PrintableError::CannotEditBecauseUnexpectedNumber { requested: 0 }
    );
    assert_eq!(
        fmt,
        concat!(
            "\u{1b}[1;31merror\u{1b}[0m: ",
            "Number 0) doesn't correspond to any of requested tasks"
        )
    );
}

#[test]
fn display_cannot_edit_because_no_task_with_number_error() {
    let fmt = format!(
        "{}",
        PrintableError::CannotEditBecauseNoTaskWithNumber { requested: 100 }
    );
    assert_eq!(fmt, "\u{1b}[1;31merror\u{1b}[0m: No task with number 100)");
}

#[test]
fn display_failed_to_use_text_editor_error() {
    let fmt = format!("{}", PrintableError::FailedToUseTextEditor);
    assert_eq!(
        fmt,
        "\u{1b}[1;31merror\u{1b}[0m: Failed to open text editor"
    );
}

#[test]
fn show_lock_icon_on_lock_action() {
    let fmt = print_task(
        &PrintableTask::new("blocked", 5, Blocked).action(Action::Lock),
    );
    assert_eq!(
        fmt,
        " \u{1b}[31m🔒\u{1b}[0m   \u{1b}[31m5)\u{1b}[0m blocked\n"
    );
}

#[test]
fn show_unlock_icon_on_unlock_action() {
    let fmt = print_task(
        &PrintableTask::new("unblocked", 10, Incomplete).action(Action::Unlock),
    );
    assert_eq!(
        fmt,
        " \u{1b}[32m🔓\u{1b}[0m  \u{1b}[33m10)\u{1b}[0m unblocked\n"
    );
}

#[test]
fn show_punt_icon_on_punt_action() {
    let fmt = print_task(
        &PrintableTask::new("punt this", 5, Incomplete).action(Action::Punt),
    );
    assert_eq!(fmt, " ⏎    \u{1b}[33m5)\u{1b}[0m punt this\n");
}

#[test]
fn validate_task() {
    let mut printer = FakePrinter::new();
    printer.print_task(&PrintableTask::new("a", 1, Incomplete));
    printer
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .end();
}

#[test]
fn validate_multiple_tasks() {
    let mut printer = FakePrinter::new();
    printer.print_task(&PrintableTask::new("a", 1, Incomplete));
    printer.print_task(&PrintableTask::new("b", 2, Incomplete));
    printer.print_task(&PrintableTask::new("c", 3, Incomplete));
    printer
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .printed_task(&PrintableTask::new("b", 2, Incomplete))
        .printed_task(&PrintableTask::new("c", 3, Incomplete))
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
        cannot_block: BriefPrintableTask::new(1, Incomplete),
        requested_dependency: BriefPrintableTask::new(1, Incomplete),
    };
    printer.print_error(&error);
    printer.validate().printed_error(&error).end();
}

#[test]
#[should_panic(expected = "Missing item")]
fn fail_validation_on_missing_task_exact() {
    let mut printer = FakePrinter::new();
    printer
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .end();
}

#[test]
#[should_panic(expected = "Unexpected description: \"a\"")]
fn fail_validation_on_incorrect_description_exact() {
    let mut printer = FakePrinter::new();
    printer.print_task(&PrintableTask::new("a", 1, Incomplete));
    printer
        .validate()
        .printed_task(&PrintableTask::new("b", 1, Incomplete))
        .end();
}

#[test]
#[should_panic(expected = "Unexpected number: 1")]
fn fail_validation_on_incorrect_number_exact() {
    let mut printer = FakePrinter::new();
    printer.print_task(&PrintableTask::new("a", 1, Incomplete));
    printer
        .validate()
        .printed_task(&PrintableTask::new("a", 2, Incomplete))
        .end();
}

#[test]
#[should_panic(expected = "Extra tasks were recorded: ")]
fn fail_validation_on_extra_tasks() {
    let mut printer = FakePrinter::new();
    printer.print_task(&PrintableTask::new("a", 1, Incomplete));
    printer.validate().end();
}

#[test]
#[should_panic(expected = "Unexpected status")]
fn fail_validation_on_incorrect_status_exact() {
    let mut printer = FakePrinter::new();
    printer.print_task(&PrintableTask::new("a", 1, Incomplete));
    printer
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Blocked))
        .end();
}

#[test]
#[should_panic(expected = "Unexpected action")]
fn fail_validation_on_incorrect_action_exact() {
    let mut printer = FakePrinter::new();
    printer.print_task(
        &PrintableTask::new("a", 1, Incomplete).action(Action::New),
    );
    printer
        .validate()
        .printed_task(
            &PrintableTask::new("a", 1, Incomplete).action(Action::Select),
        )
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
    let warning = PrintableWarning::CannotPuntBecauseComplete {
        cannot_punt: BriefPrintableTask::new(0, Incomplete),
    };
    printer.print_task(&PrintableTask::new("a", 1, Incomplete));
    printer.validate().printed_warning(&warning).end();
}

#[test]
#[should_panic(expected = "Unexpected error")]
fn fail_validation_on_wrong_error() {
    let mut printer = FakePrinter::new();
    let error1 = PrintableError::CannotCheckBecauseBlocked {
        cannot_check: BriefPrintableTask::new(3, Blocked),
        blocked_by: vec![BriefPrintableTask::new(2, Incomplete)],
    };
    let error2 = PrintableError::CannotCheckBecauseBlocked {
        cannot_check: BriefPrintableTask::new(2, Blocked),
        blocked_by: vec![BriefPrintableTask::new(1, Incomplete)],
    };
    printer.print_error(&error1);
    printer.validate().printed_error(&error2).end();
}

#[test]
#[should_panic(expected = "Expected")]
fn fail_validation_on_task_instead_of_error() {
    let mut printer = FakePrinter::new();
    let error = PrintableError::CannotCheckBecauseBlocked {
        cannot_check: BriefPrintableTask::new(3, Blocked),
        blocked_by: vec![BriefPrintableTask::new(2, Incomplete)],
    };
    printer.print_task(&PrintableTask::new("a", 1, Incomplete));
    printer.validate().printed_error(&error).end();
}

#[test]
#[should_panic(expected = "Missing required log date")]
fn fail_validation_on_missing_log_date_exact() {
    let mut printer = FakePrinter::new();
    printer.print_task(&PrintableTask::new("a", 1, Incomplete));
    printer
        .validate()
        .printed_task(
            &PrintableTask::new("a", 1, Incomplete)
                .log_date(LogDate::YearMonthDay(2021, 04, 01)),
        )
        .end();
}

#[test]
#[should_panic(expected = "Unexpected log date")]
fn fail_validation_on_incorrect_log_date_exact() {
    let mut printer = FakePrinter::new();
    printer.print_task(
        &PrintableTask::new("a", 1, Incomplete).log_date(LogDate::Invisible),
    );
    printer
        .validate()
        .printed_task(
            &PrintableTask::new("a", 1, Incomplete)
                .log_date(LogDate::YearMonthDay(2021, 04, 01)),
        )
        .end();
}

#[test]
#[should_panic(expected = "Unexpected priority")]
fn fail_validation_on_missing_priority_exact() {
    let mut printer = FakePrinter::new();
    printer.print_task(&PrintableTask::new("a", 1, Incomplete));
    printer
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete).priority(1))
        .end();
}

#[test]
#[should_panic(expected = "Unexpected priority")]
fn fail_validation_on_extraneous_priority() {
    let mut printer = FakePrinter::new();
    printer.print_task(&PrintableTask::new("a", 1, Incomplete).priority(1));
    printer
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .end();
}
