//! The repl module contains all read/eval print loop (the terminal UI for the application) code.

use std::cmp::Reverse;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::{BufRead, Write};
use colored::Colorize;
use crate::repository::SoftwareJobs;

const PROMPT: &[u8; 3] = b">> ";

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
        Self {
            string: s.into()
        }
    }

    /// Uses a writer to write a repl string to std out.
    fn write<W>(self, w: &mut W) -> std::io::Result<()>
        where W: Write
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

/// The *Jobs struct to be used in Job Hunt (e.g. SoftwareJobs) must implement the Repl trait.
pub trait Repl {
    /// Initializes a repository for the jobs type that is implementing this trait; then
    /// initializes the REPL and parses queries.
    fn init_repl<R, W>(reader: &mut R, writer: &mut W) -> Result<(), Box<dyn Error>>
        where
            R: BufRead,
            W: Write;
}

impl Repl for SoftwareJobs {
    fn init_repl<R, W>(reader: &mut R, writer: &mut W) -> Result<(), Box<dyn Error>>
        where
            R: BufRead,
            W: Write,
    {
        "Populating/indexing local datastore...\n".to_repl_string().write(writer)?;
        let mut repo = Self::init_repo();
        "Population/indexing completed successfully! Please begin your job hunt by entering a query:\n"
            .to_repl_string()
            .write(writer)?;

        loop {
            writer.write_all(PROMPT)?;
            writer.flush()?;

            let mut line = String::new();
            reader.read_line(&mut line)?;
            line = line.replace("\n", "");

            match line.trim() {
                "fetch jobs" => {
                    repo.all.sort_by_key(|job| Reverse(job.date_posted.clone()));
                    for job in &repo.all {
                        format!("{:?}\n", job).to_repl_string().write(writer)?;
                    }
                    format!("{} items returned\n", repo.all.len()).to_repl_string().write(writer)?;
                }
                "exit" => break,
                "refresh" => {
                    "Refreshing...\n".to_repl_string().write(writer)?;
                    repo = Self::init_repo();
                    "Refresh completed successfully!\n".to_repl_string().write(writer)?;
                }
                _ => {
                    format!("\"{}\" is not a valid command\n", line.trim())
                        .to_repl_string()
                        .write(writer)?;
                }
            }
        }

        "\nThank you for using Job Hunt. Goodbye!\n".to_repl_string().write(writer)?;

        Ok(())
    }
}
