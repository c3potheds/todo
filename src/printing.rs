use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

pub struct PrintableTask<'a> {
    pub desc: &'a str,
    pub number: i32,
}

impl<'a> Display for PrintableTask<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}) {}", self.number, self.desc)
    }
}

pub trait TodoPrinter {
    fn print_task(&mut self, task: &PrintableTask);
}

pub struct SimpleTodoPrinter {}

impl TodoPrinter for SimpleTodoPrinter {
    fn print_task(&mut self, task: &PrintableTask) {
        println!("{}", task);
    }
}

#[derive(Debug)]
struct PrintedTaskInfo {
    desc: String,
    number: i32,
}

pub struct FakePrinter {
    record: Vec<PrintedTaskInfo>,
}

#[derive(Debug)]
pub enum Expect<'a> {
    Desc(&'a str),
    Number(i32),
}

impl<'a> Expect<'a> {
    fn validate(&self, info: &PrintedTaskInfo) {
        match self {
            Expect::Desc(desc) => {
                if desc != &info.desc {
                    panic!(
                        "Unexpected description: {:?}. (Expected {:?})",
                        &info.desc, desc
                    );
                }
            }
            Expect::Number(number) => {
                if *number != info.number {
                    panic!(
                        "Unexpected number: {} (Expected {})",
                        info.number, number
                    );
                }
            }
        }
    }
}

pub struct Validation<'a> {
    record: &'a mut Vec<PrintedTaskInfo>,
}

impl<'a> Validation<'a> {
    pub fn printed(self, es: &[Expect<'a>]) -> Validation<'a> {
        if self.record.len() == 0 {
            panic!("Missing task: {:?}", es);
        }
        let info = self.record.drain(0..1).nth(0).unwrap();
        es.iter().for_each(|e| e.validate(&info));
        self
    }

    pub fn end(self) {
        if self.record.len() > 0 {
            panic!("Extra tasks were recorded: {:?}", self.record);
        }
    }
}

impl FakePrinter {
    pub fn new() -> Self {
        Self { record: Vec::new() }
    }

    pub fn validate<'a>(&'a mut self) -> Validation<'a> {
        Validation {
            record: &mut self.record,
        }
    }
}

impl TodoPrinter for FakePrinter {
    fn print_task(&mut self, task: &PrintableTask) {
        self.record.push(PrintedTaskInfo {
            desc: task.desc.to_string(),
            number: task.number,
        });
    }
}
