use crate::repository::Job;

pub const WEB3_JOBS_URL: &str = "https://web3.career/";
pub const CRYPTOCURRENCY_JOBS_URL: &str = "https://cryptocurrencyjobs.co/";

pub trait Site {
    fn new() -> Self;
}

pub struct Web3Jobs {
    pub url: &'static str,
    pub jobs: Vec<Job>,
}

impl Site for Web3Jobs {
    fn new() -> Self { Self { url: WEB3_JOBS_URL, jobs: vec![] } }
}

pub struct CryptocurrencyJobs {
    pub url: &'static str,
    pub jobs: Vec<Job>,
}

impl Site for CryptocurrencyJobs {
    fn new() -> Self { Self { url: CRYPTOCURRENCY_JOBS_URL, jobs: vec![] } }
}
