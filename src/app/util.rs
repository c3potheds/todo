use cli::Key;
use model::TaskId;
use model::TodoList;
use printing::Action;
use printing::PrintableTask;
use printing::PrintingContext;

pub fn format_task<'a>(
    context: &'a PrintingContext,
    model: &'a TodoList,
    id: TaskId,
    action: Action,
) -> PrintableTask<'a> {
    let number = model.get_number(id).unwrap();
    PrintableTask {
        context: context,
        desc: &model.get(id).unwrap().desc,
        number: number,
        status: model.get_status(id).unwrap(),
        action: action,
    }
}

pub fn lookup_tasks<'a>(
    model: &'a TodoList,
    keys: impl IntoIterator<Item = &'a Key>,
) -> Vec<TaskId> {
    keys.into_iter()
        .flat_map(|&Key::ByNumber(n)| model.lookup_by_number(n))
        .collect::<Vec<_>>()
}
