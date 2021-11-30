use super::PrintableWarning::*;
use super::BriefPrintableTask;
use cli::Key::*;
use super::Status::*;

#[test]
fn display_no_match_found_warning() {
    let fmt = format!(
        "{}",
        NoMatchFoundForKey {
            requested_key: ByNumber(10),
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
        NoMatchFoundForKey {
            requested_key: ByName("blah".to_string()),
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
        NoMatchFoundForKey {
            requested_key: ByRange(10, 20),
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
        CannotCheckBecauseAlreadyComplete {
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
        CannotRestoreBecauseAlreadyIncomplete {
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
        CannotUnblockBecauseTaskIsNotBlocked {
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
