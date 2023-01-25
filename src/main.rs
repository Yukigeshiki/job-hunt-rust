use crate::scraper::Scraper;
use jobhunt::scraper;
use jobhunt::site::{Site, Web3Jobs};

fn main() {
    let mut web3_jobs = Web3Jobs::new();
    web3_jobs.scrape().unwrap();
    println!("{:?}", web3_jobs.jobs);
}
