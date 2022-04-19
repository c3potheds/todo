use super::*;

type TestResult = Result<(), TodoListError>;

mod add_test;
mod basic_test;
mod block_test;
mod budget_test;
mod check_test;
mod deps_test;
mod due_date_test;
mod get_test;
mod iter_test;
mod lookup_by_number_test;
mod position_test;
mod punt_test;
mod reload_test;
mod remove_test;
mod restore_test;
mod set_desc_test;
mod snooze_test;
mod stats_test;
mod status_test;
mod unblock_test;
