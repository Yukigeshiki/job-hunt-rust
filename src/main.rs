use jobhunt::repository::Jobs;
use crate::scraper::Scraper;
use jobhunt::scraper;
use jobhunt::site::{Site, Web3Jobs};

fn main() {
    let mut jobs = Jobs::default();
    {
        let mut web3_jobs = Web3Jobs::new();
        jobs
            .aggregate_to_all(
                vec![
                    &mut web3_jobs.scrape().unwrap().jobs
                ]
            )
            .filter_software_jobs()
            .index();
    }

    println!("{:?}", jobs.all);
    println!("{:?}", jobs.date);
    println!("{:?}", jobs.company);
    println!("{:?}", jobs.job_type);
}
