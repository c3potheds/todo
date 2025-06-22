// app/src/interactive/tests.rs
#![cfg(test)]

use super::test_utils::{TestHarness, SimulatedInput}; // Use super since tests.rs is sibling to test_utils.rs
use super::app_state::ViewMode; // For asserting ViewMode
// We might need TaskId if we want to assert specific tasks are selected by ID
use todo_model::{TaskId, TaskStatus, Task}; // For checking task properties
use chrono::{Duration, Utc}; // For time-based assertions if needed for snooze/due


// --- Navigation Tests ---
#[test]
fn test_cursor_movement_empty_list() {
    let mut harness = TestHarness::new(None);
    harness.assert_cursor_at(0, "Initial cursor on empty list");
    harness.input(&SimulatedInput::Down);
    harness.assert_cursor_at(0, "Cursor down on empty list");
    harness.input(&SimulatedInput::Up);
    harness.assert_cursor_at(0, "Cursor up on empty list");
}

#[test]
fn test_cursor_movement_single_task() {
    let mut harness = TestHarness::new(None);
    harness.add_task_direct("Task 1");
    harness.assert_cursor_at(0, "Initial cursor on single task");
    harness.input(&SimulatedInput::Down);
    harness.assert_cursor_at(0, "Cursor down on single task");
    harness.input(&SimulatedInput::Up);
    harness.assert_cursor_at(0, "Cursor up on single task");
}

#[test]
fn test_cursor_movement_multiple_tasks() {
    let mut harness = TestHarness::new(None);
    harness.add_task_direct("Task 1");
    harness.add_task_direct("Task 2");
    harness.add_task_direct("Task 3");

    harness.assert_cursor_at(0, "Initial cursor");

    harness.input(&SimulatedInput::Down); // Cursor to 1 (Task 2)
    harness.assert_cursor_at(1, "Cursor moved to index 1");
    harness.assert_task_desc_at_cursor(Some("Task 2"), "Task at cursor is Task 2");


    harness.input(&SimulatedInput::Down); // Cursor to 2 (Task 3)
    harness.assert_cursor_at(2, "Cursor moved to index 2");
    harness.assert_task_desc_at_cursor(Some("Task 3"), "Task at cursor is Task 3");


    harness.input(&SimulatedInput::Down); // Cursor stays at 2 (bottom)
    harness.assert_cursor_at(2, "Cursor stays at bottom");

    harness.input(&SimulatedInput::Up); // Cursor to 1 (Task 2)
    harness.assert_cursor_at(1, "Cursor moved up to index 1");

    harness.input(&SimulatedInput::Up); // Cursor to 0 (Task 1)
    harness.assert_cursor_at(0, "Cursor moved up to index 0");
    harness.assert_task_desc_at_cursor(Some("Task 1"), "Task at cursor is Task 1");

    harness.input(&SimulatedInput::Up); // Cursor stays at 0 (top)
    harness.assert_cursor_at(0, "Cursor stays at top");
}

// --- Selection Tests ---
#[test]
fn test_single_task_selection() {
    let mut harness = TestHarness::new(None);
    let t1_id = harness.add_task_direct("Task 1");
    harness.add_task_direct("Task 2");

    // Select Task 1
    harness.input(&SimulatedInput::Space);
    harness.assert_selection_contains(t1_id, "Task 1 selected");
    assert_eq!(harness.app_state.selected_tasks.len(), 1, "Only one task selected");

    // Deselect Task 1
    harness.input(&SimulatedInput::Space);
    harness.assert_selection_is_empty("Task 1 deselected");
}

#[test]
fn test_multiple_task_selection() {
    let mut harness = TestHarness::new(None);
    let t1_id = harness.add_task_direct("Task 1");
    let t2_id = harness.add_task_direct("Task 2");
    harness.add_task_direct("Task 3");

    // Select Task 1
    harness.input(&SimulatedInput::Space);
    harness.assert_selection_contains(t1_id, "Task 1 selected");

    // Move to Task 2 and select it
    harness.input(&SimulatedInput::Down);
    harness.input(&SimulatedInput::Space);
    harness.assert_selection_contains(t1_id, "Task 1 still selected");
    harness.assert_selection_contains(t2_id, "Task 2 also selected");
    assert_eq!(harness.app_state.selected_tasks.len(), 2, "Two tasks selected");
}

