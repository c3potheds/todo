use {
    super::testing::task,
    super::testing::Fixture,
    printing::{PrintableError, Status::*},
    text_editing::FakeTextEditor,
};

#[test]
fn edit_one_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo edit 1 --desc b")
        .modified(true)
        .validate()
        .printed_task(&task("b", 1, Incomplete))
        .end();
}

#[test]
fn edit_multiple_tasks() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.test("todo edit 1 2 3 --desc d")
        .modified(true)
        .validate()
        .printed_task(&task("d", 1, Incomplete))
        .printed_task(&task("d", 2, Incomplete))
        .printed_task(&task("d", 3, Incomplete))
        .end();
}

#[test]
fn edit_with_text_editor_happy_path() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.text_editor = FakeTextEditor::user_will_enter("1) b\n");
    fix.test("todo edit 1")
        .modified(true)
        .validate()
        .printed_task(&task("b", 1, Incomplete))
        .end();
    assert_eq!(*fix.text_editor.recorded_input(), "1) a");
}

#[test]
fn edit_with_text_editor_long_desc_later_task() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.text_editor = FakeTextEditor::user_will_enter("3) this is serious\n");
    fix.test("todo edit 3")
        .modified(true)
        .validate()
        .printed_task(&task("this is serious", 3, Incomplete))
        .end();
    assert_eq!(*fix.text_editor.recorded_input(), "3) c");
}

#[test]
fn edit_multiple_tasks_with_text_editor() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.text_editor = FakeTextEditor::user_will_enter("1) d\n2) e\n3) f\n");
    fix.test("todo edit 1 2 3")
        .modified(true)
        .validate()
        .printed_task(&task("d", 1, Incomplete))
        .printed_task(&task("e", 2, Incomplete))
        .printed_task(&task("f", 3, Incomplete))
        .end();
    assert_eq!(*fix.text_editor.recorded_input(), "1) a\n2) b\n3) c");
}

#[test]
fn edit_with_text_editor_invalid_task_number() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.text_editor = FakeTextEditor::user_will_enter("2) b");
    fix.test("todo edit 1")
        .modified(false)
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
        .modified(false)
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
        .modified(false)
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
        .modified(false)
        .validate()
        .printed_error(&PrintableError::FailedToUseTextEditor)
        .end();
    assert_eq!(*fix.text_editor.recorded_input(), "1) a");
}

#[test]
fn trim_leading_whitespace_from_desc_from_text_editor() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.text_editor = FakeTextEditor::user_will_enter("1)     b");
    fix.test("todo edit 1")
        .modified(true)
        .validate()
        .printed_task(&task("b", 1, Incomplete))
        .end();
}

#[test]
fn trim_trailing_whitespace_from_desc_from_text_editor() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.text_editor = FakeTextEditor::user_will_enter("1) b     ");
    fix.test("todo edit 1")
        .modified(true)
        .validate()
        .printed_task(&task("b", 1, Incomplete))
        .end();
}

#[test]
fn trim_whitespace_from_desc_from_text_editor_with_multiple_tasks() {
    let mut fix = Fixture::default();
    fix.test("todo new a b c");
    fix.text_editor = FakeTextEditor::user_will_enter("1)  d  \n2)  e \n3) f ");
    fix.test("todo edit 1 2 3")
        .modified(true)
        .validate()
        .printed_task(&task("d", 1, Incomplete))
        .printed_task(&task("e", 2, Incomplete))
        .printed_task(&task("f", 3, Incomplete))
        .end();
}

#[test]
fn trim_leading_whitespace_from_command_line() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo edit 1 --desc '  b'")
        .modified(true)
        .validate()
        .printed_task(&task("b", 1, Incomplete))
        .end();
}

#[test]
fn trim_trailing_whitespace_from_command_line() {
    let mut fix = Fixture::default();
    fix.test("todo new a");
    fix.test("todo edit 1 --desc 'b  '")
        .modified(true)
        .validate()
        .printed_task(&task("b", 1, Incomplete))
        .end();
}

#[test]
fn rename_tag_prints_affected_tasks() {
    let mut fix = Fixture::default();
    fix.test("todo new a b");
    fix.test("todo new x --tag -p a b");
    fix.test("todo edit x --desc y")
        .modified(true)
        .validate()
        .printed_task(&task("a", 1, Incomplete).tag("y").adeps_stats(0, 1))
        .printed_task(&task("b", 2, Incomplete).tag("y").adeps_stats(0, 1))
        .printed_task(&task("y", 3, Blocked).as_tag().deps_stats(2, 2))
        .end();
}

#[test]
fn rename_tag_does_not_print_complete_affected_tasks() {
    let mut fix = Fixture::default();
    fix.test("todo new a b");
    fix.test("todo new x --tag -p a b");
    fix.test("todo check a");
    fix.test("todo edit x --desc y")
        .modified(true)
        .validate()
        .printed_task(&task("b", 1, Incomplete).tag("y").adeps_stats(1, 1))
        .printed_task(&task("y", 2, Blocked).as_tag().deps_stats(1, 2))
        .end();
}

#[test]
fn rename_tag_prints_complete_affected_tasks_with_done_flag() {
    let mut fix = Fixture::default();
    fix.test("todo new a b");
    fix.test("todo new x --tag -p a b");
    fix.test("todo check a");
    fix.test("todo edit x --desc y --include-done")
        .modified(true)
        .validate()
        .printed_task(&task("a", 0, Complete).tag("y"))
        .printed_task(&task("b", 1, Incomplete).tag("y").adeps_stats(1, 1))
        .printed_task(&task("y", 2, Blocked).as_tag().deps_stats(1, 2))
        .end();
}

#[test]
fn rename_tag_with_text_editor() {
    let mut fix = Fixture::default();
    fix.test("todo new a b");
    fix.test("todo new x --tag -p a b");
    fix.text_editor = FakeTextEditor::user_will_enter("3) y");
    fix.test("todo edit x")
        .modified(true)
        .validate()
        .printed_task(&task("a", 1, Incomplete).tag("y").adeps_stats(0, 1))
        .printed_task(&task("b", 2, Incomplete).tag("y").adeps_stats(0, 1))
        .printed_task(&task("y", 3, Blocked).as_tag().deps_stats(2, 2))
        .end();
}

#[test]
fn rename_tag_with_text_editor_and_multiple_tasks() {
    let mut fix = Fixture::default();
    fix.test("todo new a b");
    fix.test("todo new x y --tag -p a b");
    fix.text_editor = FakeTextEditor::user_will_enter("3) y\n4) z");
    fix.test("todo edit x y")
        .modified(true)
        .validate()
        .printed_task(
            &task("a", 1, Incomplete).tag("y").tag("z").adeps_stats(0, 2),
        )
        .printed_task(
            &task("b", 2, Incomplete).tag("y").tag("z").adeps_stats(0, 2),
        )
        .printed_task(&task("y", 3, Blocked).as_tag().deps_stats(2, 2))
        .printed_task(&task("z", 4, Blocked).as_tag().deps_stats(2, 2))
        .end();
}
