use super::app_state::{AppState, ViewMode};
use chrono::Utc;
use todo_model::{
    CheckError, CheckOptions, NewOptions, PuntError, RestoreError, SnoozeWarning, TaskSet, TodoList,
};
use todo_time_format;

#[derive(Debug, Clone)]
pub enum Action {
    NoOp,
    Quit,
    CheckSelectedTasks,
    RestoreSelectedTasks,
    PuntSelectedTasks,
    CreateNewTask(String),
    SnoozeSelectedTasks(String),
    SetPrioritySelectedTasks(String),
    SetDueDateSelectedTasks(String),
    RunCommand(String),
    EnterGetMode,
    UpdateGetMode,
    ExitGetMode,
    ChainSelectedTasks,
}

pub fn execute_action(
    action: Action,
    app_state: &mut AppState,
    todo_list: &mut TodoList,
) {
    app_state.status_message.clear(); // Clear previous status message before processing new action

    match action {
        Action::NoOp => {}
        Action::Quit => app_state.should_quit = true,

        Action::CreateNewTask(desc) => {
            if !desc.is_empty() {
                let new_options = NewOptions::new().desc(desc);
                let new_task_id = todo_list.add(new_options);
                app_state.status_message = format!(
                    "New task added: {}",
                    todo_list
                        .get(new_task_id)
                        .map(|t| t.desc.as_ref())
                        .unwrap_or("")
                );
            } else {
                app_state.status_message = "Cannot add empty task".to_string();
            }
            app_state.change_mode(ViewMode::Default);
        }
        Action::CheckSelectedTasks => {
            if app_state.selected_tasks.is_empty() {
                app_state.status_message = "No tasks selected to check.".to_string();
                return;
            }
            let mut success_count = 0;
            let mut error_summaries = Vec::new();
            for task_id in app_state.selected_tasks.iter() {
                match todo_list.check(CheckOptions {
                    id: *task_id,
                    now: Utc::now(),
                    force: true,
                }) {
                    Ok(_) => success_count += 1,
                    Err(e) => {
                        let err_msg = match e {
                            CheckError::TaskIsAlreadyComplete => format!(
                                "Task #{} already complete.",
                                todo_list.position(*task_id).unwrap_or(0)
                            ),
                            CheckError::TaskIsBlockedBy(deps) => format!(
                                "Task #{} blocked by {} deps.",
                                todo_list.position(*task_id).unwrap_or(0),
                                deps.len()
                            ),
                        };
                        error_summaries.push(err_msg);
                    }
                }
            }
            app_state.status_message = format!("Checked {} tasks.", success_count);
            if !error_summaries.is_empty() {
                app_state
                    .status_message
                    .push_str(&format!(" Errors: {}", error_summaries.join("; ")));
            }
            app_state.clear_selection();
        }
        Action::RestoreSelectedTasks => {
            if app_state.selected_tasks.is_empty() {
                app_state.status_message = "No tasks selected to restore.".to_string();
                return;
            }
            let mut success_count = 0;
            let mut error_summaries = Vec::new();
            for task_id in app_state.selected_tasks.iter() {
                match todo_list.restore(*task_id, true) {
                    Ok(_) => success_count += 1,
                    Err(e) => {
                        let err_msg = match e {
                            RestoreError::TaskIsAlreadyIncomplete => format!(
                                "Task #{} already incomplete.",
                                todo_list.position(*task_id).unwrap_or(0)
                            ),
                            RestoreError::WouldRestore(adeps) => format!(
                                "Task #{} would restore {} adeps (use force if intended).",
                                todo_list.position(*task_id).unwrap_or(0),
                                adeps.len()
                            ),
                        };
                        error_summaries.push(err_msg);
                    }
                }
            }
            app_state.status_message = format!("Restored {} tasks.", success_count);
            if !error_summaries.is_empty() {
                app_state
                    .status_message
                    .push_str(&format!(" Errors: {}", error_summaries.join("; ")));
            }
            app_state.clear_selection();
        }
        Action::PuntSelectedTasks => {
            if app_state.selected_tasks.is_empty() {
                app_state.status_message = "No tasks selected to punt.".to_string();
                return;
            }
            let mut punt_count = 0;
            let mut error_count = 0;
            for task_id in app_state.selected_tasks.iter() {
                if todo_list.punt(*task_id).is_ok() {
                    punt_count += 1;
                } else {
                    error_count += 1;
                }
            }
            app_state.status_message = format!("Punted {} tasks.", punt_count);
            if error_count > 0 {
                app_state.status_message.push_str(&format!(
                    " {} tasks could not be punted (e.g. already complete or no later tasks).",
                    error_count
                ));
            }
            app_state.clear_selection();
        }
        Action::SnoozeSelectedTasks(snooze_str) => {
            app_state.change_mode(ViewMode::Default);
            if app_state.selected_tasks.is_empty() {
                app_state.status_message = "No tasks selected to snooze.".to_string();
                return;
            }
            match todo_time_format::parse_date_or_duration(&snooze_str) {
                Ok(datetime) => {
                    let mut snoozed_count = 0;
                    let mut warning_messages = Vec::new();
                    for task_id in app_state.selected_tasks.iter() {
                        match todo_list.snooze(*task_id, datetime) {
                            Ok(_) => snoozed_count += 1,
                            Err(warnings) => {
                                for w in warnings {
                                    warning_messages.push(format!(
                                        "Task #{}: {:?}",
                                        todo_list.position(*task_id).unwrap_or(0),
                                        w
                                    ));
                                }
                            }
                        }
                    }
                    app_state.status_message = format!(
                        "Attempted to snooze {} tasks until {}.",
                        snoozed_count,
                        datetime.to_rfc2822()
                    );
                    if !warning_messages.is_empty() {
                        app_state.status_message.push_str(&format!(
                            " Warnings: {}",
                            warning_messages.join("; ")
                        ));
                    }
                }
                Err(e) => {
                    app_state.status_message = format!(
                        "Invalid snooze format: '{}'. Try '2h', '1d', 'tomorrow'. Error: {}",
                        snooze_str, e
                    );
                }
            }
            app_state.clear_selection();
        }
        Action::SetPrioritySelectedTasks(prio_str) => {
            app_state.change_mode(ViewMode::Default);
            if app_state.selected_tasks.is_empty() {
                app_state.status_message = "No tasks selected.".to_string();
                return;
            }
            match prio_str.parse::<i32>() {
                Ok(priority) => {
                    for task_id in app_state.selected_tasks.iter() {
                        todo_list.set_priority(*task_id, priority);
                    }
                    app_state.status_message = format!(
                        "Set priority {} for {} tasks.",
                        priority,
                        app_state.selected_tasks.len()
                    );
                }
                Err(_) => {
                    app_state.status_message =
                        format!("Invalid priority: '{}'. Must be a number.", prio_str);
                }
            }
            app_state.clear_selection();
        }
        Action::SetDueDateSelectedTasks(due_str) => {
            app_state.change_mode(ViewMode::Default);
            if app_state.selected_tasks.is_empty() {
                app_state.status_message = "No tasks selected.".to_string();
                return;
            }
            if due_str.is_empty() {
                for task_id in app_state.selected_tasks.iter() {
                    todo_list.set_due_date(*task_id, None);
                }
                app_state.status_message = format!(
                    "Cleared due date for {} tasks.",
                    app_state.selected_tasks.len()
                );
            } else {
                match todo_time_format::parse_date_or_duration(&due_str) {
                    Ok(datetime) => {
                        for task_id in app_state.selected_tasks.iter() {
                            todo_list.set_due_date(*task_id, Some(datetime));
                        }
                        app_state.status_message = format!(
                            "Set due date for {} tasks to {}.",
                            app_state.selected_tasks.len(),
                            datetime.to_rfc2822()
                        );
                    }
                    Err(e) => {
                        app_state.status_message = format!(
                            "Invalid due date format: '{}'. Try 'tomorrow'. Error: {}",
                            due_str, e
                        );
                    }
                }
            }
            app_state.clear_selection();
        }
        Action::EnterGetMode => {
            if app_state.selected_tasks.is_empty() {
                app_state.status_message =
                    "No tasks selected for 'get'. Select tasks first.".to_string();
                return;
            }
            let mut related_tasks_set = TaskSet::default();
            for task_id in app_state.selected_tasks.iter() {
                related_tasks_set.insert(*task_id);
                related_tasks_set.extend(todo_list.transitive_deps(*task_id));
                related_tasks_set.extend(todo_list.transitive_adeps(*task_id));
            }
            app_state.displayed_tasks = related_tasks_set.iter_sorted(todo_list).collect();
            app_state.view_mode = ViewMode::GetMode;
            app_state.status_message =
                format!("Showing {} related tasks.", app_state.displayed_tasks.len());
            app_state.cursor_index = if app_state.displayed_tasks.is_empty() {
                0
            } else {
                app_state
                    .displayed_tasks
                    .len()
                    .saturating_sub(1)
                    .min(app_state.cursor_index)
            };
            app_state.selected_tasks.clear();
        }
        Action::UpdateGetMode => {
            if app_state.selected_tasks.is_empty() {
                app_state.status_message =
                    "No tasks selected to update 'get' view.".to_string();
                return;
            }
            let mut related_tasks_set = TaskSet::default();
            for task_id in app_state.selected_tasks.iter() {
                related_tasks_set.insert(*task_id);
                related_tasks_set.extend(todo_list.transitive_deps(*task_id));
                related_tasks_set.extend(todo_list.transitive_adeps(*task_id));
            }
            app_state.displayed_tasks = related_tasks_set.iter_sorted(todo_list).collect();
            app_state.status_message =
                format!("Updated 'get' view with {} tasks.", app_state.displayed_tasks.len());
            app_state.cursor_index = if app_state.displayed_tasks.is_empty() {
                0
            } else {
                app_state
                    .displayed_tasks
                    .len()
                    .saturating_sub(1)
                    .min(app_state.cursor_index)
            };
            app_state.selected_tasks.clear();
        }
        Action::ExitGetMode => {
            app_state.change_mode(ViewMode::Default);
            app_state.status_message = "Exited Get mode.".to_string();
        }
        Action::ChainSelectedTasks => {
            if app_state.selected_tasks.len() < 2 {
                app_state.status_message =
                    "Need to select at least two tasks to chain.".to_string();
                return;
            }
            let mut sorted_selection: Vec<_> = app_state.selected_tasks.iter().cloned().collect();
            sorted_selection.sort_by_key(|id| {
                app_state
                    .displayed_tasks
                    .iter()
                    .position(|disp_id| disp_id == id)
                    .unwrap_or(usize::MAX)
            });

            let mut chain_success_count = 0;
            let mut last_error = String::new();
            for i in 1..sorted_selection.len() {
                let task_to_block = sorted_selection[i];
                let task_to_block_on = sorted_selection[i - 1];
                match todo_list.block(task_to_block).on(task_to_block_on) {
                    Ok(_) => chain_success_count += 1,
                    Err(e) => {
                        last_error = format!(
                            "Error chaining: Task #{} on #{}: {}",
                            todo_list.position(task_to_block).unwrap_or(0),
                            todo_list.position(task_to_block_on).unwrap_or(0),
                            e
                        );
                        break;
                    }
                }
            }
            if !last_error.is_empty() {
                app_state.status_message =
                    format!("Chained {} pairs. Error: {}", chain_success_count, last_error);
            } else {
                app_state.status_message =
                    format!("Successfully chained {} pairs of tasks.", chain_success_count);
            }
            app_state.clear_selection();
        }
        Action::RunCommand(cmd_str) => {
            let parts: Vec<&str> = cmd_str.splitn(2, ' ').collect();
            let command = parts[0].to_lowercase();
            let args = if parts.len() > 1 { parts[1] } else { "" };

            let mut action_to_run = Action::NoOp;

            match command.as_str() {
                "due" => action_to_run = Action::SetDueDateSelectedTasks(args.to_string()),
                "priority" | "prio" => {
                    action_to_run = Action::SetPrioritySelectedTasks(args.to_string())
                }
                "chain" => action_to_run = Action::ChainSelectedTasks,
                "check" | "done" => action_to_run = Action::CheckSelectedTasks,
                "restore" | "uncheck" => action_to_run = Action::RestoreSelectedTasks,
                "punt" => action_to_run = Action::PuntSelectedTasks,
                "snooze" | "snoz" => {
                    action_to_run = Action::SnoozeSelectedTasks(args.to_string())
                }
                "get" => action_to_run = Action::EnterGetMode,
                "new" => action_to_run = Action::CreateNewTask(args.to_string()),
                "quit" | "q" | "exit" => app_state.should_quit = true,
                _ => {
                    app_state.status_message = format!("Unknown command: '{}'", cmd_str);
                }
            }

            if !matches!(action_to_run, Action::NoOp) {
                execute_action(action_to_run, app_state, todo_list);
            }

            // Ensure mode returns to Default if a command didn't quit or change mode itself.
            if !app_state.should_quit && app_state.view_mode == ViewMode::CommandInput {
                app_state.change_mode(ViewMode::Default);
            }
        }
    }
}