#[test]
fn test_range_selection_downwards() {
    let mut harness = TestHarness::new(None);
    let t1_id = harness.add_task_direct("Task 1"); // idx 0
    let t2_id = harness.add_task_direct("Task 2"); // idx 1
    let t3_id = harness.add_task_direct("Task 3"); // idx 2
    harness.add_task_direct("Task 4");       // idx 3

    // Select Task 2 (index 1)
    harness.input(&SimulatedInput::Down); // cursor to index 1
    harness.input(&SimulatedInput::Space); // select task at index 1 (t2_id)
    harness.assert_selection_contains(t2_id, "Task 2 (idx 1) selected initially");

    // Move to Task 4 (index 3)
    harness.input(&SimulatedInput::Down); // cursor to index 2
    harness.input(&SimulatedInput::Down); // cursor to index 3

    // Shift+Space to select from last selected (idx 1) to current (idx 3)
    harness.input(&SimulatedInput::ShiftSpace);

    harness.assert_selection_contains(t2_id, "Task 2 (idx 1) should be selected in range");
    harness.assert_selection_contains(t3_id, "Task 3 (idx 2) should be selected in range");
    let t4_id = harness.app_state.displayed_tasks[3]; // Get TaskId for task at index 3
    harness.assert_selection_contains(t4_id, "Task 4 (idx 3) should be selected in range");
    assert!(!harness.app_state.selected_tasks.contains(&t1_id), "Task 1 (idx 0) should NOT be selected");
    assert_eq!(harness.app_state.selected_tasks.len(), 3, "Range selection downwards: 3 tasks selected");
}

#[test]
fn test_range_selection_upwards() {
    let mut harness = TestHarness::new(None);
    let t1_id = harness.add_task_direct("Task 1"); // idx 0
    let t2_id = harness.add_task_direct("Task 2"); // idx 1
    let t3_id = harness.add_task_direct("Task 3"); // idx 2
    let t4_id = harness.add_task_direct("Task 4"); // idx 3

    // Move to Task 4 (index 3) and select it
    harness.inputs(&[SimulatedInput::Down, SimulatedInput::Down, SimulatedInput::Down, SimulatedInput::Space]);
    harness.assert_selection_contains(t4_id, "Task 4 (idx 3) selected initially");

    // Move to Task 2 (index 1)
    harness.inputs(&[SimulatedInput::Up, SimulatedInput::Up]);

    // Shift+Space to select from last selected (idx 3) to current (idx 1)
    harness.input(&SimulatedInput::ShiftSpace);

    harness.assert_selection_contains(t2_id, "Task 2 (idx 1) should be selected in range");
    harness.assert_selection_contains(t3_id, "Task 3 (idx 2) should be selected in range");
    harness.assert_selection_contains(t4_id, "Task 4 (idx 3) should be selected in range");
    assert!(!harness.app_state.selected_tasks.contains(&t1_id), "Task 1 (idx 0) should NOT be selected");
    assert_eq!(harness.app_state.selected_tasks.len(), 3, "Range selection upwards: 3 tasks selected");
}

#[test]
fn test_range_selection_no_initial_for_shift_select() {
    let mut harness = TestHarness::new(None);
    let t1_id = harness.add_task_direct("Task 1");
    // Cursor is at index 0 (Task 1)
    // No prior selection for shift-select (last_selected_task_index_for_shift_select is None)
    harness.input(&SimulatedInput::ShiftSpace);
    harness.assert_selection_contains(t1_id, "Task 1 selected by ShiftSpace with no prior anchor");
    assert_eq!(harness.app_state.selected_tasks.len(), 1, "Only current task selected");
}

#[test]
fn test_clear_selection_with_esc() {
    let mut harness = TestHarness::new(None);
    harness.add_task_direct("Task 1");
    harness.input(&SimulatedInput::Space); // Select Task 1
    assert!(!harness.app_state.selected_tasks.is_empty(), "Selection should not be empty");

    harness.input(&SimulatedInput::Esc);
    harness.assert_selection_is_empty("Selection cleared by Esc");
}

