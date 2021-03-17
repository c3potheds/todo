use cli::Key;
use itertools::Itertools;
use model::TaskId;
use model::TodoList;
use printing::Action;
use printing::PrintableTask;

pub fn format_task<'a>(
    model: &'a TodoList,
    id: TaskId,
    action: Action,
) -> PrintableTask<'a> {
    let number = model.position(id).unwrap();
    PrintableTask {
        desc: &model.get(id).unwrap().desc,
        number: number,
        status: model.status(id).unwrap(),
        action: action,
    }
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
        })
        .unique()
        .collect()
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
