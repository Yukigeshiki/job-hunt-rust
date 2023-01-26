use jobhunt::repository::{SoftwareJobs, JobRepository};
use crate::scraper::Scraper;
use jobhunt::scraper;
use jobhunt::site::{Site, Web3Jobs};

fn main() {
    let mut repo = SoftwareJobs::default();
    {
        let mut web3_jobs = Web3Jobs::new();
        repo
            .import(
                vec![
                    &mut web3_jobs.scrape().unwrap().jobs
                ]
            )
            .filter(
                |job|
                    job.title.to_lowercase().contains("developer") ||
                        job.title.to_lowercase().contains("engineer") ||
                        job.title.to_lowercase().contains("engineering")
            ) // optional filter - in this case filter on software jobs
            .index();
    }

    println!("{:?}", repo.all);
    println!("{:?}", repo.date);
    println!("{:?}", repo.company);
    println!("{:?}", repo.location);
    println!("{:?}", repo.skill);
    println!("{:?}", repo.level);
}
