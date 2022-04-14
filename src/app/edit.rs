use crate::{
    app::util::{format_task, lookup_tasks},
    cli::Edit,
    model::{TaskId, TaskSet, TodoList},
    printing::{PrintableError, TodoPrinter},
    text_editing::TextEditor,
};
use itertools::Itertools;
use std::borrow::Cow;

fn format_tasks_for_text_editor(list: &TodoList, ids: &TaskSet) -> String {
    ids.iter_sorted(list)
        .flat_map(|id| {
            list.position(id)
                .zip(list.get(id).map(|task| &task.desc))
                .map(|(ref pos, ref desc)| format!("{}) {}", pos, desc))
                .into_iter()
        })
        .join("\n")
}

fn edit_with_description(
    list: &mut TodoList,
    printer: &mut impl TodoPrinter,
    ids: &TaskSet,
    desc: &str,
) {
    ids.iter_sorted(list)
        .filter(|&id| list.set_desc(id, Cow::Owned(desc.to_string())))
        .collect::<TaskSet>()
        .iter_sorted(list)
        .for_each(|id| printer.print_task(&format_task(list, id)));
}

enum EditError {
    MissingDelimiterBetweenNumberAndDescription,
    MissingTaskDescription,
    InvalidNumber(String),
}

fn parse_line_from_text_editor(line: &str) -> Result<(i32, String), EditError> {
    let mut split = line.splitn(2, ") ");
    match split.next() {
        Some(should_be_num) => match should_be_num.parse::<i32>() {
            Ok(num) => match split.next() {
                Some(desc) => Ok((num, desc.to_string())),
                _ => Err(EditError::MissingTaskDescription),
            },
            _ => Err(EditError::InvalidNumber(should_be_num.to_string())),
        },
        _ => Err(EditError::MissingDelimiterBetweenNumberAndDescription),
    }
}

fn update_desc(
    list: &mut TodoList,
    printer: &mut impl TodoPrinter,
    ids: &TaskSet,
    pos: i32,
    desc: &str,
) -> Option<TaskId> {
    match list.lookup_by_number(pos) {
        Some(id) => {
            if !ids.contains(id) {
                printer.print_error(
                    &PrintableError::CannotEditBecauseUnexpectedNumber {
                        requested: pos,
                    },
                );
                None
            } else {
                Some(id)
            }
        }
        _ => {
            printer.print_error(
                &PrintableError::CannotEditBecauseNoTaskWithNumber {
                    requested: pos,
                },
            );
            None
        }
    }
    .filter(|&id| list.set_desc(id, Cow::Owned(desc.to_string())))
}

fn edit_with_text_editor(
    list: &mut TodoList,
    printer: &mut impl TodoPrinter,
    ids: &TaskSet,
    editor_output: &str,
) {
    editor_output
        .lines()
        .flat_map(|line| {
            match parse_line_from_text_editor(line) {
                Ok((pos, desc)) => update_desc(list, printer, ids, pos, &desc),
                Err(_) => {
                    printer.print_error(
                        &PrintableError::CannotEditBecauseInvalidLine {
                            malformed_line: line.to_string(),
                        },
                    );
                    None
                }
            }
            .into_iter()
        })
        .collect::<TaskSet>()
        .iter_sorted(list)
        .for_each(|id| printer.print_task(&format_task(list, id)))
}

pub fn run(
    list: &mut TodoList,
    printer: &mut impl TodoPrinter,
    text_editor: &impl TextEditor,
    cmd: &Edit,
) {
    let tasks_to_edit = lookup_tasks(list, &cmd.keys);
    match &cmd.desc {
        Some(ref desc) => {
            edit_with_description(list, printer, &tasks_to_edit, desc)
        }
        None => match text_editor
            .edit_text(&format_tasks_for_text_editor(list, &tasks_to_edit))
        {
            Ok(ref output) => {
                edit_with_text_editor(list, printer, &tasks_to_edit, output)
            }
            Err(_) => {
                printer.print_error(&PrintableError::FailedToUseTextEditor)
            }
        },
    };
}