// --- Basic Mode Change Tests ---
#[test]
fn test_enter_editing_new_task_mode() {
    let mut harness = TestHarness::new(None);
    harness.input(&SimulatedInput::Char('n'));
    harness.assert_mode_is(ViewMode::EditingNewTask, "Mode changed to EditingNewTask");
    harness.assert_input_buffer_is("", "Input buffer empty on entering mode");
}

#[test]
fn test_exit_editing_new_task_mode_with_esc() {
    let mut harness = TestHarness::new(None);
    harness.input(&SimulatedInput::Char('n')); // Enter mode
    harness.assert_mode_is(ViewMode::EditingNewTask, "In EditingNewTask mode");

    harness.input(&SimulatedInput::Esc); // Exit mode
    harness.assert_mode_is(ViewMode::Default, "Mode changed back to Default");
    harness.assert_input_buffer_is("", "Input buffer cleared on Esc"); // AppState.change_mode clears buffer
}

#[test]
fn test_enter_other_input_modes() {
    let mut harness = TestHarness::new(None);

    // Snooze Input Mode
    harness.input(&SimulatedInput::Char('s'));
    harness.assert_mode_is(ViewMode::SnoozeInput, "Mode changed to SnoozeInput");
    harness.input(&SimulatedInput::Esc); // Back to default
    harness.assert_mode_is(ViewMode::Default, "Mode back to Default from SnoozeInput");

    // Priority Input Mode
    harness.input(&SimulatedInput::Char('P'));
    harness.assert_mode_is(ViewMode::PriorityInput, "Mode changed to PriorityInput");
    harness.input(&SimulatedInput::Esc); // Back to default
    harness.assert_mode_is(ViewMode::Default, "Mode back to Default from PriorityInput");

    // Command Input Mode
    harness.input(&SimulatedInput::Char(':'));
    harness.assert_mode_is(ViewMode::CommandInput, "Mode changed to CommandInput");
    harness.input(&SimulatedInput::Esc); // Back to default
    harness.assert_mode_is(ViewMode::Default, "Mode back to Default from CommandInput");
}

// --- Action Tests: New Task ---
#[test]
fn test_create_new_task() {
    let mut harness = TestHarness::new(None);
    harness.inputs(&[
        SimulatedInput::Char('n'), // Enter NewTask mode
        SimulatedInput::TypeString("A new test task".to_string()),
        SimulatedInput::Enter,
    ]);
    harness.assert_mode_is(ViewMode::Default, "Back to default mode after adding task");
    harness.assert_input_buffer_is("", "Input buffer cleared");
    assert_eq!(harness.todo_list.incomplete_tasks().count(), 1, "One task should be added");
    let task_id = harness.todo_list.incomplete_tasks().next().unwrap();
    assert_eq!(harness.todo_list.get(task_id).unwrap().desc, "A new test task");
    harness.assert_status_message_contains("New task added", "Status message for new task");
}

#[test]
fn test_create_new_task_empty_description() {
    let mut harness = TestHarness::new(None);
    harness.inputs(&[
        SimulatedInput::Char('n'),
        SimulatedInput::Enter,
    ]);
    harness.assert_mode_is(ViewMode::Default, "Still in default mode"); // or EditingNewTask depending on exact behavior
    assert_eq!(harness.todo_list.incomplete_tasks().count(), 0, "No task should be added");
    harness.assert_status_message_contains("Cannot add empty task", "Status for empty task desc");
}

// --- Action Tests: Check/Restore ---
#[test]
fn test_check_task() {
    let mut harness = TestHarness::new(None);
    let t1_id = harness.add_task_direct("Task to check");
    harness.select_task_at_index(0); // Select the task

    harness.input(&SimulatedInput::Char('c')); // Check action

    assert_eq!(harness.todo_list.status(t1_id), Some(TaskStatus::Complete), "Task should be complete");
    harness.assert_status_message_contains("Checked 1 tasks", "Status for check");
    // Selection should persist as per requirements
    harness.assert_selection_contains(t1_id, "Task should remain selected after check");
}

