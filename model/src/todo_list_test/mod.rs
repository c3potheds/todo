use thiserror::Error;

use super::*;

#[derive(Debug, Error)]
pub enum TodoListError {
    #[error("could not complete task")]
    Check(#[from] CheckError),
    #[error("could not restore task")]
    Restore(#[from] RestoreError),
    #[error("could not block task")]
    Block(#[from] BlockError),
    #[error("could not unblock task")]
    Unblock(#[from] UnblockError),
    #[error("could not punt task")]
    Punt(#[from] PuntError),
}

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
mod status_test;
mod tag_test;
mod unblock_test;
