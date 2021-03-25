use app::testing::*;
use model::TaskStatus;
use model::TodoList;
use printing::Action;
use printing::Expect;
use printing::PrintableError;
use text_editing::FakeTextEditor;

#[test]
fn edit_one_task() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a"]);
    test(&mut list, &["todo", "edit", "1", "--desc", "b"])
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
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b", "c"]);
    test(&mut list, &["todo", "edit", "1", "2", "3", "--desc", "d"])
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
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a"]);
    let text_editor = FakeTextEditor::user_will_enter("1) b\n");
    test_with_text_editor(&mut list, &text_editor, &["todo", "edit", "1"])
        .validate()
        .printed_task(&[
            Expect::Desc("b"),
            Expect::Number(1),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::None),
        ])
        .end();
    assert_eq!(*text_editor.recorded_input(), "1) a");
}

#[test]
fn edit_with_text_editor_long_desc_later_task() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b", "c"]);
    let text_editor = FakeTextEditor::user_will_enter("3) this is serious\n");
    test_with_text_editor(&mut list, &text_editor, &["todo", "edit", "3"])
        .validate()
        .printed_task(&[
            Expect::Desc("this is serious"),
            Expect::Number(3),
            Expect::Status(TaskStatus::Incomplete),
            Expect::Action(Action::None),
        ])
        .end();
    assert_eq!(*text_editor.recorded_input(), "3) c");
}

#[test]
fn edit_multiple_tasks_with_text_editor() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b", "c"]);
    let text_editor = FakeTextEditor::user_will_enter("1) d\n2) e\n3) f\n");
    test_with_text_editor(
        &mut list,
        &text_editor,
        &["todo", "edit", "1", "2", "3"],
    )
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
    assert_eq!(*text_editor.recorded_input(), "1) a\n2) b\n3) c");
}

#[test]
fn edit_with_text_editor_invalid_task_number() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a"]);
    let text_editor = FakeTextEditor::user_will_enter("2) b");
    test_with_text_editor(&mut list, &text_editor, &["todo", "edit", "1"])
        .validate()
        .printed_error(&PrintableError::CannotEditBecauseNoTaskWithNumber {
            requested: 2,
        })
        .end();
    assert_eq!(*text_editor.recorded_input(), "1) a");
}

#[test]
fn edit_with_text_editor_unexpected_task_number() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a", "b"]);
    let text_editor = FakeTextEditor::user_will_enter("2) c");
    test_with_text_editor(&mut list, &text_editor, &["todo", "edit", "1"])
        .validate()
        .printed_error(&PrintableError::CannotEditBecauseUnexpectedNumber {
            requested: 2,
        })
        .end();
    assert_eq!(*text_editor.recorded_input(), "1) a");
}

#[test]
fn edit_with_text_editor_empty_description() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a"]);
    let text_editor = FakeTextEditor::user_will_enter("1)");
    test_with_text_editor(&mut list, &text_editor, &["todo", "edit", "1"])
        .validate()
        .printed_error(&PrintableError::CannotEditBecauseInvalidLine {
            malformed_line: "1)".to_string(),
        })
        .end();
    assert_eq!(*text_editor.recorded_input(), "1) a");
}

#[test]
fn edit_with_text_editor_remove_delimiter() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a"]);
    let text_editor = FakeTextEditor::user_will_enter("1 b");
    test_with_text_editor(&mut list, &text_editor, &["todo", "edit", "1"])
        .validate()
        .printed_error(&PrintableError::CannotEditBecauseInvalidLine {
            malformed_line: "1 b".to_string(),
        })
        .end();
    assert_eq!(*text_editor.recorded_input(), "1) a");
}

#[test]
fn edit_with_text_editor_text_editor_fails() {
    let mut list = TodoList::new();
    test(&mut list, &["todo", "new", "a"]);
    let text_editor = FakeTextEditor::no_user_output();
    test_with_text_editor(&mut list, &text_editor, &["todo", "edit", "1"])
        .validate()
        .printed_error(&PrintableError::FailedToUseTextEditor)
        .end();
    assert_eq!(*text_editor.recorded_input(), "1) a");
}
