use crate::repository::{JobRepositoryBuilder, SoftwareJobsBuilder, SoftwareJobs};
use crate::scraper::Scraper;
use crate::site::{Site, UseWeb3, Web3Careers};

pub mod scraper;
pub mod repository;
pub mod site;

pub fn init() -> Result<SoftwareJobs, String> {
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
