use {crate::SubCommand, structopt::StructOpt};

/// Maintains and manipulates your to-do list.
///
/// You can create new tasks on your to-do list like this:
///
///   >>todo new "walk the dog" "wash the dishes"
///   NEW  1) walk the dog
///   NEW  2) wash the dishes
///
/// And view the existing tasks by just typing 'todo':
///
///   >>todo
///        1) walk the dog
///        2) wash the dishes
///
/// To use other commands that manipulate existing tasks, you can use
/// "task keys" as arguments. A task key is either a string that matches the
/// task description that you gave in 'todo new' or the number next to it in the
/// list. The number always reflects the task's absolute position in the list.
/// For example:
///
///   >>todo check 1
///   [✓]  0) walk the dog
///   >>todo
///        1) wash the dishes
///
/// The first command, 'todo check 1' "checked off" the first task in the list,
/// and the second, 'todo', showed the updated list, with "wash the dishes" now
/// the first and only remaining task, and therefore with position 1.
///
/// Notice that the completed "walk the dog" task was updated to position 0.
/// Checking a task off does not remove it from the list. You can view completed
/// tasks by using 'todo log', which at this point will show:
///
///   2021-05-01     0) walk the dog
///
/// As more tasks are completed, you'll see that the log shows completed tasks
/// in descending order (most recent first), with the position number counting
/// down into negative numbers.
///
///   >>todo check 1
///   [✓]  0) wash the dishes
///   >>todo log
///   2021-05-01      0) wash the dishes
///                  -1) walk the dog
///
/// The most unique aspect of this to-do list app is the ability to "block"
/// tasks on other tasks. This helps you focus on the tasks that you can get
/// done now, and only worry about tasks that require prerequisites once those
/// prerequisites are completed.
///
/// Let's say we add two more tasks to the list:
///
///   >>todo new "plant tomatoes in the garden"
///   NEW  1) plant tomatoes in the garden
///   >>todo new "buy tomato seeds"
///   NEW  2) buy tomato seeds
///
/// It makes sense that we need to buy tomato seeds before we plant tomatoes in
/// the garden, so we can de-clutter our list by blocking the "plant" task on
/// the "buy" task:
///
///   >>todo block 1 --on 2
///        1) buy tomato seeds
///   LCK  2) plant tomatoes in the garden
///   >>todo
///        1) buy tomato seeds
///
/// In this example, we call "buy tomato seeds" a "dependency" (or "dep") of the
/// "plant tomatoes in the garden" task. Conversely, the "plant" task is an
/// "antidependency" (or "adep") of the "buy" task. We say the "status" of the
/// "buy" task is "incomplete" and the status of the "plant" task is "blocked".
/// You may also see documentation refer to the "buy" task as "independent" and
/// the "plant" task as "dependent". On the command line, by default, incomplete
/// tasks are shown with yellow numbers and blocked tasks are shown with red
/// numbers.
///
/// To show the blocked tasks alongside the incomplete tasks, just pass the '-b'
/// flag to 'todo':
///
///   >>todo -b
///        1) buy tomato seeds
///        2) plant tomatoes in the garden
///
/// Once you 'check' the independent task, the dependent task will be "unlocked"
/// and will show up in the main to-do list again.
///
///   >>todo check 1
///        0) buy tomato seeds
///   ULK  1) plant tomatoes in the garden
///   >>todo
///        1) plant tomatoes in the garden
///
/// There are many other things you can do with this app. To read more, try
/// using 'todo help' with one of the subcommands, listed below.
#[derive(Debug, StructOpt)]
#[structopt(
    name = "todo",
    author = "Simeon Anfinrud",
    version = "0.1",
    verbatim_doc_comment
)]
pub struct Options {
    #[structopt(subcommand)]
    pub cmd: Option<SubCommand>,

    /// Show blocked tasks in the status.
    #[structopt(long, short = "b")]
    pub include_blocked: bool,

    /// Show complete tasks in the status.
    #[structopt(long, short = "d")]
    pub include_done: bool,

    /// Show all tasks in the status.
    #[structopt(long, short = "a")]
    pub include_all: bool,
}
