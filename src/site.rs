use crate::repository::Job;

pub const WEB3_JOBS_URL: &str = "https://web3.career/";
pub const CRYPTOCURRENCY_JOBS_URL: &str = "https://cryptocurrencyjobs.co/";

/// All website structs must implement the Site trait and conform to the structure:
/// ```
/// pub struct Website {
///    pub url: &'static str,
///    pub jobs: Vec<jobhunt::repository::Job>,
/// }
/// ```
pub trait Site {
    /// Creates a new instance - default values must be provided in the implementation.
    fn new() -> Self;
}

/// Represents the Web3 Jobs website.
pub struct Web3Jobs {
    pub url: &'static str,
    pub jobs: Vec<Job>,
}

impl Site for Web3Jobs {
    fn new() -> Self { Self { url: WEB3_JOBS_URL, jobs: vec![] } }
}

/// Represents the Cryptocurrency Jobs website.
pub struct CryptocurrencyJobs {
    pub url: &'static str,
    pub jobs: Vec<Job>,
}

impl Site for CryptocurrencyJobs {
    fn new() -> Self { Self { url: CRYPTOCURRENCY_JOBS_URL, jobs: vec![] } }
}
