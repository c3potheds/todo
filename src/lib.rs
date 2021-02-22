#[macro_use]
extern crate serde;

extern crate structopt;

#[cfg(test)]
#[macro_use]
extern crate serde_json;

mod app;
mod cli;
mod model;
mod printing;

#[cfg(test)]
mod app_test;

#[cfg(test)]
mod cli_test;

#[cfg(test)]
mod model_test;

#[cfg(test)]
mod printing_test;
