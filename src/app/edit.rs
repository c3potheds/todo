use app::util::format_task;
use app::util::lookup_tasks;
use cli::Edit;
use itertools::Itertools;
use model::TaskId;
use model::TaskSet;
use model::TodoList;
use printing::PrintableError;
use printing::TodoPrinter;
use std::borrow::Cow;
use text_editing::TextEditor;

fn format_tasks_for_text_editor(model: &TodoList, ids: &TaskSet) -> String {
    ids.iter_sorted(model)
        .flat_map(|id| {
            model
                .position(id)
                .zip(model.get(id).map(|task| &task.desc))
                .map(|(ref pos, ref desc)| format!("{}) {}", pos, desc))
                .into_iter()
        })
        .join("\n")
}

fn edit_with_description(
    model: &mut TodoList,
    printer: &mut impl TodoPrinter,
    ids: &TaskSet,
    desc: &str,
) {
    ids.iter_sorted(model)
        .filter(|&id| model.set_desc(id, Cow::Owned(desc.to_string())))
        .collect::<TaskSet>()
        .iter_sorted(model)
        .for_each(|id| printer.print_task(&format_task(model, id)));
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
    model: &mut TodoList,
    printer: &mut impl TodoPrinter,
    ids: &TaskSet,
    pos: i32,
    desc: &str,
) -> Option<TaskId> {
    match model.lookup_by_number(pos) {
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
    .filter(|&id| model.set_desc(id, Cow::Owned(desc.to_string())))
}

fn edit_with_text_editor(
    model: &mut TodoList,
    printer: &mut impl TodoPrinter,
    ids: &TaskSet,
    editor_output: &str,
) {
    editor_output
        .lines()
        .flat_map(|line| {
            match parse_line_from_text_editor(line) {
                Ok((pos, desc)) => update_desc(model, printer, ids, pos, &desc),
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
        .iter_sorted(model)
        .for_each(|id| printer.print_task(&format_task(model, id)))
}

pub fn run(
    model: &mut TodoList,
    printer: &mut impl TodoPrinter,
    text_editor: &impl TextEditor,
    cmd: &Edit,
) {
    let tasks_to_edit = lookup_tasks(model, &cmd.keys);
    match &cmd.desc {
        Some(ref desc) => {
            edit_with_description(model, printer, &tasks_to_edit, desc)
        }
        None => match text_editor
            .edit_text(&format_tasks_for_text_editor(model, &tasks_to_edit))
        {
            Ok(ref output) => {
                edit_with_text_editor(model, printer, &tasks_to_edit, output)
            }
            Err(_) => {
                printer.print_error(&PrintableError::FailedToUseTextEditor)
            }
        },
    };
}
