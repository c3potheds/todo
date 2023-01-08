use printing::{PrintableAppSuccess, PrintableResult};

use {
    super::util::{format_task, lookup_tasks},
    cli::Edit,
    itertools::Itertools,
    model::{TaskSet, TodoList},
    printing::PrintableError,
    std::borrow::Cow,
    text_editing::TextEditor,
};

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

fn edit_with_description<'list>(
    list: &'list mut TodoList,
    ids: &TaskSet,
    desc: &str,
    include_done: bool,
) -> PrintableResult<'list> {
    let tasks_to_print: Vec<_> = ids
        .iter_sorted(list)
        .fold(TaskSet::default(), |so_far, id| {
            so_far | list.set_desc(id, Cow::Owned(desc.trim().to_string()))
        })
        .include_done(list, include_done)
        .iter_sorted(list)
        .map(|id| format_task(list, id))
        .collect();
    let mutated = !tasks_to_print.is_empty();
    Ok(PrintableAppSuccess {
        tasks: tasks_to_print,
        mutated,
        ..Default::default()
    })
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
                Some(desc) => Ok((num, desc.trim().to_string())),
                _ => Err(EditError::MissingTaskDescription),
            },
            _ => Err(EditError::InvalidNumber(should_be_num.to_string())),
        },
        _ => Err(EditError::MissingDelimiterBetweenNumberAndDescription),
    }
}

fn update_desc(
    list: &mut TodoList,
    ids: &TaskSet,
    pos: i32,
    desc: &str,
) -> Result<TaskSet, PrintableError> {
    match list.lookup_by_number(pos) {
        Some(id) => {
            if !ids.contains(id) {
                Err(PrintableError::CannotEditBecauseUnexpectedNumber {
                    requested: pos,
                })
            } else {
                Ok(list.set_desc(id, Cow::Owned(desc.to_string())))
            }
        }
        _ => Err(PrintableError::CannotEditBecauseNoTaskWithNumber {
            requested: pos,
        }),
    }
}

fn edit_with_text_editor<'list>(
    list: &'list mut TodoList,
    ids: &TaskSet,
    editor_output: &str,
) -> PrintableResult<'list> {
    let mut mutated = false;
    let tasks_to_print = editor_output
        .lines()
        .try_fold(TaskSet::default(), |so_far, line| {
            match parse_line_from_text_editor(line) {
                Ok((pos, desc)) => Ok(so_far
                    | update_desc(list, ids, pos, &desc).map(|x| {
                        mutated = true;
                        x
                    })?),
                Err(_) => Err(PrintableError::CannotEditBecauseInvalidLine {
                    malformed_line: line.to_string(),
                }),
            }
        })
        .map_err(|e| vec![e])?
        .iter_sorted(list)
        .map(|id| format_task(list, id))
        .collect();
    Ok(PrintableAppSuccess {
        tasks: tasks_to_print,
        mutated,
        ..Default::default()
    })
}

pub fn run<'list>(
    list: &'list mut TodoList,
    text_editor: &impl TextEditor,
    cmd: &Edit,
) -> PrintableResult<'list> {
    let tasks_to_edit = lookup_tasks(list, &cmd.keys);
    match &cmd.desc {
        Some(ref desc) => {
            edit_with_description(list, &tasks_to_edit, desc, cmd.include_done)
        }
        None => match text_editor
            .edit_text(&format_tasks_for_text_editor(list, &tasks_to_edit))
        {
            Ok(ref output) => {
                edit_with_text_editor(list, &tasks_to_edit, output)
            }
            Err(_) => Err(vec![PrintableError::FailedToUseTextEditor]),
        },
    }
}
