use jobhunt::repository::{SoftwareJobs, JobRepository};
use crate::scraper::Scraper;
use jobhunt::scraper;
use jobhunt::site::{UseWeb3, Site, Web3Careers};

fn main() {
    let mut repo = SoftwareJobs::new();
    {
        let mut web3_careers = Web3Careers::new();
        let mut use_web3 = UseWeb3::new();
        repo
            .import(
                vec![
                    &mut web3_careers.scrape().unwrap().jobs,
                    &mut use_web3.scrape().unwrap().jobs,
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

    println!("{}", repo.all.len());
    println!("{:?}", repo.all);
    println!("{:?}", repo.date);
    println!("{:?}", repo.company);
    println!("{:?}", repo.location);
    println!("{:?}", repo.skill);
    println!("{:?}", repo.level);
}
