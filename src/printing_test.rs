use printing::*;

#[test]
fn fmt_incomplete_task() {
    let fmt = format!(
        "{}",
        PrintableTask {
            desc: "a",
            number: 1,
            status: TaskStatus::Incomplete,
        }
    );
    // The 1) is wrapped in ANSI codes painting it yellow.
    assert_eq!(fmt, "\t\u{1b}[33m1)\u{1b}[0m\ta");
}

#[test]
fn fmt_complete_task() {
    let fmt = format!(
        "{}",
        PrintableTask {
            desc: "b",
            number: 0,
            status: TaskStatus::Complete,
        }
    );
    // The 0) is wrapped in ANSI codes painting it green.
    assert_eq!(fmt, "\t\u{1b}[32m0)\u{1b}[0m\tb");
}

#[test]
fn fmt_blocked_task() {
    let fmt = format!(
        "{}",
        PrintableTask {
            desc: "c",
            number: 2,
            status: TaskStatus::Blocked,
        }
    );
    // The 2) is wrapped in ANSI codes painting it red.
    assert_eq!(fmt, "\t\u{1b}[31m2)\u{1b}[0m\tc");
}

#[test]
fn validate_single_task() {
    let mut printer = FakePrinter::new();
    printer.print_task(&PrintableTask {
        desc: "a",
        number: 1,
        status: TaskStatus::Incomplete,
    });
    printer
        .validate()
        .printed(&[Expect::Desc("a"), Expect::Number(1)])
        .end();
}

#[test]
fn validate_multiple_tasks() {
    let mut printer = FakePrinter::new();
    printer.print_task(&PrintableTask {
        desc: "a",
        number: 1,
        status: TaskStatus::Incomplete,
    });
    printer.print_task(&PrintableTask {
        desc: "b",
        number: 2,
        status: TaskStatus::Incomplete,
    });
    printer
        .validate()
        .printed(&[Expect::Desc("a"), Expect::Number(1)])
        .printed(&[Expect::Desc("b"), Expect::Number(2)])
        .end();
}

#[test]
#[should_panic(expected = "Missing task: [Desc(\"a\")]")]
fn fail_validation_on_missing_task() {
    let mut printer = FakePrinter::new();
    printer.validate().printed(&[Expect::Desc("a")]).end();
}

#[test]
#[should_panic(expected = "Unexpected description: \"a\"")]
fn fail_validation_on_incorrect_description() {
    let mut printer = FakePrinter::new();
    printer.print_task(&PrintableTask {
        desc: "a",
        number: 1,
        status: TaskStatus::Incomplete,
    });
    printer.validate().printed(&[Expect::Desc("b")]).end();
}

#[test]
#[should_panic(expected = "Unexpected number: 1")]
fn fail_validation_on_incorrect_number() {
    let mut printer = FakePrinter::new();
    printer.print_task(&PrintableTask {
        desc: "a",
        number: 1,
        status: TaskStatus::Incomplete,
    });
    printer.validate().printed(&[Expect::Number(2)]).end();
}

#[test]
#[should_panic(expected = "Extra tasks were recorded: ")]
fn fail_validation_on_extra_tasks() {
    let mut printer = FakePrinter::new();
    printer.print_task(&PrintableTask {
        desc: "a",
        number: 1,
        status: TaskStatus::Incomplete,
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
    });
    printer
        .validate()
        .printed(&[Expect::Status(TaskStatus::Complete)])
        .end();
}
