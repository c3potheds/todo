use chrono::DateTime;
use chrono::Duration;
use chrono::Local;
use chrono::Utc;
use cli::Key;
use model::DurationInSeconds;
use model::TaskId;
use model::TaskSet;
use model::TaskStatus;
use model::TodoList;
use printing::BriefPrintableTask;
use printing::PrintableError;
use printing::PrintableTask;
use printing::Status;
use printing::TodoPrinter;
use std::convert::TryFrom;

fn to_printing_status(status: TaskStatus) -> Status {
    match status {
        TaskStatus::Incomplete => Status::Incomplete,
        TaskStatus::Blocked => Status::Blocked,
        TaskStatus::Complete => Status::Complete,
    }
}

pub fn format_task<'a>(model: &'a TodoList, id: TaskId) -> PrintableTask<'a> {
    match (
        model.get(id),
        model.position(id),
        model.status(id),
        model.implicit_priority(id),
        model.implicit_due_date(id),
    ) {
        (
            Some(task),
            Some(pos),
            Some(status),
            Some(implicit_priority),
            Some(implicit_due_date),
        ) => {
            let mut result =
                PrintableTask::new(&task.desc, pos, to_printing_status(status))
                    .priority(implicit_priority);
            if let Some(due_date) = implicit_due_date {
                result = result.due_date(due_date);
            }
            if task.budget.0 > 0 {
                result = result.budget(Duration::seconds(task.budget.0.into()));
            }
            result
        }
        _ => panic!("Failed to get task info for id {:?}", id),
    }
}

pub fn format_task_brief(model: &TodoList, id: TaskId) -> BriefPrintableTask {
    BriefPrintableTask::new(
        model.position(id).unwrap(),
        to_printing_status(model.status(id).unwrap()),
    )
}

pub fn format_tasks_brief(
    model: &TodoList,
    tasks: &TaskSet,
) -> Vec<BriefPrintableTask> {
    tasks
        .iter_sorted(model)
        .map(|id| format_task_brief(model, id))
        .collect()
}

pub fn lookup_task(model: &TodoList, key: &Key) -> TaskSet {
    match key {
        &Key::ByNumber(n) => model.lookup_by_number(n).into_iter().collect(),
        &Key::ByName(ref name) => model
            .all_tasks()
            .filter(|&id| {
                model.get(id).filter(|task| &task.desc == name).is_some()
            })
            .collect(),
        &Key::ByRange(start, end) => model
            .all_tasks()
            .filter(|&id| {
                model
                    .position(id)
                    .filter(|&pos| start <= pos && pos <= end)
                    .is_some()
            })
            .collect(),
    }
}

pub fn lookup_tasks<'a>(
    model: &'a TodoList,
    keys: impl IntoIterator<Item = &'a Key>,
) -> TaskSet {
    keys.into_iter().fold(TaskSet::new(), |so_far, key| {
        so_far | lookup_task(model, key)
    })
}

fn any_tasks_are_complete(
    list: &TodoList,
    mut tasks: impl Iterator<Item = TaskId>,
) -> bool {
    tasks.any(|id| list.status(id) == Some(TaskStatus::Complete))
}

pub fn should_include_done(
    from_cmdline: bool,
    list: &TodoList,
    tasks: impl IntoIterator<Item = TaskId>,
) -> bool {
    from_cmdline || any_tasks_are_complete(list, tasks.into_iter())
}

pub fn parse_due_date_or_print_error(
    now: DateTime<Utc>,
    due_date_vec: &Vec<String>,
    printer: &mut impl TodoPrinter,
) -> Result<Option<DateTime<Utc>>, ()> {
    if due_date_vec.is_empty() {
        return Ok(None);
    }
    let due_date_string = due_date_vec.join(" ");
    match ::time_format::parse_time(
        Local,
        now.with_timezone(&Local),
        &due_date_string,
    ) {
        Ok(due_date) => Ok(Some(due_date.with_timezone(&Utc))),
        Err(_) => {
            printer.print_error(&PrintableError::CannotParseDueDate {
                cannot_parse: due_date_string.to_string(),
            });
            Err(())
        }
    }
}

pub fn parse_budget_or_print_error(
    budget_vec: &Vec<String>,
    printer: &mut impl TodoPrinter,
) -> Result<DurationInSeconds, ()> {
    let budget_string = budget_vec.join(" ");
    if budget_string == "0" || budget_string == "" {
        return Ok(DurationInSeconds::default());
    }
    match humantime::parse_duration(&budget_string) {
        Ok(duration) => {
            Ok(DurationInSeconds(match u32::try_from(duration.as_secs()) {
                Ok(secs) => secs,
                Err(_) => {
                    printer.print_error(&PrintableError::DurationIsTooLong {
                        duration: duration.as_secs(),
                        string_repr: budget_string.clone(),
                    });
                    return Err(());
                }
            }))
        }
        Err(_) => {
            printer.print_error(&PrintableError::CannotParseDuration {
                cannot_parse: budget_string.clone(),
            });
            return Err(());
        }
    }
}

struct Pairwise<T, I>
where
    I: Iterator<Item = T>,
{
    current: Option<T>,
    rest: I,
}

impl<T, I> Iterator for Pairwise<T, I>
where
    I: Iterator<Item = T>,
    T: Copy,
{
    type Item = (T, T);
    fn next(&mut self) -> Option<(T, T)> {
        match (&mut self.current, self.rest.next()) {
            (_, None) => None,
            (&mut None, Some(a)) => {
                self.current = Some(a);
                self.next()
            }
            (&mut Some(a), Some(b)) => {
                self.current = Some(b);
                Some((a, b))
            }
        }
    }
}

pub fn pairwise<T, I>(iter: I) -> impl Iterator<Item = (T, T)>
where
    I: IntoIterator<Item = T>,
    T: Copy,
{
    Pairwise {
        current: None,
        rest: iter.into_iter(),
    }
}
