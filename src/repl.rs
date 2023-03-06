//! The repl module contains all read/eval print loop (the terminal UI for the application) code.

use std::cmp::Reverse;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::{BufRead, Write};
use colored::Colorize;
use crate::repository::SoftwareJobs;

const PROMPT: &[u8; 3] = b">> ";

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
}

impl Display for ReplString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string.bold().green())
    }
}

/// All job type structs (e.g. SoftwareJobs) must implement the Repl trait.
pub trait Repl {
    /// Initializes a repository for the job type that is implementing this trait;
    /// then initializes the REPL and parses queries.
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
        writer
            .write_all(
                format!(
                    "{}",
                    ReplString::new("Populating/indexing local datastore...\n")
                ).as_bytes()
            )?;
        let mut repo = Self::init_repo();
        writer
            .write_all(
                format!(
                    "{}",
                    ReplString::new(
                        "Population/indexing completed successfully! Please begin your job hunt by entering a query:\n"
                    )
                )
                    .as_bytes()
            )?;
        writer.flush()?;

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
                        writer.write_all(format!("{:?}\n", job).as_bytes())?;
                    }
                    writer
                        .write_all(
                            format!(
                                "{}",
                                ReplString::new(format!("{} items returned\n", repo.all.len()))
                            ).as_bytes()
                        )?;
                    writer.flush()?;
                }
                "exit" => break,
                "refresh" => {
                    writer.write_all(
                        format!("{}", ReplString::new("Refreshing...\n")).as_bytes()
                    )?;
                    repo = Self::init_repo();
                    writer.write_all(
                        format!(
                            "{}",
                            ReplString::new("Refresh completed successfully!\n")
                        ).as_bytes()
                    )?;
                    writer.flush()?;
                }
                _ => {
                    writer
                        .write_all(
                            format!(
                                "{}",
                                ReplString::new(format!("\"{}\" is not a valid command\n", line))
                            ).as_bytes()
                        )?;
                    writer.flush()?;
                }
            }
        }

        writer
            .write_all(
                format!(
                    "{}",
                    ReplString::new("\nThank you for using Job Hunt. Goodbye!\n")
                ).as_bytes()
            )?;
        writer.flush()?;

        Ok(())
    }
}
