use std::error::Error;
use std::io;
use crate::repl::Repl;

pub mod scraper;
pub mod repository;
pub mod site;
pub mod repl;

/// Initialize Job Hunt for jobs type T, eg. SoftwareJobs.
pub fn init_jobhunt<T>() -> Result<(), Box<dyn Error>>
    where T: Repl
{
    let stdin = io::stdin();
    let stdout = io::stdout();

    T::init_repl(&mut stdin.lock(), &mut stdout.lock())
        .unwrap_or_else(|e| panic!("An error occurred while initializing Job Hunt: {}", e));

    Ok(())
}
