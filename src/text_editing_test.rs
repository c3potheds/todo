use text_editing::*;

#[test]
fn fake_text_editor_prints_user_output() {
    let text_editor = FakeTextEditor::user_will_enter("blah");
    assert_eq!(*text_editor.recorded_input(), "");
    assert_eq!(text_editor.edit_text("default").unwrap(), "blah");
    assert_eq!(*text_editor.recorded_input(), "default");
}
