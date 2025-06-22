use super::app_state::{AppState, ViewMode};
use chrono::Utc;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};
use todo_model::{Task, TaskId, TodoList, TaskStatus};

/// Determines the character and style for a task's status indicator.
/// Note: Current status determination relies on Task fields and direct dependency checks.
/// For more complex states or better performance, integrating `todo_list.status()` would be an improvement,
/// but it has lifetime complexities with the current `displayed_tasks` approach.
fn get_task_status_char(task: &Task, todo_list: &TodoList, task_id: TaskId) -> (char, Style) {
    if task.completion_time.is_some() {
        return ('✓', Style::default().fg(Color::Green));
    }

    // Overdue tasks have high priority in status display.
    if task.implicit_due_date.map_or(false, |due| due < Utc::now()) {
        return ('!', Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD));
    }

    // Blocked status is based on a simplified check of direct, incomplete dependencies.
    // A full status check would involve traversing the dependency graph.
    let is_blocked = todo_list
        .deps(task_id)
        .iter_sorted(todo_list)
        .any(|dep_id| todo_list.status(dep_id) != Some(TaskStatus::Complete));
    if is_blocked {
        return ('✗', Style::default().fg(Color::Yellow));
    }

    if task.is_snoozed() {
        return ('~', Style::default().fg(Color::Cyan));
    }

    (' ', Style::default().fg(Color::Gray)) // Default for incomplete, non-special tasks
}

pub fn draw_ui<B: Backend>(frame: &mut Frame<B>, app_state: &AppState, todo_list: &TodoList) {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0), // Task list area
            Constraint::Length(
                // Base height for help text line
                1
                // Add height for input buffer if an input mode is active
                + if app_state.view_mode != ViewMode::Default && app_state.view_mode != ViewMode::GetMode { 1 } else { 0 }
                // Add height for status message if present
                + if !app_state.status_message.is_empty() { 1 } else { 0 }
            ),
        ])
        .split(frame.size());

    let task_list_chunk = main_chunks[0];
    let status_area_chunk = main_chunks[1];

    // Task List
    let mut list_items = Vec::<ListItem>::new();
    for (i, task_id) in app_state.displayed_tasks.iter().enumerate() {
        if let Some(task) = todo_list.get(*task_id) {
            let (status_char, status_style) = get_task_status_char(task, todo_list, *task_id);
            let selected_indicator = if app_state.selected_tasks.contains(task_id) { "[*]" } else { "[ ]" };

            let mut line_spans = vec![
                Span::styled(format!("{} {} ", selected_indicator, status_char), status_style),
                Span::raw(format!("{}. ", i + 1)), // Visual task number
                Span::raw(task.desc.as_ref()),
            ];

            if task.completion_time.is_none()
                && task.implicit_due_date.map_or(false, |due| due < Utc::now())
            {
                line_spans.push(Span::styled(
                    " (OVERDUE)",
                    Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD),
                ));
            }
            // Future: Display priority, full due date, tags here if desired.

            let mut list_item = ListItem::new(Line::from(line_spans));

            if i == app_state.cursor_index {
                list_item = list_item.style(
                    Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD),
                );
            } else if app_state.selected_tasks.contains(task_id) {
                // Style for selected tasks not under the cursor
                list_item = list_item.style(Style::default().bg(Color::Rgb(50, 50, 50)));
            }
            list_items.push(list_item);
        }
    }

    let tasks_list_widget = List::new(list_items)
        .block(Block::default().borders(Borders::ALL).title("To-Do List"));
    frame.render_widget(tasks_list_widget, task_list_chunk);

    // Status Bar & Input Area
    let mut status_bar_content = Vec::new();

    let help_text = match app_state.view_mode {
        ViewMode::Default => "Arrows: Nav, Space: Select, Shift+Spc: Range Sel, Esc: Clear, n: New, c:Chk, r:Restore, g:Get, p:Punt, s:Snooze, P:Prio, d:Due, :Cmd, q:Quit",
        ViewMode::EditingNewTask => "Enter: Add Task, Esc: Cancel",
        ViewMode::SnoozeInput => "Enter: Snooze (e.g. '2h', '1d12h', 'tomorrow'), Esc: Cancel",
        ViewMode::PriorityInput => "Enter: Set Priority (number), Esc: Cancel",
        ViewMode::DueDateInput => "Enter: Set Due (e.g. 'tomorrow', 'next mon'), Esc: Cancel",
        ViewMode::CommandInput => "Enter: Run Command, Esc: Cancel",
        ViewMode::GetMode => "Arrows: Nav, Space: Select, Shift+Spc: Range Sel, g: Update Get, Esc: Default View",
    };
    status_bar_content.push(Line::from(Span::styled(help_text, Style::default().fg(Color::Yellow))));

    if app_state.view_mode != ViewMode::Default && app_state.view_mode != ViewMode::GetMode {
        let input_prompt = match app_state.view_mode {
            ViewMode::EditingNewTask => "New Task",
            ViewMode::SnoozeInput => "Snooze",
            ViewMode::PriorityInput => "Priority",
            ViewMode::DueDateInput => "Due Date",
            ViewMode::CommandInput => "Command",
            _ => "Input", // Should not happen if logic is correct
        };
        let input_line = format!("{}: {}", input_prompt, app_state.input_buffer);

        // Display simple cursor at the end of the input buffer
        let paragraph = Paragraph::new(input_line + "_").wrap(Wrap { trim: true });
        status_bar_content.push(Line::from(paragraph.into_inner()));
    }

    if !app_state.status_message.is_empty() {
        status_bar_content.push(Line::from(Span::styled(
            app_state.status_message.as_str(),
            Style::default().fg(Color::LightRed),
        )));
    }

    let status_paragraph_widget = Paragraph::new(status_bar_content)
        .block(Block::default().borders(Borders::TOP));
    frame.render_widget(status_paragraph_widget, status_area_chunk);
}
