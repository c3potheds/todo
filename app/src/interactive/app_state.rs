use std::collections::HashSet;
use todo_model::{TaskId, TodoList};

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub enum ViewMode {
    #[default]
    Default,
    EditingNewTask,
    GetMode, // Mode for showing filtered tasks based on 'get' command
    SnoozeInput,
    PriorityInput,
    DueDateInput,
    CommandInput,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub cursor_index: usize,
    /// List of TaskIds currently visible in the task list UI.
    /// This is updated by `update_displayed_tasks` or directly by actions like 'get'.
    pub displayed_tasks: Vec<TaskId>,
    pub selected_tasks: HashSet<TaskId>,
    pub view_mode: ViewMode,
    pub input_buffer: String,
    /// Stores the visual index of the last task selected/deselected,
    /// used as the anchor point for range selections (Shift + Space).
    pub last_selected_task_index_for_shift_select: Option<usize>,
    pub status_message: String,
    pub should_quit: bool,
}

impl AppState {
    pub fn new(todo_list: &TodoList) -> Self {
        let mut app_state = Self {
            cursor_index: 0,
            displayed_tasks: Vec::new(),
            selected_tasks: HashSet::new(),
            view_mode: ViewMode::Default,
            input_buffer: String::new(),
            last_selected_task_index_for_shift_select: None,
            status_message: String::new(),
            should_quit: false,
        };
        app_state.update_displayed_tasks(todo_list);
        app_state
    }

    /// Updates `displayed_tasks` based on the current `TodoList` and `ViewMode`.
    /// Note: For `ViewMode::GetMode`, `displayed_tasks` is set directly by the
    /// `EnterGetMode` or `UpdateGetMode` actions.
    pub fn update_displayed_tasks(&mut self, todo_list: &TodoList) {
        match self.view_mode {
            ViewMode::Default
            | ViewMode::EditingNewTask
            | ViewMode::SnoozeInput
            | ViewMode::PriorityInput
            | ViewMode::DueDateInput
            | ViewMode::CommandInput => {
                // Shows all incomplete tasks.
                // Future enhancements could include respecting CLI options (include_blocked, include_done)
                // and specific sorting.
                self.displayed_tasks = todo_list.incomplete_tasks().collect();
                // TODO: Add sorting based on task properties if needed (e.g., by priority, due date)
            }
            ViewMode::GetMode => {
                // In GetMode, displayed_tasks is managed by EnterGetMode/UpdateGetMode actions.
                // This method call will ensure cursor clamping if it's called while in GetMode,
                // but won't change the content of displayed_tasks.
            }
        }

        // Ensure cursor_index is within bounds of the (potentially new) displayed_tasks list.
        if !self.displayed_tasks.is_empty() {
            self.cursor_index = self.cursor_index.min(self.displayed_tasks.len() - 1);
        } else {
            self.cursor_index = 0;
        }
    }

    /// Returns the TaskId of the task currently under the cursor, if any.
    pub fn current_task_id(&self) -> Option<TaskId> {
        self.displayed_tasks.get(self.cursor_index).copied()
    }

    pub fn move_cursor_up(&mut self) {
        if self.cursor_index > 0 {
            self.cursor_index -= 1;
        }
    }

    pub fn move_cursor_down(&mut self) {
        if !self.displayed_tasks.is_empty()
            && self.cursor_index < self.displayed_tasks.len() - 1
        {
            self.cursor_index += 1;
        }
    }

    /// Toggles the selection state of the task currently under the cursor.
    /// Updates `last_selected_task_index_for_shift_select`.
    pub fn toggle_selection(&mut self) {
        if let Some(task_id) = self.current_task_id() {
            if self.selected_tasks.contains(&task_id) {
                self.selected_tasks.remove(&task_id);
            } else {
                self.selected_tasks.insert(task_id);
            }
            self.last_selected_task_index_for_shift_select = Some(self.cursor_index);
        }
    }

    /// Selects all tasks between `last_selected_task_index_for_shift_select` and the current cursor position.
    /// If `last_selected_task_index_for_shift_select` is None, it behaves like `toggle_selection`.
    pub fn select_all_to_cursor(&mut self) {
        if let Some(last_selected_idx) = self.last_selected_task_index_for_shift_select {
            let current_idx = self.cursor_index;
            let start = std::cmp::min(last_selected_idx, current_idx);
            let end = std::cmp::max(last_selected_idx, current_idx);
            for i in start..=end {
                if let Some(task_id) = self.displayed_tasks.get(i) {
                    self.selected_tasks.insert(*task_id);
                }
            }
        } else {
            self.toggle_selection();
        }
    }

    pub fn clear_selection(&mut self) {
        self.selected_tasks.clear();
        self.last_selected_task_index_for_shift_select = None;
    }

    /// Changes the current view mode and clears the input buffer and status message.
    pub fn change_mode(&mut self, new_mode: ViewMode) {
        self.view_mode = new_mode;
        self.input_buffer.clear();
        self.status_message.clear();
    }
}
