use jobhunt::repository::{SoftwareJobs, Repository};
use crate::scraper::Scraper;
use jobhunt::scraper;
use jobhunt::site::{Site, Web3Jobs};

fn main() {
    let mut repo = SoftwareJobs::default();
    {
        let mut web3_jobs = Web3Jobs::new();
        repo
            .import_into_repository(
                vec![
                    &mut web3_jobs.scrape().unwrap().jobs
                ]
            )
            .filter_job_type() // in this case software jobs
            .index();
    }

    println!("{:?}", repo.all);
    println!("{:?}", repo.date);
    println!("{:?}", repo.company);
    println!("{:?}", repo.skill);
    println!("{:?}", repo.level);
}
