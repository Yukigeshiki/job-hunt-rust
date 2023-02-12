use std::fmt::Debug;
use crate::repository::{JobRepositoryBuilder, SoftwareJobsBuilder, SoftwareJobs};
use crate::scraper::Scraper;
use crate::site::{Site, UseWeb3, Web3Careers};

pub mod scraper;
pub mod repository;
pub mod site;

/// Job Hunt can be initialized with any job repository type by implementing this trait.
pub trait Initializer {
    /// The repository type to initialise Job Hunt with.
    type Output: Debug;

    /// Called to build a repository of type Output using it's accompanying builder.
    fn init() -> Result<Self::Output, String>;
}

/// Represents an initializer for software jobs.
pub struct SoftwareJobsInitializer {}

impl Initializer for SoftwareJobsInitializer {
    type Output = SoftwareJobs;

    fn init() -> Result<SoftwareJobs, String> {
        let repo = SoftwareJobsBuilder::new()
            .import(
                vec![
                    Web3Careers::new().scrape()?.jobs,
                    UseWeb3::new().scrape()?.jobs,
                ]
            )
            .filter(
                |job|
                    job.title.to_lowercase().contains("developer") ||
                        job.title.to_lowercase().contains("engineer") ||
                        job.title.to_lowercase().contains("engineering")
            ) // optional filter - in this case filter on software jobs
            .index();

        Ok(repo)
    }
}