#[test]
fn test_restore_task() {
    let mut harness = TestHarness::new(None);
    let t1_id = harness.add_task_direct("Task to restore");
    harness.todo_list.force_check(t1_id.into()).unwrap(); // Pre-complete the task
    harness.app_state.update_displayed_tasks(&harness.todo_list); // Refresh displayed list

    // Need to select it from the potentially new position if displayed_tasks changes after check
    // For simplicity, assuming it's still at index 0 or we re-find it.
    // If 'log' or completed tasks view is not default, this test needs adjustment.
    // For now, let's assume it is NOT displayed in default incomplete view.
    // So we'll add it to selection manually for testing restore action.
    harness.app_state.selected_tasks.insert(t1_id);


    harness.input(&SimulatedInput::Char('r')); // Restore action

    assert_eq!(harness.todo_list.status(t1_id), Some(TaskStatus::Incomplete), "Task should be incomplete after restore");
    harness.assert_status_message_contains("Restored 1 tasks", "Status for restore");
    harness.assert_selection_contains(t1_id, "Task should remain selected after restore");
}

#[test]
fn test_check_no_selection() {
    let mut harness = TestHarness::new(None);
    harness.add_task_direct("A task");
    harness.input(&SimulatedInput::Char('c'));
    harness.assert_status_message_contains("No tasks selected", "Status for check with no selection");
}

// --- Action Tests: Snooze ---
#[test]
fn test_snooze_task_valid_duration() {
    let mut harness = TestHarness::new(None);
    let t1_id = harness.add_task_direct("Task to snooze");
    harness.select_task_at_index(0);

    let before_snooze_start_date = harness.todo_list.get(t1_id).unwrap().start_date;

    harness.inputs(&[
        SimulatedInput::Char('s'), // Enter SnoozeInput mode
        SimulatedInput::TypeString("1d".to_string()),
        SimulatedInput::Enter,
    ]);

    harness.assert_mode_is(ViewMode::Default, "Back to default after snoozing");
    let after_snooze_start_date = harness.todo_list.get(t1_id).unwrap().start_date;
    assert!(after_snooze_start_date > before_snooze_start_date, "Task start_date should be updated");
    // More precise check: before_snooze_start_date + Duration::days(1) - some_epsilon < after_snooze_start_date < ...
    // For now, greater than is a good start.
    harness.assert_status_message_contains("Attempted to snooze 1 tasks", "Status for snooze");
}

#[test]
fn test_snooze_task_invalid_format() {
    let mut harness = TestHarness::new(None);
    let t1_id = harness.add_task_direct("Task to snooze badly");
    harness.select_task_at_index(0);
    let original_start_date = harness.todo_list.get(t1_id).unwrap().start_date;

    harness.inputs(&[
        SimulatedInput::Char('s'),
        SimulatedInput::TypeString("invalid_snooze_text".to_string()),
        SimulatedInput::Enter,
    ]);

    harness.assert_mode_is(ViewMode::Default, "Back to default after failed snooze");
    harness.assert_status_message_contains("Invalid snooze format", "Status for invalid snooze");
    let current_start_date = harness.todo_list.get(t1_id).unwrap().start_date;
    assert_eq!(current_start_date, original_start_date, "Task start_date should not change on invalid format");
}

// --- Action Tests: Priority & Due Date (via command input for now) ---
#[test]
fn test_set_priority_valid() {
    let mut harness = TestHarness::new(None);
    let t1_id = harness.add_task_direct("Task for priority");
    harness.select_task_at_index(0);

    harness.inputs(&[
        SimulatedInput::Char('P'), // PriorityInput mode
        SimulatedInput::TypeString("5".to_string()),
        SimulatedInput::Enter,
    ]);
    harness.assert_mode_is(ViewMode::Default, "Back to default after setting priority");
    assert_eq!(harness.todo_list.get(t1_id).unwrap().priority, 5, "Priority should be set to 5");
    harness.assert_status_message_contains("Set priority 5 for 1 tasks", "Status for priority set");
}

