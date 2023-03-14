//! The library module for the application.

use std::error::Error;
use std::io;

use crate::repl::Repl;

pub mod repository;
mod scraper;
mod site;
mod repl;

/// Initialize Job Hunt for jobs type T, e.g. SoftwareJobs.
pub fn init_jobhunt<T>() -> Result<(), Box<dyn Error>>
    where T: Repl
{
    let stdin = io::stdin();
    let stdout = io::stdout();

    T::init_repl(&mut stdin.lock(), &mut stdout.lock())
        .unwrap_or_else(|err| panic!("An error occurred while initializing Job Hunt: {err}"));
    Ok(())
}
