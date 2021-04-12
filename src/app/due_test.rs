use app::testing::Fixture;
use chrono::Local;
use chrono::TimeZone;
use chrono::Utc;
use model::TaskStatus::*;
use printing::DueDate;
use printing::PrintableTask;
use printing::Urgency::*;

#[test]
#[ignore = "app.new.due"]
fn show_tasks_with_due_date() {
    let mut fix = Fixture::new();
    fix.clock.now = Local
        .ymd(2021, 04, 12)
        .and_hms(14, 00, 00)
        .with_timezone(&Utc);
    fix.test("todo new a b c --due 1 day");
    fix.test("todo new d e f");
    fix.test("todo due")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete).due_date(
            DueDate {
                urgency: Meh,
                desc: "in 1day",
            },
        ))
        .printed_task(&PrintableTask::new("b", 2, Incomplete).due_date(
            DueDate {
                urgency: Meh,
                desc: "in 1day",
            },
        ))
        .printed_task(&PrintableTask::new("c", 3, Incomplete).due_date(
            DueDate {
                urgency: Meh,
                desc: "in 1day",
            },
        ))
        .end();
}

#[test]
#[ignore = "app.new.due"]
fn show_tasks_with_due_date_includes_blocked() {
    let mut fix = Fixture::new();
    fix.clock.now = Local
        .ymd(2021, 04, 12)
        .and_hms(14, 00, 00)
        .with_timezone(&Utc);
    fix.test("todo new a --due 5 hours");
    fix.test("todo new b -p a");
    fix.test("todo new c -p b --due 2 days");
    fix.test("todo new d e f");
    fix.test("todo due")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete).due_date(
            DueDate {
                urgency: Moderate,
                desc: "in 5h",
            },
        ))
        .printed_task(&PrintableTask::new("b", 5, Blocked).due_date(DueDate {
            urgency: Meh,
            desc: "in 2days",
        }))
        .printed_task(&PrintableTask::new("c", 6, Blocked).due_date(DueDate {
            urgency: Meh,
            desc: "in 2days",
        }))
        .end();
}

#[test]
#[ignore = "app.new.due"]
fn show_tasks_with_due_date_excludes_complete() {
    let mut fix = Fixture::new();
    fix.clock.now = Local
        .ymd(2021, 04, 12)
        .and_hms(14, 00, 00)
        .with_timezone(&Utc);
    fix.test("todo new a --due 5 hours");
    fix.test("todo new b -p a");
    fix.test("todo new c -p b --due 2 days");
    fix.test("todo new d e f");
    fix.test("todo check a");
    fix.test("todo due")
        .validate()
        .printed_task(&PrintableTask::new("b", 1, Incomplete).due_date(
            DueDate {
                urgency: Meh,
                desc: "in 2days",
            },
        ))
        .printed_task(&PrintableTask::new("c", 5, Blocked).due_date(DueDate {
            urgency: Meh,
            desc: "in 2days",
        }))
        .end();
}

#[test]
#[ignore = "app.new.due"]
fn show_tasks_with_due_date_include_done() {
    let mut fix = Fixture::new();
    fix.clock.now = Local
        .ymd(2021, 04, 12)
        .and_hms(14, 00, 00)
        .with_timezone(&Utc);
    fix.test("todo new a --due 5 hours");
    fix.test("todo new b -p a");
    fix.test("todo new c -p b --due 2 days");
    fix.test("todo new d e f");
    fix.test("todo check a");
    fix.test("todo due --include-done")
        .validate()
        .printed_task(&PrintableTask::new("a", 0, Complete).due_date(DueDate {
            urgency: Moderate,
            desc: "in 5h",
        }))
        .printed_task(&PrintableTask::new("b", 1, Incomplete).due_date(
            DueDate {
                urgency: Meh,
                desc: "in 2days",
            },
        ))
        .printed_task(&PrintableTask::new("c", 5, Blocked).due_date(DueDate {
            urgency: Meh,
            desc: "in 2days",
        }))
        .end();
}

#[test]
#[ignore = "app.new.due"]
fn show_tasks_with_due_date_earlier_than_given_date() {
    let mut fix = Fixture::new();
    fix.clock.now = Local
        .ymd(2021, 04, 12)
        .and_hms(14, 00, 00)
        .with_timezone(&Utc);
    fix.test("todo new a --due 5 hours");
    fix.test("todo new b -p a");
    fix.test("todo new c -p b --due 2 days");
    fix.test("todo new d e f");
    fix.test("todo new g --due 6 hours");
    fix.test("todo due --in 1 day")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete).due_date(
            DueDate {
                urgency: Moderate,
                desc: "in 5h",
            },
        ))
        .printed_task(&PrintableTask::new("g", 2, Incomplete).due_date(
            DueDate {
                urgency: Moderate,
                desc: "in 6h",
            },
        ))
        .end();
}

