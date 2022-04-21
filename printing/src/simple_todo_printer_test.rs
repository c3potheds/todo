#![allow(clippy::zero_prefixed_literal)]

use {
    crate::{
        BriefPrintableTask, PrintableError, PrintableTask, PrintableWarning,
        PrintingContext, SimpleTodoPrinter, Status::*, TodoPrinter,
    },
    lookup_key::Key,
    std::io::Write,
    testing::ymdhms,
};

fn create_printer_to_vec() -> SimpleTodoPrinter<Vec<u8>> {
    SimpleTodoPrinter {
        out: Vec::new(),
        context: PrintingContext {
            max_index_digits: 3,
            width: 80,
            now: ymdhms(2022, 02, 22, 2, 22, 22),
        },
    }
}

struct BrokenPipe;

impl Write for BrokenPipe {
    fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
        Err(std::io::Error::new(
            std::io::ErrorKind::BrokenPipe,
            "broken pipe",
        ))
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Err(std::io::Error::new(
            std::io::ErrorKind::BrokenPipe,
            "broken pipe",
        ))
    }
}

fn create_printer_to_broken_pipe() -> SimpleTodoPrinter<BrokenPipe> {
    SimpleTodoPrinter {
        out: BrokenPipe,
        context: PrintingContext {
            max_index_digits: 3,
            width: 80,
            now: ymdhms(2022, 02, 22, 2, 22, 22),
        },
    }
}

#[test]
fn print_single_task() {
    let mut printer = create_printer_to_vec();
    printer.print_task(&PrintableTask::new("a", 1, Incomplete));
    let out = String::from_utf8(printer.out).unwrap();
    assert_eq!(out, "      \u{1b}[33m1)\u{1b}[0m a\n");
}

#[test]
fn print_multiple_tasks() {
    let mut printer = create_printer_to_vec();
    printer.print_task(&PrintableTask::new("a", 1, Incomplete));
    printer.print_task(&PrintableTask::new("b", 2, Incomplete));
    let out = String::from_utf8(printer.out).unwrap();
    assert_eq!(
        out,
        concat!(
            "      \u{1b}[33m1)\u{1b}[0m a\n",
            "      \u{1b}[33m2)\u{1b}[0m b\n"
        )
    );
}

#[test]
fn print_task_to_broken_pipe() {
    let mut printer = create_printer_to_broken_pipe();
    printer.print_task(&PrintableTask::new("a", 1, Incomplete));
}

#[test]
fn print_warning_to_broken_pipe() {
    let mut printer = create_printer_to_broken_pipe();
    printer.print_warning(&PrintableWarning::AmbiguousKey {
        key: Key::ByName("a".to_string()),
        matches: vec![
            BriefPrintableTask::new(1, Incomplete),
            BriefPrintableTask::new(2, Incomplete),
        ],
    });
}

#[test]
fn print_error_to_broken_pipe() {
    let mut printer = create_printer_to_broken_pipe();
    printer.print_error(&PrintableError::ConflictingArgs((
        "a".to_string(),
        "b".to_string(),
    )));
}
