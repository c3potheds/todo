#![allow(clippy::zero_prefixed_literal)]

use chrono::DateTime;
use chrono::Utc;
use todo_testing::ymdhms;

use crate::Action::*;
use crate::LogDate::*;
use crate::Plicit::*;
use crate::PrintableTask;
use crate::PrintingContext;
use crate::SimpleTodoPrinter;
use crate::Status::*;
use crate::TodoPrinter;

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
        now,
    }
}

fn print_task_with_context(
    context: PrintingContext,
    task: &PrintableTask,
) -> String {
    let mut out: Vec<u8> = Vec::new();
    let mut printer = SimpleTodoPrinter {
        out: &mut out,
        context,
    };
    printer.print_task(task);
    String::from(std::str::from_utf8(&out).unwrap())
}

fn print_task(task: &PrintableTask) -> String {
    let context = make_printing_context();
    print_task_with_context(context, task)
}

#[test]
fn fmt_blank_task() {
    let fmt = print_task(&PrintableTask::new("", 1, Incomplete));
    assert_eq!(fmt, "      \u{1b}[33m1)\u{1b}[0m\n");
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
    let fmt =
        print_task(&PrintableTask::new("done!", 0, Complete).action(Check));
    assert_eq!(
        fmt,
        "\u{1b}[32m[✓]\u{1b}[0m   \u{1b}[32m0)\u{1b}[0m done!\n"
    );
}

