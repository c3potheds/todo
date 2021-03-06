use {
    super::testing::Fixture,
    printing::{PrintableError, PrintableTask, Status::*},
    text_editing::FakeTextEditor,
};

#[test]
fn edit_one_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo edit 1 --desc b")
        .validate()
        .printed_task(&PrintableTask::new("b", 1, Incomplete))
        .end();
}

#[test]
fn edit_multiple_tasks() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo edit 1 2 3 --desc d")
        .validate()
        .printed_task(&PrintableTask::new("d", 1, Incomplete))
        .printed_task(&PrintableTask::new("d", 2, Incomplete))
        .printed_task(&PrintableTask::new("d", 3, Incomplete))
        .end();
}

#[test]
fn edit_with_text_editor_happy_path() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.text_editor = FakeTextEditor::user_will_enter("1) b\n");
    fix.test("todo edit 1")
        .validate()
        .printed_task(&PrintableTask::new("b", 1, Incomplete))
        .end();
    assert_eq!(*fix.text_editor.recorded_input(), "1) a");
}

#[test]
fn edit_with_text_editor_long_desc_later_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.text_editor = FakeTextEditor::user_will_enter("3) this is serious\n");
    fix.test("todo edit 3")
        .validate()
        .printed_task(&PrintableTask::new("this is serious", 3, Incomplete))
        .end();
    assert_eq!(*fix.text_editor.recorded_input(), "3) c");
}

#[test]
fn edit_multiple_tasks_with_text_editor() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.text_editor = FakeTextEditor::user_will_enter("1) d\n2) e\n3) f\n");
    fix.test("todo edit 1 2 3")
        .validate()
        .printed_task(&PrintableTask::new("d", 1, Incomplete))
        .printed_task(&PrintableTask::new("e", 2, Incomplete))
        .printed_task(&PrintableTask::new("f", 3, Incomplete))
        .end();
    assert_eq!(*fix.text_editor.recorded_input(), "1) a\n2) b\n3) c");
}

#[test]
fn edit_with_text_editor_invalid_task_number() {
    let mut fix = Fixture::default();
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
    let mut fix = Fixture::default();
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
    let mut fix = Fixture::default();
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
    let mut fix = Fixture::default();
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
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo edit 1")
        .validate()
        .printed_error(&PrintableError::FailedToUseTextEditor)
        .end();
    assert_eq!(*fix.text_editor.recorded_input(), "1) a");
}
