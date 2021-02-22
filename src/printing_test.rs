use printing::*;

#[test]
fn validate_single_task() {
    let mut printer = FakePrinter::new();
    printer.print_task(&PrintableTask {
        desc: "a",
        number: 1,
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
    });
    printer.print_task(&PrintableTask {
        desc: "b",
        number: 2,
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
    });
    printer.validate().end();
}
