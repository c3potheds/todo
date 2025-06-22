// app/src/interactive/test_utils.rs
#![cfg(test)] // This module will only be compiled when running tests

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crate::interactive::{AppState, ViewMode, Action, handle_input, execute_action};
use todo_model::{TodoList, TaskId, NewOptions, TaskStatus, Task}; // Ensure Task is imported
use chrono::{DateTime, Utc, Duration}; // Ensure DateTime, Utc, Duration are imported


#[derive(Debug, Clone)]
pub enum SimulatedInput {
    RawKeyEvent(KeyEvent),
    Char(char),
    CtrlChar(char),
    ShiftChar(char),
    Enter,
    Esc,
    Up,
    Down,
    Left,
    Right,
    Space,
    ShiftSpace,
    Backspace,
    TypeString(String),
}

fn to_key_event(sim_input: &SimulatedInput) -> KeyEvent {
    match sim_input {
        SimulatedInput::RawKeyEvent(ke) => *ke,
        SimulatedInput::Char(c) => KeyEvent::new(KeyCode::Char(*c), KeyModifiers::NONE),
        SimulatedInput::CtrlChar(c) => KeyEvent::new(KeyCode::Char(*c), KeyModifiers::CONTROL),
        SimulatedInput::ShiftChar(c) => KeyEvent::new(KeyCode::Char(*c), KeyModifiers::SHIFT),
        SimulatedInput::Enter => KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE),
        SimulatedInput::Esc => KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE),
        SimulatedInput::Up => KeyEvent::new(KeyCode::Up, KeyModifiers::NONE),
        SimulatedInput::Down => KeyEvent::new(KeyCode::Down, KeyModifiers::NONE),
        SimulatedInput::Left => KeyEvent::new(KeyCode::Left, KeyModifiers::NONE),
        SimulatedInput::Right => KeyEvent::new(KeyCode::Right, KeyModifiers::NONE),
        SimulatedInput::Space => KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE),
        SimulatedInput::ShiftSpace => KeyEvent::new(KeyCode::Char(' '), KeyModifiers::SHIFT),
        SimulatedInput::Backspace => KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE),
        SimulatedInput::TypeString(_) => panic!("TypeString should be handled by TestHarness::inputs directly"),
    }
}

pub struct TestHarness {
    pub app_state: AppState,
    pub todo_list: TodoList<'static>, // Owning for isolated tests
}

impl TestHarness {
    pub fn new(initial_todo_list: Option<TodoList<'static>>) -> Self {
        let list = initial_todo_list.unwrap_or_default();
        let app_state = AppState::new(&list);
        Self {
            app_state,
            todo_list: list,
        }
    }

    pub fn add_task_direct(&mut self, desc: &str) -> TaskId {
        let id = self.todo_list.add(NewOptions { desc: desc.into() });
        self.app_state.update_displayed_tasks(&self.todo_list);
        id
    }

    pub fn select_task_at_index(&mut self, index: usize) {
        if let Some(task_id) = self.app_state.displayed_tasks.get(index) {
            self.app_state.selected_tasks.insert(*task_id);
        } else {
            panic!("Cannot select task at index {}: index out of bounds or list empty.", index);
        }
    }

    pub fn input(&mut self, simulated_input: &SimulatedInput) {
        if let SimulatedInput::TypeString(s) = simulated_input {
            for char_event in s.chars().map(SimulatedInput::Char) {
                self.process_key_event(to_key_event(&char_event));
            }
        } else {
            self.process_key_event(to_key_event(simulated_input));
        }
    }

    fn process_key_event(&mut self, key_event: KeyEvent) {
        let action = handle_input(key_event, &mut self.app_state);

        if self.app_state.should_quit && !matches!(action, Action::Quit) {
            return;
        }

        if !matches!(action, Action::NoOp) {
            execute_action(action, &mut self.app_state, &mut self.todo_list);
        }

        if self.app_state.should_quit {
             return;
        }
        self.app_state.update_displayed_tasks(&self.todo_list);
    }

    pub fn inputs(&mut self, simulated_inputs: &[SimulatedInput]) {
        for sim_input in simulated_inputs {
            if self.app_state.should_quit { break; }
            self.input(sim_input);
        }
    }

    pub fn assert_cursor_at(&self, expected_index: usize, message: &str) {
        assert_eq!(self.app_state.cursor_index, expected_index, "{} - Cursor position mismatch", message);
    }

    pub fn assert_mode_is(&self, expected_mode: ViewMode, message: &str) {
        assert_eq!(self.app_state.view_mode, expected_mode, "{} - ViewMode mismatch", message);
    }

    pub fn assert_task_desc_at_cursor(&self, expected_desc: Option<&str>, message: &str) {
        let current_task_id = self.app_state.current_task_id();
        match (expected_desc, current_task_id) {
            (Some(desc), Some(id)) => assert_eq!(self.todo_list.get(id).unwrap().desc, desc, "{} - Task description at cursor mismatch", message),
            (None, None) => { /* Both None, that's a match */ },
            (Some(_), None) => panic!("{} - Expected task description '{}', but no task at cursor", message, expected_desc.unwrap()),
            (None, Some(id)) => panic!("{} - Expected no task at cursor, but found '{}'", message, self.todo_list.get(id).unwrap().desc),
        }
    }

