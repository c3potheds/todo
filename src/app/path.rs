use app::util::format_task;
use app::util::format_task_brief;
use app::util::lookup_tasks;
use cli::Key;
use cli::Path;
use model::TaskId;
use model::TaskSet;
use model::TodoList;
use printing::Action;
use printing::PrintableError;
use printing::PrintableWarning;
use printing::TodoPrinter;

fn verify_unambiguous(
    key: &Key,
    model: &TodoList,
    printer: &mut impl TodoPrinter,
    ids: &Vec<TaskId>,
) -> bool {
    if ids.len() == 0 {
        printer.print_warning(&PrintableWarning::NoMatchFoundForKey {
            requested_key: key.clone(),
        });
        false
    } else if ids.len() > 1 {
        printer.print_error(&PrintableError::AmbiguousKey {
            key: key.clone(),
            matches: ids
                .iter()
                .copied()
                .map(|id| format_task_brief(model, id))
                .collect(),
        });
        false
    } else {
        true
    }
}

pub fn run(model: &TodoList, printer: &mut impl TodoPrinter, cmd: &Path) {
    let from = lookup_tasks(model, vec![&cmd.from]);
    let to = lookup_tasks(model, vec![&cmd.to]);
    let from_is_unambiguous =
        verify_unambiguous(&cmd.from, model, printer, &from);
    let to_is_unambiguous = verify_unambiguous(&cmd.to, model, printer, &to);
    if !from_is_unambiguous || !to_is_unambiguous {
        return;
    }
    let from_and_adeps = from.iter().copied().collect::<TaskSet>()
        | model.transitive_adeps(from[0]);
    let to_and_deps =
        to.iter().copied().collect::<TaskSet>() | model.transitive_deps(to[0]);
    let tasks_in_path: Vec<_> =
        (from_and_adeps & to_and_deps).iter_sorted(model).collect();
    if tasks_in_path.len() == 0 {
        return;
    }
    tasks_in_path.into_iter().for_each(|id| {
        printer.print_task(&format_task(model, id).action(
            if id == from[0] || id == to[0] {
                Action::Select
            } else {
                Action::None
            },
        ))
    });
}
