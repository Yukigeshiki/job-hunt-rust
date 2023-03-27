//! The repl module contains all read/eval print loop (the terminal UI for the application) code.
//! The rustyline crate is used to provide all standard CLI functionality, e.g. command history,
//! CTRL-L to clear screen, CTRL-C to interrupt, etc.

use std::cmp::Reverse;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::Write;

use chrono::Local;
use colored::Colorize;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

use crate::repository::SoftwareJobs;

/// A trait to be implemented by both the String and str types.
trait ReplStringConverter {
    /// Converts a String or str to a ReplString.
    fn to_repl_string(&self) -> ReplString;
}

impl ReplStringConverter for str {
    fn to_repl_string(&self) -> ReplString {
        ReplString::new(self)
    }
}

impl ReplStringConverter for String {
    fn to_repl_string(&self) -> ReplString {
        ReplString::new(self)
    }
}

/// A String with custom Display used by the REPL writer.
struct ReplString {
    string: String,
}

impl ReplString {
    fn new<S: Into<String>>(s: S) -> Self {
        Self { string: s.into() }
    }

    /// Uses a writer to write a repl string to std out.
    fn write<W>(self, w: &mut W) -> std::io::Result<()>
    where
        W: Write,
    {
        w.write_all(format!("{}", self).as_bytes())?;
        w.flush()
    }
}

impl Display for ReplString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string.bold().green())
    }
}

/// This trait must be implemented by the specific job repo struct to be used in Job Hunt (e.g. SoftwareJobs).
pub trait Repl {
    /// Initializes a repository for the job repo type that is implementing this trait; then
    /// initializes the REPL and parses queries.
    fn init_repl<W>(writer: &mut W) -> Result<(), Box<dyn Error>>
    where
        W: Write;
}

impl Repl for SoftwareJobs {
    fn init_repl<W>(writer: &mut W) -> Result<(), Box<dyn Error>>
    where
        W: Write,
    {
        "Populating/indexing local datastore...\n"
            .to_repl_string()
            .write(writer)?;
        let mut repo = Self::init_repo();
        "Population/indexing completed successfully! Welcome, please begin your job \
        hunt by entering a query:\n"
            .to_repl_string()
            .write(writer)?;

        let mut rl = DefaultEditor::new()?;
        rl.load_history(".jobhunthistory").ok();

        loop {
            let readline = rl.readline(">> ");
            match readline {
                Ok(line) => {
                    rl.add_history_entry(line.as_str())?;

                    match line.as_str() {
                        "fetch jobs" => {
                            repo.all.sort_by_key(|job| {
                                (job.date_posted.clone(), Reverse(job.company.clone()))
                            });
                            for job in &repo.all {
                                writer.write_all(format!("{:?}\n", job).as_bytes())?;
                                writer.flush()?;
                            }
                            format!("{} items returned.\n", repo.all.len())
                                .to_repl_string()
                                .write(writer)?;
                        }
                        "exit" => break,
                        "refresh" => {
                            "Refreshing...\n".to_repl_string().write(writer)?;
                            repo = Self::init_repo();
                            format!(
                                "Refresh completed successfully at {}.\n",
                                Local::now().format("%d-%m-%Y %H:%M:%S")
                            )
                            .to_repl_string()
                            .write(writer)?;
                        }
                        _ => {
                            format!(
                                "Does not compute! 🤖 \"{}\" is not a valid \
                                query/command.\n",
                                line.trim()
                            )
                            .to_repl_string()
                            .write(writer)?;
                        }
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    // CTRL-C
                    break;
                }
                Err(ReadlineError::Eof) => {
                    // CTRL-D
                    break;
                }
                Err(err) => {
                    format!("An error has occurred: {err}")
                        .to_repl_string()
                        .write(writer)?;
                    break;
                }
            }
        }

        "\nThank you for using Job Hunt. Goodbye!\n"
            .to_repl_string()
            .write(writer)?;
        rl.save_history(".jobhunthistory")?;

        Ok(())
    }
}
