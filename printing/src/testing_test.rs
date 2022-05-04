#![allow(clippy::zero_prefixed_literal)]

use {
    crate::{
        Action::*, BriefPrintableTask, FakePrinter, LogDate::*, Plicit::*,
        PrintableError, PrintableTask, PrintableWarning, Status::*,
        TodoPrinter,
    },
    lookup_key::Key,
    testing::ymdhms,
};

#[test]
fn validate_task() {
    let mut printer = FakePrinter::default();
    printer.print_task(&PrintableTask::new("a", 1, Incomplete));
    printer
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .end();
}

#[test]
fn validate_multiple_tasks() {
    let mut printer = FakePrinter::default();
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
    let mut printer = FakePrinter::default();
    let warning = PrintableWarning::NoMatchFoundForKey {
        requested_key: Key::ByName("a".to_string()),
    };
    printer.print_warning(&warning);
    printer.validate().printed_warning(&warning).end();
}

#[test]
fn validate_error() {
    let mut printer = FakePrinter::default();
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
    let mut printer = FakePrinter::default();
    printer
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .end();
}

#[test]
#[should_panic(expected = "Unexpected description: \"a\"")]
fn fail_validation_on_incorrect_description_exact() {
    let mut printer = FakePrinter::default();
    printer.print_task(&PrintableTask::new("a", 1, Incomplete));
    printer
        .validate()
        .printed_task(&PrintableTask::new("b", 1, Incomplete))
        .end();
}

#[test]
#[should_panic(expected = "Unexpected number: 1")]
fn fail_validation_on_incorrect_number_exact() {
    let mut printer = FakePrinter::default();
    printer.print_task(&PrintableTask::new("a", 1, Incomplete));
    printer
        .validate()
        .printed_task(&PrintableTask::new("a", 2, Incomplete))
        .end();
}

#[test]
#[should_panic(expected = "Extra tasks were recorded: ")]
fn fail_validation_on_extra_tasks() {
    let mut printer = FakePrinter::default();
    printer.print_task(&PrintableTask::new("a", 1, Incomplete));
    printer.validate().end();
}

#[test]
#[should_panic(expected = "Unexpected status")]
fn fail_validation_on_incorrect_status_exact() {
    let mut printer = FakePrinter::default();
    printer.print_task(&PrintableTask::new("a", 1, Incomplete));
    printer
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Blocked))
        .end();
}

#[test]
#[should_panic(expected = "Unexpected action")]
fn fail_validation_on_incorrect_action_exact() {
    let mut printer = FakePrinter::default();
    printer.print_task(&PrintableTask::new("a", 1, Incomplete).action(New));
    printer
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete).action(Select))
        .end();
}

#[test]
#[should_panic(expected = "Unexpected warning")]
fn fail_validation_on_wrong_warning() {
    let mut printer = FakePrinter::default();
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
    let mut printer = FakePrinter::default();
    let warning = PrintableWarning::CannotPuntBecauseComplete {
        cannot_punt: BriefPrintableTask::new(0, Incomplete),
    };
    printer.print_task(&PrintableTask::new("a", 1, Incomplete));
    printer.validate().printed_warning(&warning).end();
}

#[test]
#[should_panic(expected = "Unexpected error")]
fn fail_validation_on_wrong_error() {
    let mut printer = FakePrinter::default();
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
    let mut printer = FakePrinter::default();
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
    let mut printer = FakePrinter::default();
    printer.print_task(&PrintableTask::new("a", 1, Incomplete));
    printer
        .validate()
        .printed_task(
            &PrintableTask::new("a", 1, Incomplete)
                .log_date(YearMonthDay(2021, 04, 01)),
        )
        .end();
}

#[test]
#[should_panic(expected = "Unexpected log date")]
fn fail_validation_on_incorrect_log_date_exact() {
    let mut printer = FakePrinter::default();
    printer.print_task(
        &PrintableTask::new("a", 1, Incomplete).log_date(Invisible),
    );
    printer
        .validate()
        .printed_task(
            &PrintableTask::new("a", 1, Incomplete)
                .log_date(YearMonthDay(2021, 04, 01)),
        )
        .end();
}

#[test]
#[should_panic(expected = "Unexpected priority")]
fn fail_validation_on_missing_priority_exact() {
    let mut printer = FakePrinter::default();
    printer.print_task(&PrintableTask::new("a", 1, Incomplete));
    printer
        .validate()
        .printed_task(
            &PrintableTask::new("a", 1, Incomplete).priority(Explicit(1)),
        )
        .end();
}

#[test]
#[should_panic(expected = "Unexpected priority")]
fn fail_validation_on_extraneous_priority() {
    let mut printer = FakePrinter::default();
    printer.print_task(
        &PrintableTask::new("a", 1, Incomplete).priority(Implicit(1)),
    );
    printer
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .end();
}

#[test]
#[should_panic(expected = "Unexpected priority")]
fn fail_validation_on_priority_with_wrong_plicit() {
    let mut printer = FakePrinter::default();
    printer.print_task(
        &PrintableTask::new("a", 1, Incomplete).priority(Implicit(1)),
    );
    printer
        .validate()
        .printed_task(
            &PrintableTask::new("a", 1, Incomplete).priority(Explicit(1)),
        )
        .end();
}

#[test]
#[should_panic(expected = "Unexpected priority")]
fn fail_validation_on_priority_with_wrong_value() {
    let mut printer = FakePrinter::default();
    printer.print_task(
        &PrintableTask::new("a", 1, Incomplete).priority(Explicit(1)),
    );
    printer
        .validate()
        .printed_task(
            &PrintableTask::new("a", 1, Incomplete).priority(Explicit(2)),
        )
        .end();
}

#[test]
#[should_panic(expected = "Missing required start date")]
fn fail_validation_on_missing_start_date() {
    let mut printer = FakePrinter::default();
    printer.print_task(&PrintableTask::new("a", 1, Incomplete));
    printer
        .validate()
        .printed_task(
            &PrintableTask::new("a", 1, Incomplete)
                .start_date(ymdhms(2021, 05, 27, 00, 00, 00)),
        )
        .end()
}

#[test]
#[should_panic(expected = "Unexpected start date")]
fn fail_validation_on_incorrect_start_date() {
    let mut printer = FakePrinter::default();
    printer.print_task(
        &PrintableTask::new("a", 1, Incomplete)
            .start_date(ymdhms(2021, 05, 28, 00, 00, 00)),
    );
    printer
        .validate()
        .printed_task(
            &PrintableTask::new("a", 1, Incomplete)
                .start_date(ymdhms(2021, 05, 27, 00, 00, 00)),
        )
        .end()
}
