use app::testing::ymdhms;
use app::testing::Fixture;
use printing::Action::*;
use printing::BriefPrintableTask;
use printing::PrintableTask;
use printing::PrintableWarning;
use printing::Status::*;

#[test]
#[ignore = "todo-dev.app.snooze"]
fn snooze_one_task() {
    let mut fix = Fixture::new();
    fix.clock.now = ymdhms(2021, 05, 27, 11, 00, 00);
    fix.test("todo new a b");
    fix.test("todo snooze a --until 1 day")
        .validate()
        .printed_task(
            &PrintableTask::new("a", 2, Blocked)
                .start_date(ymdhms(2021, 05, 28, 00, 00, 00))
                .action(Snooze),
        )
        .end();
}

#[test]
#[ignore = "todo-dev.app.snooze"]
fn snooze_multiple_tasks() {
    let mut fix = Fixture::new();
    fix.clock.now = ymdhms(2021, 05, 27, 11, 00, 00);
    fix.test("todo new a b c d e");
    fix.test("todo snooze a c e --until saturday")
        .validate()
        .printed_task(
            &PrintableTask::new("a", 2, Blocked)
                .start_date(ymdhms(2021, 05, 29, 00, 00, 00))
                .action(Snooze),
        )
        .end();
}

#[test]
#[ignore = "todo-dev.app.snooze"]
fn snooze_snoozed_task() {
    let mut fix = Fixture::new();
    fix.clock.now = ymdhms(2021, 05, 27, 11, 00, 00);
    fix.test("todo new a --snooze 2 hours");
    fix.test("todo snooze a --until 3 hours")
        .validate()
        .printed_task(
            &PrintableTask::new("a", 2, Blocked)
                .start_date(ymdhms(2021, 05, 27, 14, 00, 00))
                .action(Snooze),
        )
        .end();
}

#[test]
#[ignore = "todo-dev.app.snooze"]
fn cannot_snooze_completed_task() {
    let mut fix = Fixture::new();
    fix.clock.now = ymdhms(2021, 05, 27, 11, 00, 00);
    fix.test("todo new a");
    fix.test("todo check a");
    fix.test("todo snooze a --until saturday")
        .validate()
        .printed_warning(&PrintableWarning::CannotSnoozeBecauseComplete {
            cannot_snooze: BriefPrintableTask::new(0, Complete),
        })
        .end();
}

#[test]
#[ignore = "todo-dev.app.snooze"]
fn snooze_blocked_task_above_layer_1() {
    let mut fix = Fixture::new();
    fix.clock.now = ymdhms(2021, 05, 27, 11, 00, 00);
    fix.test("todo new a b c --chain");
    fix.test("todo snooze c --until tomorrow")
        .validate()
        .printed_task(
            &PrintableTask::new("c", 3, Blocked)
                .start_date(ymdhms(2021, 05, 28, 00, 00, 00))
                .action(Snooze),
        )
        .end();
}