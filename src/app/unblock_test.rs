use app::testing::Fixture;
use cli::Key;
use printing::Action::*;
use printing::BriefPrintableTask;
use printing::PrintableError;
use printing::PrintableTask;
use printing::PrintableWarning;
use printing::Status::*;

#[test]
fn unblock_task_from_direct_dependency() {
    let mut fix = Fixture::default();
    fix.test("todo new a b");
    fix.test("todo block 2 --on 1");
    fix.test("todo unblock 2 --from 1")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .printed_task(&PrintableTask::new("b", 2, Incomplete).action(Unlock))
        .end();
}

#[test]
fn unblock_task_from_indirect_dependency() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo block 3 --on 2");
    fix.test("todo block 2 --on 1");
    fix.test("todo unblock 3 --from 1")
        .validate()
        .printed_warning(
            &PrintableWarning::CannotUnblockBecauseTaskIsNotBlocked {
                cannot_unblock: BriefPrintableTask::new(3, Blocked),
                requested_unblock_from: BriefPrintableTask::new(1, Incomplete),
            },
        )
        .end();
}

#[test]
fn unblock_complete_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a b --chain");
    fix.test("todo check 1 2");
    fix.test("todo unblock 0 --from -1")
        .validate()
        .printed_task(&PrintableTask::new("a", -1, Complete))
        .printed_task(&PrintableTask::new("b", 0, Complete).action(Unlock))
        .end();
}

#[test]
fn unblock_by_name() {
    let mut fix = Fixture::default();
    fix.test("todo new a b --chain");
    fix.test("todo unblock b --from a")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .printed_task(&PrintableTask::new("b", 2, Incomplete).action(Unlock))
        .end();
}

#[test]
fn unblock_from_all() {
    let mut fix = Fixture::default();
    fix.test("todo new a b");
    fix.test("todo new c -p a b");
    fix.test("todo unblock c")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .printed_task(&PrintableTask::new("b", 2, Incomplete))
        .printed_task(&PrintableTask::new("c", 3, Incomplete).action(Unlock))
        .end();
}

#[test]
fn unblock_from_all2() {
    let mut fix = Fixture::default();
    fix.test("todo new a b --chain");
    fix.test("todo new c -p a b");
    fix.test("todo unblock c")
        .validate()
        .printed_task(&PrintableTask::new("a", 1, Incomplete))
        .printed_task(&PrintableTask::new("c", 2, Incomplete).action(Unlock))
        .printed_task(&PrintableTask::new("b", 3, Blocked))
        .end();
}

#[test]
fn unblock_complete() {
    let mut fix = Fixture::default();
    fix.test("todo new a b --chain");
    fix.test("todo check a b");
    fix.test("todo unblock b")
        .validate()
        .printed_task(&PrintableTask::new("a", -1, Complete))
        .printed_task(&PrintableTask::new("b", 0, Complete).action(Unlock))
        .end();
}

#[test]
fn unblock_from_matchless_key_is_error() {
    let mut fix = Fixture::default();
    fix.test("todo new a b --chain");
    fix.test("todo unblock b --from c")
        .validate()
        .printed_error(&PrintableError::NoMatchForKeys {
            keys: vec![Key::ByName("c".to_string())],
        })
        .end();
}

#[test]
fn unblock_updates_priority() {
    let mut fix = Fixture::default();
    fix.test("todo new a b --chain --priority 1");
    fix.test("Todo new c --priority 2");
    fix.test("todo block c --on b");
    fix.test("todo unblock c --from b")
        .validate()
        // c is printed first, because its priority is higher.
        .printed_task(
            &PrintableTask::new("c", 1, Incomplete)
                .action(Unlock)
                .priority(2),
        )
        // a and b have their priorities reset to 1.
        .printed_task(&PrintableTask::new("a", 2, Incomplete).priority(1))
        .printed_task(&PrintableTask::new("b", 3, Blocked).priority(1))
        .end();
}

#[test]
fn unblock_does_not_show_unaffected_priority() {
    let mut fix = Fixture::default();
    fix.test("todo new a b --chain --priority 1");
    fix.test("Todo new c --priority 1");
    fix.test("todo block c --on b");
    fix.test("todo unblock c --from b")
        .validate()
        .printed_task(
            &PrintableTask::new("c", 2, Incomplete)
                .action(Unlock)
                .priority(1),
        )
        .printed_task(&PrintableTask::new("b", 3, Blocked).priority(1))
        .end();
}

#[test]
fn unblock_excludes_affected_complete_tasks() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo priority c --is 1");
    fix.test("todo check a");
    fix.test("todo unblock c --from b")
        .validate()
        .printed_task(
            &PrintableTask::new("c", 1, Incomplete)
                .priority(1)
                .action(Unlock),
        )
        .printed_task(&PrintableTask::new("b", 2, Incomplete))
        .end();
}

#[test]
fn unblock_include_done() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c --chain");
    fix.test("todo priority c --is 1");
    fix.test("todo check a");
    fix.test("todo unblock c --from b -d")
        .validate()
        .printed_task(&PrintableTask::new("a", 0, Complete))
        .printed_task(
            &PrintableTask::new("c", 1, Incomplete)
                .priority(1)
                .action(Unlock),
        )
        .printed_task(&PrintableTask::new("b", 2, Incomplete))
        .end();
}