#[test]
fn show_empty_box_on_uncheck_action() {
    let fmt =
        print_task(&PrintableTask::new("oh", 1, Incomplete).action(Uncheck));
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
        .log_date(YearMonthDay(2020, 03, 15)),
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
        .log_date(YearMonthDay(2021, 02, 28)),
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
        .log_date(Invisible),
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
fn show_implicit_priority_on_task() {
    let fmt = print_task(
        &PrintableTask::new("a", 1, Incomplete).priority(Implicit(1)),
    );
    assert_eq!(
        fmt,
        "      \u{1b}[33m1)\u{1b}[0m \u{1b}[1;3;35mP1\u{1b}[0m a\n"
    );
}

#[test]
fn show_explicit_priority_on_task() {
    let fmt = print_task(
        &PrintableTask::new("a", 1, Incomplete).priority(Explicit(1)),
    );
    assert_eq!(
        fmt,
        "      \u{1b}[33m1)\u{1b}[0m \u{1b}[1;35mP1\u{1b}[0m a\n"
    );
}

#[test]
fn show_implicit_meh_due_date_on_task() {
    let now = ymdhms(2021, 04, 15, 10, 00, 00);
    let task = PrintableTask::new("a", 1, Incomplete)
        .due_date(Implicit(now + chrono::Duration::days(2)));
    let fmt = print_task_with_context(now_context(now), &task);
    assert_eq!(
        fmt,
        "      \u{1b}[33m1)\u{1b}[0m \u{1b}[1;2;3;37mDue in 2 days\u{1b}[0m a\n"
    );
}

#[test]
fn show_explicit_meh_due_date_on_task() {
    let now = ymdhms(2021, 04, 15, 10, 00, 00);
    let task = PrintableTask::new("a", 1, Incomplete)
        .due_date(Explicit(now + chrono::Duration::days(2)));
    let fmt = print_task_with_context(now_context(now), &task);
    assert_eq!(
        fmt,
        "      \u{1b}[33m1)\u{1b}[0m \u{1b}[1;2;37mDue in 2 days\u{1b}[0m a\n"
    );
}

#[test]
fn show_implicit_moderate_due_date_on_task() {
    let now = ymdhms(2021, 04, 15, 10, 00, 00);
    let task = PrintableTask::new("a", 1, Incomplete)
        .due_date(Implicit(now + chrono::Duration::hours(9)));
    let fmt = print_task_with_context(now_context(now), &task);
    assert_eq!(
        fmt,
        "      \u{1b}[33m1)\u{1b}[0m \u{1b}[1;3;33mDue in 9 hours\u{1b}[0m a\n"
    );
}

#[test]
fn show_explicit_moderate_due_date_on_task() {
    let now = ymdhms(2021, 04, 15, 10, 00, 00);
    let task = PrintableTask::new("a", 1, Incomplete)
        .due_date(Explicit(now + chrono::Duration::hours(9)));
    let fmt = print_task_with_context(now_context(now), &task);
    assert_eq!(
        fmt,
        "      \u{1b}[33m1)\u{1b}[0m \u{1b}[1;33mDue in 9 hours\u{1b}[0m a\n"
    );
}

#[test]
fn show_implicit_urgent_due_date_on_task() {
    let now = ymdhms(2021, 04, 15, 10, 00, 00);
    let task = PrintableTask::new("a", 1, Incomplete)
        .due_date(Implicit(now - chrono::Duration::days(1)));
    let fmt = print_task_with_context(now_context(now), &task);
    assert_eq!(
        fmt,
        "      \u{1b}[33m1)\u{1b}[0m \u{1b}[1;3;31mDue 1 day ago\u{1b}[0m a\n"
    );
}

#[test]
fn show_explicit_urgent_due_date_on_task() {
    let now = ymdhms(2021, 04, 15, 10, 00, 00);
    let task = PrintableTask::new("a", 1, Incomplete)
        .due_date(Explicit(now - chrono::Duration::days(1)));
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
        .priority(Implicit(1))
        .due_date(Implicit(now - chrono::Duration::days(1)));
    let fmt = print_task_with_context(now_context(now), &task);
    assert_eq!(
        fmt,
        concat!(
            "      \u{1b}[33m1)\u{1b}[0m ",
            "\u{1b}[1;3;35mP1\u{1b}[0m ",
            "\u{1b}[1;3;31mDue 1 day ago\u{1b}[0m ",
            "a\n"
        ),
    );
}

#[test]
fn show_snooze_date_on_task() {
    let now = ymdhms(2021, 05, 27, 12, 00, 00);
    let snooze_date = ymdhms(2021, 05, 27, 14, 00, 00);
    let task = PrintableTask::new("a", 1, Incomplete).start_date(snooze_date);
    let fmt = print_task_with_context(now_context(now), &task);
    assert_eq!(
        fmt,
        concat!(
            "      \u{1b}[33m1)\u{1b}[0m ",
            "\u{1b}[1;35mSnoozed for 2 hours\u{1b}[0m a\n"
        )
    );
}

#[test]
fn do_not_show_snooze_time_if_time_elapsed() {
    let now = ymdhms(2021, 05, 27, 15, 00, 00);
    let snooze_date = ymdhms(2021, 05, 27, 14, 00, 00);
    let task = PrintableTask::new("a", 1, Incomplete).start_date(snooze_date);
    let fmt = print_task_with_context(now_context(now), &task);
    assert_eq!(fmt, "      \u{1b}[33m1)\u{1b}[0m a\n");
}

#[test]
fn show_lock_icon_on_lock_action() {
    let fmt =
        print_task(&PrintableTask::new("blocked", 5, Blocked).action(Lock));
    assert_eq!(
        fmt,
        " \u{1b}[31m🔒\u{1b}[0m   \u{1b}[31m5)\u{1b}[0m blocked\n"
    );
}

#[test]
fn show_unlock_icon_on_unlock_action() {
    let fmt = print_task(
        &PrintableTask::new("unblocked", 10, Incomplete).action(Unlock),
    );
    assert_eq!(
        fmt,
        " \u{1b}[32m🔓\u{1b}[0m  \u{1b}[33m10)\u{1b}[0m unblocked\n"
    );
}

#[test]
fn show_punt_icon_on_punt_action() {
    let fmt = print_task(
        &PrintableTask::new("punt this", 5, Incomplete).action(Punt),
    );
    assert_eq!(fmt, " ⏎    \u{1b}[33m5)\u{1b}[0m punt this\n");
}

#[test]
fn show_done_icon_on_done_action() {
    let fmt = print_task(
        &PrintableTask::new("finish this", 5, Incomplete).action(Check),
    );
    assert_eq!(
        fmt,
        "\u{1b}[32m[✓]\u{1b}[0m   \u{1b}[33m5)\u{1b}[0m finish this\n"
    );
}

#[test]
fn show_adeps_stats() {
    let fmt =
        print_task(&PrintableTask::new("a", 1, Incomplete).adeps_stats(1, 2));
    assert_eq!(
        fmt,
        "      \u{1b}[33m1)\u{1b}[0m \u{1b}[37m🔓1/2\u{1b}[0m a\n"
    );
}

#[test]
fn show_adeps_stats_and_priority() {
    let fmt = print_task(
        &PrintableTask::new("a", 1, Incomplete)
            .priority(Explicit(1))
            .adeps_stats(2, 4),
    );
    assert_eq!(
        fmt,
        concat!(
            "      \u{1b}[33m1)\u{1b}[0m ",
            "\u{1b}[1;35mP1\u{1b}[0m ",
            "\u{1b}[37m🔓2/4\u{1b}[0m a\n"
        )
    );
}

#[test]
fn show_due_date_and_adeps_stats() {
    let now = ymdhms(2021, 04, 15, 10, 00, 00);
    let fmt = print_task_with_context(
        now_context(now),
        &PrintableTask::new("a", 1, Incomplete)
            .due_date(Explicit(now - chrono::Duration::days(1)))
            .adeps_stats(12, 20),
    );
    assert_eq!(
        fmt,
        concat!(
            "      \u{1b}[33m1)\u{1b}[0m ",
            "\u{1b}[37m🔓12/20\u{1b}[0m ",
            "\u{1b}[1;31mDue 1 day ago\u{1b}[0m a\n"
        )
    );
}

#[test]
fn show_deps_stats() {
    let fmt =
        print_task(&PrintableTask::new("a", 1, Incomplete).deps_stats(1, 2));
    assert_eq!(
        fmt,
        "      \u{1b}[33m1)\u{1b}[0m \u{1b}[31m🔒1/2\u{1b}[0m a\n"
    );
}

#[test]
fn show_deps_and_adeps_stats() {
    let fmt = print_task(
        &PrintableTask::new("a", 1, Incomplete)
            .deps_stats(1, 2)
            .adeps_stats(3, 4),
    );
    assert_eq!(
        fmt,
        concat!(
            "      \u{1b}[33m1)\u{1b}[0m ",
            "\u{1b}[31m🔒1/2\u{1b}[0m ",
            "\u{1b}[37m🔓3/4\u{1b}[0m a\n"
        )
    );
}

#[test]
fn show_deps_and_adeps_stats_and_priority() {
    let fmt = print_task(
        &PrintableTask::new("a", 1, Incomplete)
            .priority(Explicit(1))
            .deps_stats(2, 4)
            .adeps_stats(5, 6),
    );
    assert_eq!(
        fmt,
        concat!(
            "      \u{1b}[33m1)\u{1b}[0m ",
            "\u{1b}[1;35mP1\u{1b}[0m ",
            "\u{1b}[31m🔒2/4\u{1b}[0m ",
            "\u{1b}[37m🔓5/6\u{1b}[0m a\n"
        )
    );
}

#[test]
fn show_due_date_and_deps_stats() {
    let now = ymdhms(2021, 04, 15, 10, 00, 00);
    let fmt = print_task_with_context(
        now_context(now),
        &PrintableTask::new("a", 1, Incomplete)
            .due_date(Explicit(now - chrono::Duration::days(1)))
            .deps_stats(12, 20),
    );
    assert_eq!(
        fmt,
        concat!(
            "      \u{1b}[33m1)\u{1b}[0m ",
            "\u{1b}[31m🔒12/20\u{1b}[0m ",
            "\u{1b}[1;31mDue 1 day ago\u{1b}[0m a\n"
        )
    );
}

#[test]
fn show_punctuality_completed_early() {
    let now = ymdhms(2022, 04, 11, 09, 00, 00);
    let fmt = print_task_with_context(
        now_context(now),
        &PrintableTask::new("a", 0, Complete)
            .punctuality(-chrono::Duration::days(1)),
    );
    assert_eq!(
        fmt,
        concat!(
            "      \u{1b}[32m0)\u{1b}[0m ",
            "\u{1b}[1;32mDone 1 day early\u{1b}[0m a\n"
        )
    );
}

#[test]
fn show_punctuality_completed_late() {
    let now = ymdhms(2022, 04, 11, 09, 00, 00);
    let fmt = print_task_with_context(
        now_context(now),
        &PrintableTask::new("a", 0, Complete)
            .punctuality(chrono::Duration::days(1)),
    );
    assert_eq!(
        fmt,
        concat!(
            "      \u{1b}[32m0)\u{1b}[0m ",
            "\u{1b}[1;31mDone 1 day late\u{1b}[0m a\n"
        )
    );
}

#[test]
fn show_punctuality_completed_minutes_early() {
    let now = ymdhms(2022, 04, 11, 09, 00, 00);
    let fmt = print_task_with_context(
        now_context(now),
        &PrintableTask::new("a", 0, Complete)
            .punctuality(-chrono::Duration::minutes(1)),
    );
    assert_eq!(
        fmt,
        concat!(
            "      \u{1b}[32m0)\u{1b}[0m ",
            "\u{1b}[1;32mDone 1 minute early\u{1b}[0m a\n"
        )
    );
}

#[test]
fn show_punctuality_completed_minutes_late() {
    let now = ymdhms(2022, 04, 11, 09, 00, 00);
    let fmt = print_task_with_context(
        now_context(now),
        &PrintableTask::new("a", 0, Complete)
            .punctuality(chrono::Duration::minutes(1)),
    );
    assert_eq!(
        fmt,
        concat!(
            "      \u{1b}[32m0)\u{1b}[0m ",
            "\u{1b}[1;31mDone 1 minute late\u{1b}[0m a\n"
        )
    );
}

#[test]
fn show_implicit_tags() {
    let fmt =
        print_task(&PrintableTask::new("a", 1, Incomplete).tag("x").tag("y"));
    assert_eq!(
        fmt,
        concat!(
            "      \u{1b}[33m1)\u{1b}[0m ",
            "\u{1b}[3;38;5;11mx\u{1b}[0m ",
            "\u{1b}[3;38;5;15my\u{1b}[0m ",
            "a\n"
        )
    );
}

#[test]
fn explicit_tag() {
    let fmt = print_task(&PrintableTask::new("a", 1, Incomplete).as_tag());
    assert_eq!(
        fmt,
        "      \u{1b}[33m1)\u{1b}[0m \u{1b}[38;5;2ma\u{1b}[0m\n",
    );
}

#[test]
fn explicit_tag_with_implicit_tags() {
    let fmt = print_task(
        &PrintableTask::new("a", 1, Incomplete)
            .tag("x")
            .tag("y")
            .as_tag(),
    );
    assert_eq!(
        fmt,
        concat!(
            "      \u{1b}[33m1)\u{1b}[0m ",
            "\u{1b}[3;38;5;11mx\u{1b}[0m ",
            "\u{1b}[3;38;5;15my\u{1b}[0m ",
            "\u{1b}[38;5;2ma\u{1b}[0m\n",
        )
    );
}

#[test]
fn explicit_tag_with_implicit_tags_and_punctuality() {
    let fmt = print_task(
        &PrintableTask::new("a", 0, Complete)
            .punctuality(chrono::Duration::days(1))
            .tag("x")
            .tag("y")
            .as_tag(),
    );
    assert_eq!(
        fmt,
        concat!(
            "      \u{1b}[32m0)\u{1b}[0m ",
            "\u{1b}[1;31mDone 1 day late\u{1b}[0m ",
            "\u{1b}[3;38;5;11mx\u{1b}[0m ",
            "\u{1b}[3;38;5;15my\u{1b}[0m ",
            "\u{1b}[38;5;2ma\u{1b}[0m\n",
        )
    );
}

#[test]
fn do_not_split_url() {
    let context = PrintingContext {
        max_index_digits: 3,
        width: 40,
        now: Utc::now(),
    };
    let fmt = print_task_with_context(
        context,
        &PrintableTask::new(
            "http://example.com/this/is/a/long/url/that/should/not/be/split",
            1,
            Incomplete,
        ),
    );
    assert_eq!(
        fmt,
        concat!(
            "      \u{1b}[33m1)\u{1b}[0m ",
            "http://example.com/this/is/a/long/url/that/should/not/be/split\n"
        )
    );
}

#[test]
fn truncate_tags_if_they_do_not_fit_on_line() {
    let context = PrintingContext {
        max_index_digits: 3,
        width: 45,
        now: Utc::now(),
    };
    let fmt = print_task_with_context(
        context,
        &PrintableTask::new("a", 1, Incomplete)
            .tag("project-stardust")
            .tag("project-zeppelin")
            .tag("project-apollo")
            .truncate_tags_if_needed(true),
    );
    // Tags are truncated to "project-stardust ... project-apollo"
    assert_eq!(
        fmt,
        concat!(
            "      \u{1b}[33m1)\u{1b}[0m ",
            "\u{1b}[3;38;5;9mproject-stardust\u{1b}[0m ",
            "... ",
            "\u{1b}[3;38;5;12mproject-apollo\u{1b}[0m\n",
            "         a\n",
        )
    );
}

#[test]
fn do_not_truncate_tags_if_not_opted_in() {
    let context = PrintingContext {
        max_index_digits: 3,
        width: 45,
        now: Utc::now(),
    };
    let fmt = print_task_with_context(
        context,
        &PrintableTask::new("a", 1, Incomplete)
            .tag("project-stardust")
            .tag("project-zeppelin")
            .tag("project-apollo"),
    );
    // "project-stardust project-zeppelin project-apollo" don't fit on one line,
    // so "project-apollo" is moved to the next line.
    assert_eq!(
        fmt,
        concat!(
            "      \u{1b}[33m1)\u{1b}[0m ",
            "\u{1b}[3;38;5;9mproject-stardust\u{1b}[0m ",
            "\u{1b}[3;38;5;1mproject-zeppelin\u{1b}[0m\n",
            "         \u{1b}[3;38;5;12mproject-apollo\u{1b}[0m a\n",
        )
    );
}

#[test]
fn start_description_on_new_line_if_tags_are_truncated() {
    let context = PrintingContext {
        max_index_digits: 3,
        // The content would fit in 47 columns with truncation, but we put the
        // description on a new line when tags are truncated to better visually
        // separate the tags from the description.
        width: 47,
        now: Utc::now(),
    };
    let fmt = print_task_with_context(
        context,
        &PrintableTask::new("a", 1, Incomplete)
            .tag("project-stardust")
            .tag("project-zeppelin")
            .tag("project-apollo")
            .truncate_tags_if_needed(true),
    );
    assert_eq!(
        fmt,
        concat!(
            "      \u{1b}[33m1)\u{1b}[0m ",
            "\u{1b}[3;38;5;9mproject-stardust\u{1b}[0m ",
            "... ",
            "\u{1b}[3;38;5;12mproject-apollo\u{1b}[0m\n",
            "         a\n",
        )
    );
}
