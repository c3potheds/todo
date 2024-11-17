use crate::BriefPrintableTask;
use crate::PrintableError::*;
use crate::Status::*;

#[test]
fn display_cannot_check_because_blocked_error() {
    let fmt = format!(
        "{}",
        CannotCheckBecauseBlocked {
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
        CannotRestoreBecauseAntidependencyIsComplete {
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
        CannotBlockBecauseWouldCauseCycle {
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
    let fmt = format!("{}", CannotEditBecauseUnexpectedNumber { requested: 0 });
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
    let fmt =
        format!("{}", CannotEditBecauseNoTaskWithNumber { requested: 100 });
    assert_eq!(fmt, "\u{1b}[1;31merror\u{1b}[0m: No task with number 100)");
}

#[test]
fn display_failed_to_use_text_editor_error() {
    let fmt = format!("{}", FailedToUseTextEditor);
    assert_eq!(
        fmt,
        "\u{1b}[1;31merror\u{1b}[0m: Failed to open text editor"
    );
}
