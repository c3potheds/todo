use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use super::app_state::{AppState, ViewMode};
use super::actions::Action;

/// Handles user key events and translates them into `Action`s or direct `AppState` modifications.
pub fn handle_input(
    key_event: KeyEvent,
    app_state: &mut AppState,
    // removed: _todo_list: &mut TodoList,
) -> Action {
    // Global quit shortcut
    if key_event.code == KeyCode::Char('c') && key_event.modifiers == KeyModifiers::CONTROL {
        return Action::Quit;
    }

    match app_state.view_mode {
        ViewMode::Default => match key_event.code {
            KeyCode::Char('q') => Action::Quit,
            KeyCode::Up | KeyCode::Char('k') => { app_state.move_cursor_up(); Action::NoOp }
            KeyCode::Down | KeyCode::Char('j') => { app_state.move_cursor_down(); Action::NoOp }
            KeyCode::Char(' ') => {
                if key_event.modifiers == KeyModifiers::SHIFT {
                    app_state.select_all_to_cursor();
                } else {
                    app_state.toggle_selection();
                }
                Action::NoOp
            }
            KeyCode::Esc => { app_state.clear_selection(); Action::NoOp }

            // Mode changes - these modify AppState directly and usually don't trigger further immediate actions.
            KeyCode::Char('n') => { app_state.change_mode(ViewMode::EditingNewTask); Action::NoOp }
            KeyCode::Char('s') => { app_state.change_mode(ViewMode::SnoozeInput); Action::NoOp }
            KeyCode::Char('P') => { app_state.change_mode(ViewMode::PriorityInput); Action::NoOp }
            KeyCode::Char('d') => { app_state.change_mode(ViewMode::DueDateInput); Action::NoOp }
            KeyCode::Char(':') => { app_state.change_mode(ViewMode::CommandInput); Action::NoOp }

            // Direct actions
            KeyCode::Char('c') => Action::CheckSelectedTasks,
            KeyCode::Char('r') => Action::RestoreSelectedTasks,
            KeyCode::Char('g') => Action::EnterGetMode,
            KeyCode::Char('p') => Action::PuntSelectedTasks,
            // Enter on selected tasks in default mode attempts to chain them.
            KeyCode::Enter if !app_state.selected_tasks.is_empty() => Action::ChainSelectedTasks,
            _ => Action::NoOp,
        },

        ViewMode::EditingNewTask => match key_event.code {
            KeyCode::Enter => Action::CreateNewTask(app_state.input_buffer.trim().to_string()),
            KeyCode::Esc => { app_state.change_mode(ViewMode::Default); app_state.status_message = "Input cancelled".to_string(); Action::NoOp }
            KeyCode::Char(c) => { app_state.input_buffer.push(c); Action::NoOp }
            KeyCode::Backspace => { app_state.input_buffer.pop(); Action::NoOp }
            _ => Action::NoOp,
        },
        ViewMode::SnoozeInput => match key_event.code {
            KeyCode::Enter => Action::SnoozeSelectedTasks(app_state.input_buffer.trim().to_string()),
            KeyCode::Esc => { app_state.change_mode(ViewMode::Default); app_state.status_message = "Input cancelled".to_string(); Action::NoOp }
            KeyCode::Char(c) => { app_state.input_buffer.push(c); Action::NoOp }
            KeyCode::Backspace => { app_state.input_buffer.pop(); Action::NoOp }
            _ => Action::NoOp,
        },
        ViewMode::PriorityInput => match key_event.code {
            KeyCode::Enter => Action::SetPrioritySelectedTasks(app_state.input_buffer.trim().to_string()),
            KeyCode::Esc => { app_state.change_mode(ViewMode::Default); app_state.status_message = "Input cancelled".to_string(); Action::NoOp }
            KeyCode::Char(c) => { app_state.input_buffer.push(c); Action::NoOp }
            KeyCode::Backspace => { app_state.input_buffer.pop(); Action::NoOp }
            _ => Action::NoOp,
        },
        ViewMode::DueDateInput => match key_event.code {
            KeyCode::Enter => Action::SetDueDateSelectedTasks(app_state.input_buffer.trim().to_string()),
            KeyCode::Esc => { app_state.change_mode(ViewMode::Default); app_state.status_message = "Input cancelled".to_string(); Action::NoOp }
            KeyCode::Char(c) => { app_state.input_buffer.push(c); Action::NoOp }
            KeyCode::Backspace => { app_state.input_buffer.pop(); Action::NoOp }
            _ => Action::NoOp,
        },
        ViewMode::CommandInput => match key_event.code {
            KeyCode::Enter => Action::RunCommand(app_state.input_buffer.trim().to_string()),
            KeyCode::Esc => { app_state.change_mode(ViewMode::Default); app_state.status_message = "Input cancelled".to_string(); Action::NoOp }
            KeyCode::Char(c) => { app_state.input_buffer.push(c); Action::NoOp }
            KeyCode::Backspace => { app_state.input_buffer.pop(); Action::NoOp }
            _ => Action::NoOp,
        },
        ViewMode::GetMode => match key_event.code {
            KeyCode::Up | KeyCode::Char('k') => { app_state.move_cursor_up(); Action::NoOp }
            KeyCode::Down | KeyCode::Char('j') => { app_state.move_cursor_down(); Action::NoOp }
            KeyCode::Char(' ') => {
                if key_event.modifiers == KeyModifiers::SHIFT {
                    app_state.select_all_to_cursor();
                } else {
                    app_state.toggle_selection();
                }
                Action::NoOp
            }
            KeyCode::Char('g') => Action::UpdateGetMode, // Re-filter 'get' view based on current selection
            KeyCode::Esc => Action::ExitGetMode, // Exit 'get' view and return to default task list
            _ => Action::NoOp,
        }
    }
}