    pub fn assert_input_buffer_is(&self, expected_buffer: &str, message: &str) {
        assert_eq!(self.app_state.input_buffer, expected_buffer, "{} - Input buffer mismatch", message);
    }

    pub fn assert_selection_contains(&self, task_id: TaskId, message: &str) {
        assert!(self.app_state.selected_tasks.contains(&task_id), "{} - Selection does not contain task {:?}", message, task_id);
    }

    pub fn assert_selection_is_empty(&self, message: &str) {
        assert!(self.app_state.selected_tasks.is_empty(), "{} - Selection is not empty", message);
    }

    pub fn assert_status_message_contains(&self, substring: &str, message: &str) {
        assert!(self.app_state.status_message.contains(substring), "{} - Status message '{}' does not contain '{}'", message, self.app_state.status_message, substring);
    }

    // New assertion helpers
    pub fn assert_task_status(&self, task_id: TaskId, expected_status: TaskStatus, message: &str) {
        assert_eq!(self.todo_list.status(task_id), Some(expected_status), "{} - Task {:?} status mismatch", message, task_id);
    }

    pub fn assert_task_priority(&self, task_id: TaskId, expected_priority: i32, message: &str) {
        match self.todo_list.get(task_id) {
            Some(task) => assert_eq!(task.priority, expected_priority, "{} - Task {:?} priority mismatch", message, task_id),
            None => panic!("{} - Task {:?} not found for priority check", message, task_id),
        }
    }

    fn assert_datetime_approx_eq(actual: Option<DateTime<Utc>>, expected: Option<DateTime<Utc>>, tolerance_seconds: i64, msg_prefix: &str) {
        match (actual, expected) {
            (Some(act_dt), Some(exp_dt)) => {
                assert!((act_dt - exp_dt).num_seconds().abs() <= tolerance_seconds,
                        "{} - DateTime mismatch: actual='{}', expected='{}', tolerance_seconds={}",
                        msg_prefix, act_dt.to_rfc3339(), exp_dt.to_rfc3339(), tolerance_seconds);
            }
            (None, None) => { /* Both None, this is fine */ }
            _ => panic!("{} - DateTime presence mismatch: actual='{:?}', expected='{:?}'", msg_prefix, actual, expected),
        }
    }

    pub fn assert_task_due_date_approx(&self, task_id: TaskId, expected_date: Option<DateTime<Utc>>, message: &str) {
        match self.todo_list.get(task_id) {
            Some(task) => {
                let msg = format!("{} - Task {:?} due date", message, task_id);
                Self::assert_datetime_approx_eq(task.due_date, expected_date, 60, &msg);
            }
            None => panic!("{} - Task {:?} not found for due date check", message, task_id),
        }
    }

    pub fn assert_task_snooze_date_approx(&self, task_id: TaskId, expected_start_date: DateTime<Utc>, message: &str) {
        match self.todo_list.get(task_id) {
            Some(task) => {
                 let msg = format!("{} - Task {:?} snooze (start_date)", message, task_id);
                 Self::assert_datetime_approx_eq(Some(task.start_date), Some(expected_start_date), 60, &msg);
            }
            None => panic!("{} - Task {:?} not found for snooze date check", message, task_id),
        }
    }

    pub fn assert_dependency_exists(&self, dependent_id: TaskId, dependency_id: TaskId, message: &str) {
        let deps = self.todo_list.deps(dependent_id);
        assert!(deps.contains(dependency_id), "{} - Task {:?} does not depend on {:?}", message, dependent_id, dependency_id);
    }

    pub fn assert_task_count(&self, expected_count: usize, status_filter: Option<TaskStatus>, message: &str) {
        let count = self.todo_list.all_tasks()
            .filter(|&id| {
                match status_filter {
                    Some(filter_status) => self.todo_list.status(id) == Some(filter_status),
                    None => true, // No filter, count all
                }
            })
            .count();
        assert_eq!(count, expected_count, "{} - Task count mismatch (filter: {:?})", message, status_filter);
    }

    pub fn assert_displayed_tasks_contain_all(&self, task_ids: &[TaskId], message: &str) {
        for task_id in task_ids {
            assert!(self.app_state.displayed_tasks.contains(task_id), "{} - Displayed tasks do not contain {:?}", message, task_id);
        }
    }

    pub fn assert_displayed_tasks_contain_none(&self, task_ids: &[TaskId], message: &str) {
        for task_id in task_ids {
            assert!(!self.app_state.displayed_tasks.contains(task_id), "{} - Displayed tasks should not contain {:?}", message, task_id);
        }
    }

    pub fn assert_displayed_tasks_exact(&self, expected_task_ids_ordered: &[TaskId], message: &str) {
        assert_eq!(&self.app_state.displayed_tasks, expected_task_ids_ordered, "{} - Displayed tasks do not match expected order/content", message);
    }

    pub fn assert_selection_count(&self, expected_count: usize, message: &str) {
        assert_eq!(self.app_state.selected_tasks.len(), expected_count, "{} - Selection count mismatch", message);
    }
}
