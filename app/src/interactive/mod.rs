use std::io;
use chrono::Utc;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use todo_model::TodoList;

use super::app_state::AppState;
use super::input::handle_input;
use super::ui::draw_ui;
pub mod actions;
use self::actions::{Action, execute_action};
#[cfg(test)] // Only include test_utils when testing
pub mod test_utils;

#[cfg(test)]
mod tests; // Add this line

/// Runs the interactive terminal user interface.
///
/// This function initializes the terminal, sets up an event loop to handle user input
/// and time-based events (like unsnoozing tasks), and manages the application state.
/// It continuously draws the UI until the user quits.
pub fn run_interactive_ui(
    todo_list: &mut TodoList,
) -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app_state = AppState::new(todo_list);

    // Main event loop
    loop {
        if app_state.should_quit {
            break;
        }

        // Draw UI
        terminal.draw(|frame| {
            draw_ui(frame, &app_state, todo_list);
        })?;

        // Handle user input
        if event::poll(std::time::Duration::from_millis(100))? { // Poll timeout; adjust as needed
            if let CEvent::Key(key_event) = event::read()? {
                app_state.status_message.clear(); // Clear previous status before new input
                let action = handle_input(key_event, &mut app_state, todo_list);

                // `handle_input` might set `should_quit` (e.g., Ctrl-C).
                if app_state.should_quit {
                    break;
                }

                // Execute the action derived from input, if it's not a NoOp.
                if !matches!(action, Action::NoOp) {
                    execute_action(action, &mut app_state, todo_list);
                }

                // The action itself might have set `should_quit`.
                if app_state.should_quit {
                    break;
                }
            }
        }

        // Time-based events: Handle unsnoozing tasks
        let unsnoozed_tasks = todo_list.unsnooze_up_to(Utc::now());
        if !unsnoozed_tasks.is_empty() && app_state.status_message.is_empty() {
            // Display message only if no other more specific message (e.g., from an action) is present.
            app_state.status_message = format!("{} task(s) became active.", unsnoozed_tasks.len());
        }

        // Update task list for display (e.g., if tasks were unsnoozed or modified by an action)
        app_state.update_displayed_tasks(todo_list);
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
