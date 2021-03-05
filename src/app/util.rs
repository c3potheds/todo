use cli::Key;
use model::TaskId;
use model::TodoList;
use printing::PrintableTask;
use printing::PrintingContext;

pub fn format_task<'a>(
    context: &'a PrintingContext,
    model: &'a TodoList,
    id: TaskId,
) -> PrintableTask<'a> {
    let number = model.get_number(id).unwrap();
    PrintableTask {
        context: context,
        desc: &model.get(id).unwrap().desc,
        number: number,
        status: model.get_status(id).unwrap(),
    }
}

pub fn lookup_tasks(model: &TodoList, keys: &Vec<Key>) -> Vec<TaskId> {
    keys.iter()
        .flat_map(|&Key::ByNumber(n)| model.lookup_by_number(n))
        .collect::<Vec<_>>()
}
