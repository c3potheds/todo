use app::testing::Fixture;
use model::TaskStatus;
use printing::Action;
use printing::Expect;
use printing::PrintableError;
use text_editing::FakeTextEditor;

#[test]
fn edit_one_task() {
    let mut fix = Fixture::new();
    fix.test("todo new a");
    fix.test("todo edit 1 --desc b")
        .validate()
        .printed_task(&[
            Expect::Desc("b"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::None),
        ])
        .end();
}

#[test]
fn edit_multiple_tasks() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c");
    fix.test("todo edit 1 2 3 --desc d")
        .validate()
        .printed_task(&[
            Expect::Desc("d"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::None),
        ])
        .printed_task(&[
            Expect::Desc("d"),
            Expect::Number(2),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::None),
        ])
        .printed_task(&[
            Expect::Desc("d"),
            Expect::Number(3),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::None),
        ])
        .end();
}

#[test]
fn edit_with_text_editor_happy_path() {
    let mut fix = Fixture::new();
    fix.test("todo new a");
    fix.text_editor = FakeTextEditor::user_will_enter("1) b\n");
    fix.test("todo edit 1")
        .validate()
        .printed_task(&[
            Expect::Desc("b"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::None),
        ])
        .end();
    assert_eq!(*fix.text_editor.recorded_input(), "1) a");
}

#[test]
fn edit_with_text_editor_long_desc_later_task() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c");
    fix.text_editor = FakeTextEditor::user_will_enter("3) this is serious\n");
    fix.test("todo edit 3")
        .validate()
        .printed_task(&[
            Expect::Desc("this is serious"),
            Expect::Number(3),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::None),
        ])
        .end();
    assert_eq!(*fix.text_editor.recorded_input(), "3) c");
}

#[test]
fn edit_multiple_tasks_with_text_editor() {
    let mut fix = Fixture::new();
    fix.test("todo new a b c");
    fix.text_editor = FakeTextEditor::user_will_enter("1) d\n2) e\n3) f\n");
    fix.test("todo edit 1 2 3")
        .validate()
        .printed_task(&[
            Expect::Desc("d"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::None),
        ])
        .printed_task(&[
            Expect::Desc("e"),
            Expect::Number(2),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::None),
        ])
        .printed_task(&[
            Expect::Desc("f"),
            Expect::Number(3),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::None),
        ])
        .end();
    assert_eq!(*fix.text_editor.recorded_input(), "1) a\n2) b\n3) c");
}

#[test]
fn edit_with_text_editor_invalid_task_number() {
    let mut fix = Fixture::new();
    fix.test("todo new a");
    fix.text_editor = FakeTextEditor::user_will_enter("2) b");
    fix.test("todo edit 1")
        .validate()
        .printed_error(&PrintableError::CannotEditBecauseNoTaskWithNumber {
            requested: 2,
        })
        .end();
    assert_eq!(*fix.text_editor.recorded_input(), "1) a");
}

#[test]
fn edit_with_text_editor_unexpected_task_number() {
    let mut fix = Fixture::new();
    fix.test("todo new a b");
    fix.text_editor = FakeTextEditor::user_will_enter("2) c");
    fix.test("todo edit 1")
        .validate()
        .printed_error(&PrintableError::CannotEditBecauseUnexpectedNumber {
            requested: 2,
        })
        .end();
    assert_eq!(*fix.text_editor.recorded_input(), "1) a");
}

#[test]
fn edit_with_text_editor_empty_description() {
    let mut fix = Fixture::new();
    fix.test("todo new a");
    fix.text_editor = FakeTextEditor::user_will_enter("1)");
    fix.test("todo edit 1")
        .validate()
        .printed_error(&PrintableError::CannotEditBecauseInvalidLine {
            malformed_line: "1)".to_string(),
        })
        .end();
    assert_eq!(*fix.text_editor.recorded_input(), "1) a");
}

#[test]
fn edit_with_text_editor_remove_delimiter() {
    let mut fix = Fixture::new();
    fix.test("todo new a");
    fix.text_editor = FakeTextEditor::user_will_enter("1 b");
    fix.test("todo edit 1")
        .validate()
        .printed_error(&PrintableError::CannotEditBecauseInvalidLine {
            malformed_line: "1 b".to_string(),
        })
        .end();
    assert_eq!(*fix.text_editor.recorded_input(), "1) a");
}

#[test]
fn edit_with_text_editor_text_editor_fails() {
    let mut fix = Fixture::new();
    fix.test("todo new a");
    fix.test("todo edit 1")
        .validate()
        .printed_error(&PrintableError::FailedToUseTextEditor)
        .end();
    assert_eq!(*fix.text_editor.recorded_input(), "1) a");
}
