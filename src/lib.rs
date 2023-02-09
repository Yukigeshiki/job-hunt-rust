use crate::repository::{JobRepositoryBuilder, SoftwareJobsBuilder, SoftwareJobs};
use crate::scraper::Scraper;
use crate::site::{Site, UseWeb3, Web3Careers};

pub mod scraper;
pub mod repository;
pub mod site;

pub fn init() -> Result<SoftwareJobs, String> {
    let mut web3_careers = Web3Careers::new();
    let mut use_web3 = UseWeb3::new();

    let repo = SoftwareJobsBuilder::new()
        .import(
            vec![
                &mut web3_careers.scrape()?.jobs,
                &mut use_web3.scrape()?.jobs,
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
