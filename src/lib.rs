//! The library module for the application.

use std::error::Error;
use std::io;

use crate::repl::Repl;

mod repl;
pub mod repository;
mod scraper;
mod site;

/// Initialize Job Hunt for job repo type T, e.g. SoftwareJobs.
pub fn init_jobhunt<T>() -> Result<(), Box<dyn Error>>
where
    T: Repl,
{
    let stdout = io::stdout();

    T::init_repl(&mut stdout.lock())
        .unwrap_or_else(|err| panic!("An error occurred while initializing Job Hunt: {err}"));
    Ok(())
}
