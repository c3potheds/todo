use clap::Parser;

/// Repair the database.
///
/// Tasks are intended to be shown in sorted order, but if definition of sorted
/// order changes, or if the database is corrupted, tasks may be shown in an
/// unexpected order. This command will re-sort the database, and ensure that
/// tasks are shown in the correct order.
///
/// If the tasks are already in canonical order, this command should have no
/// effect.
#[derive(Debug, PartialEq, Eq, Parser)]
pub struct Clean {}
