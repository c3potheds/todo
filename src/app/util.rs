use cli::Key;
use itertools::Itertools;
use model::TaskId;
use model::TaskStatus;
use model::TodoList;
use printing::BriefPrintableTask;
use printing::PrintableTask;
use printing::Status;

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
            implicit_priority,
            Some(implicit_due_date),
        ) => {
            let mut result =
                PrintableTask::new(&task.desc, pos, to_printing_status(status));
            if let Some(p) = implicit_priority {
                if p != 0 {
                    result = result.priority(p);
                }
            }
            if let Some(due_date) = implicit_due_date {
                result = result.due_date(due_date);
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

pub fn lookup_tasks<'a>(
    model: &'a TodoList,
    keys: impl IntoIterator<Item = &'a Key>,
) -> Vec<TaskId> {
    keys.into_iter()
        .flat_map(|key| match key {
            &Key::ByNumber(n) => model
                .lookup_by_number(n)
                .into_iter()
                .collect::<Vec<_>>()
                .into_iter(),
            &Key::ByName(ref name) => model
                .all_tasks()
                .filter(|&id| {
                    model.get(id).filter(|task| &task.desc == name).is_some()
                })
                .collect::<Vec<_>>()
                .into_iter(),
            &Key::ByRange(start, end) => model
                .all_tasks()
                .filter(|&id| {
                    model
                        .position(id)
                        .filter(|&pos| start <= pos && pos <= end)
                        .is_some()
                })
                .collect::<Vec<_>>()
                .into_iter(),
        })
        .unique()
        .collect()
}

pub fn any_tasks_are_complete(
    list: &TodoList,
    mut tasks: impl Iterator<Item = TaskId>,
) -> bool {
    tasks.any(|id| list.status(id) == Some(TaskStatus::Complete))
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