#[test]
fn test_set_due_date_valid_from_command() {
    let mut harness = TestHarness::new(None);
    let t1_id = harness.add_task_direct("Task for due date");
    harness.select_task_at_index(0);

    let current_time = Utc::now();
    let expected_due_date_approx = current_time + Duration::days(1);


    harness.inputs(&[
        SimulatedInput::Char(':'), // CommandInput mode
        SimulatedInput::TypeString("due tomorrow".to_string()),
        SimulatedInput::Enter,
    ]);
    harness.assert_mode_is(ViewMode::Default, "Back to default after setting due date");
    let task_due_date = harness.todo_list.get(t1_id).unwrap().due_date;
    assert!(task_due_date.is_some(), "Due date should be set");
    // Check if it's approximately tomorrow (within a few seconds tolerance for test execution)
    assert!((task_due_date.unwrap() - expected_due_date_approx).num_seconds().abs() < 60, "Due date not approx tomorrow");
    harness.assert_status_message_contains("Set due date for 1 tasks", "Status for due date set");
}

// --- Action Tests: Get Mode ---
#[test]
fn test_get_mode_enter_and_display() {
    let mut harness = TestHarness::new(None);
    let t1 = harness.add_task_direct("T1"); // Dep of T2
    let t2 = harness.add_task_direct("T2"); // Selected, Adep of T1, Dep of T3
    let t3 = harness.add_task_direct("T3"); // Adep of T2
    harness.todo_list.block(t2).on(t1).unwrap();
    harness.todo_list.block(t3).on(t2).unwrap();
    harness.app_state.update_displayed_tasks(&harness.todo_list); // Refresh displayed tasks

    // Select T2 (assuming it's at index 1 in default incomplete view if T1 is also incomplete)
    // Or find its index:
    let t2_index = harness.app_state.displayed_tasks.iter().position(|&id| id == t2).expect("T2 not found in displayed tasks");
    harness.app_state.cursor_index = t2_index;
    harness.select_task_at_index(t2_index);

    harness.input(&SimulatedInput::Char('g')); // Enter GetMode

    harness.assert_mode_is(ViewMode::GetMode, "Should be in GetMode");
    harness.assert_status_message_contains("Showing 3 related tasks", "Get mode status message");

    // Check displayed tasks in GetMode (order might vary, so check presence)
    let displayed_ids_in_get: std::collections::HashSet<TaskId> = harness.app_state.displayed_tasks.iter().cloned().collect();
    assert!(displayed_ids_in_get.contains(&t1), "GetMode should display T1");
    assert!(displayed_ids_in_get.contains(&t2), "GetMode should display T2");
    assert!(displayed_ids_in_get.contains(&t3), "GetMode should display T3");
    assert_eq!(displayed_ids_in_get.len(), 3, "Should display exactly 3 tasks in GetMode");
    harness.assert_selection_is_empty("Selection should be cleared on entering GetMode");
}

// --- Action Tests: Chain ---
#[test]
fn test_chain_selected_tasks() {
    let mut harness = TestHarness::new(None);
    let t1 = harness.add_task_direct("Chain_T1");
    let t2 = harness.add_task_direct("Chain_T2");
    let t3 = harness.add_task_direct("Chain_T3");

    // Select T1, T2, T3 in order (assuming they are displayed in this order)
    harness.select_task_at_index(0); // Select T1
    harness.select_task_at_index(1); // Select T2
    harness.select_task_at_index(2); // Select T3

    harness.inputs(&[
        SimulatedInput::Char(':'),
        SimulatedInput::TypeString("chain".to_string()),
        SimulatedInput::Enter,
    ]);

    harness.assert_mode_is(ViewMode::Default, "Back to default after chain command");
    harness.assert_status_message_contains("Successfully chained 2 pairs", "Chain success message");

    // Verify dependencies
    assert!(harness.todo_list.deps(t2).contains(t1), "T2 should be blocked by T1");
    assert!(harness.todo_list.deps(t3).contains(t2), "T3 should be blocked by T2");
}
