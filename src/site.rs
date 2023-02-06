use chrono::{Duration, Utc};
use crate::repository::Job;

pub const WEB3_JOBS_URL: &str = "https://web3.career/";
pub const USE_WEB3_URL: &str = "https://useweb3.xyz/jobs/t/engineering/";

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
    fn new() -> Self { Self { url: WEB3_JOBS_URL, jobs: Vec::new() } }
}

/// Represents the Use Web3 Jobs website.
pub struct UseWeb3 {
    pub url: &'static str,
    pub jobs: Vec<Job>,
}

impl Site for UseWeb3 {
    fn new() -> Self { Self { url: USE_WEB3_URL, jobs: Vec::new() } }
}

/// Helper functions for the UseWeb3 website scraper.
impl UseWeb3 {
    pub fn get_date(time_elapsed: String) -> String {
        let v = time_elapsed.split(" ").collect::<Vec<&str>>();
        if v[1] == "hours" {
            Self::sub_and_format(Duration::hours(v[0].parse().unwrap()))
        } else if v[1] == "days" {
            Self::sub_and_format(Duration::days(v[0].parse().unwrap()))
        } else if v[1] == "months" {
            Self::sub_and_format(Duration::days(v[0].parse::<i64>().unwrap() * 30 as i64)) // estimate
        } else {
            "".to_string()
        }
    }

    fn sub_and_format(duration: Duration) -> String {
        Utc::now().checked_sub_signed(duration).unwrap().format("%Y-%m-%d").to_string()
    }
}
