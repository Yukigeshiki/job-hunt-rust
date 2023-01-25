use crate::repository::Job;

pub const WEB3_JOBS_URL: &str = "https://web3.career/";

pub trait Site<T> {
    fn new() -> T;
}

pub struct Web3Jobs {
    pub url: &'static str,
    pub jobs: Vec<Job>,
}

impl Site<Web3Jobs> for Web3Jobs {
    fn new() -> Self { Self { url: WEB3_JOBS_URL, jobs: vec![] } }
}