#[test]
#[ignore = "app.new.due"]
fn show_tasks_with_due_date_earlier_than_given_date_include_done() {
    let mut fix = Fixture::new();
    fix.clock.now = Local
        .ymd(2021, 04, 12)
        .and_hms(14, 00, 00)
        .with_timezone(&Utc);
    fix.test("todo new a --due 5 hours");
    fix.test("todo new b -p a");
    fix.test("todo new c -p b --due 2 days");
    fix.test("todo new d e f");
    fix.test("todo new g --due 6 hours");
    fix.test("todo check g");
    fix.test("todo due --in 1 day -d")
        .validate()
        .printed_task(&PrintableTask::new("g", 0, Complete).due_date(DueDate {
            urgency: Moderate,
            desc: "in 6h",
        }))
        .printed_task(&PrintableTask::new("a", 1, Incomplete).due_date(
            DueDate {
                urgency: Moderate,
                desc: "in 5h",
            },
        ))
        .end();
}

#[test]
#[ignore = "app.new.due"]
fn show_source_of_implicit_due_date() {
    let mut fix = Fixture::new();
    fix.clock.now = Local
        .ymd(2021, 04, 12)
        .and_hms(14, 00, 00)
        .with_timezone(&Utc);
    fix.test("todo new a --due 5 hours");
    fix.test("todo new b -p a");
    fix.test("todo new c -p b --due 2 days");
    fix.test("todo new d e f --due today");
    fix.test("todo due 1")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete).due_date(
            DueDate {
                urgency: Moderate,
                desc: "in 5h",
            },
        ))
        .printed_task(&PrintableTask::new("b", 5, Blocked).due_date(DueDate {
            urgency: Meh,
            desc: "in 2days",
        }))
        .printed_task(&PrintableTask::new("c", 6, Blocked).due_date(DueDate {
            urgency: Meh,
            desc: "in 2days",
        }))
        .end();
}

#[test]
#[ignore = "app.new.due"]
fn set_due_date() {
    let mut fix = Fixture::new();
    fix.clock.now = Local
        .ymd(2021, 04, 12)
        .and_hms(14, 00, 00)
        .with_timezone(&Utc);
    fix.test("todo new a --due 5 hours");
    fix.test("todo new b -p a");
    fix.test("todo new c -p b --due 2 days");
    fix.test("todo new d e f");
    fix.test("todo due d e --on thursday")
        .validate()
        .printed_task(&PrintableTask::new("d", 2, Incomplete).due_date(
            DueDate {
                urgency: Meh,
                desc: "in 3days",
            },
        ))
        .printed_task(&PrintableTask::new("e", 5, Incomplete).due_date(
            DueDate {
                urgency: Meh,
                desc: "in 3days",
            },
        ))
        .end();
}

#[test]
#[ignore = "app.new.due"]
fn set_due_date_prints_affected_tasks() {
    let mut fix = Fixture::new();
    fix.clock.now = Local
        .ymd(2021, 04, 12)
        .and_hms(14, 00, 00)
        .with_timezone(&Utc);
    fix.test("todo new a --due 5 hours");
    fix.test("todo new b -p a");
    fix.test("todo new c -p b --due 2 days");
    fix.test("todo new d e f");
    fix.test("todo due c --in 1 hour")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete).due_date(
            DueDate {
                urgency: Moderate,
                desc: "in 1h",
            },
        ))
        .printed_task(&PrintableTask::new("b", 5, Incomplete).due_date(
            DueDate {
                urgency: Moderate,
                desc: "in 1h",
            },
        ))
        .printed_task(&PrintableTask::new("c", 6, Incomplete).due_date(
            DueDate {
                urgency: Moderate,
                desc: "in 1h",
            },
        ))
        .end();
}

#[test]
#[ignore = "app.new.due"]
fn reset_due_date() {
    let mut fix = Fixture::new();
    fix.clock.now = Local
        .ymd(2021, 04, 12)
        .and_hms(14, 00, 00)
        .with_timezone(&Utc);
    fix.test("todo new a --due 5 hours");
    fix.test("todo new b -p a");
    fix.test("todo new c -p b --due 2 days");
    fix.test("todo new d e f");
    fix.test("todo due c --none")
        .validate()
        .printed_task(&PrintableTask::new("b", 5, Blocked))
        .printed_task(&PrintableTask::new("c", 6, Blocked))
        .end();
}
