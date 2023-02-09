use chrono::{Duration, Utc};
use crate::repository::Job;

pub const WEB3_CAREERS_URL: &str = "https://web3.career/";
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
pub struct Web3Careers {
    pub url: &'static str,
    pub jobs: Vec<Job>,
}

impl Site for Web3Careers {
    fn new() -> Self { Self { url: WEB3_CAREERS_URL, jobs: Vec::new() } }
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
    pub fn get_date_from(time_elapsed: String) -> String {
        let v = time_elapsed.split(" ").collect::<Vec<&str>>();
        match v[1] {
            "hour" => Self::sub_duration_and_format(Duration::hours(1)),
            "hours" => Self::sub_duration_and_format(Duration::hours(v[0].parse().unwrap())),
            "day" => Self::sub_duration_and_format(Duration::days(1)),
            "days" => Self::sub_duration_and_format(Duration::days(v[0].parse().unwrap())),
            "week" => Self::sub_duration_and_format(Duration::weeks(1)),
            "weeks" => Self::sub_duration_and_format(Duration::weeks(v[0].parse().unwrap())),
            "month" => Self::sub_duration_and_format(Duration::days(31)), // estimate
            "months" => Self::sub_duration_and_format(Duration::days(v[0].parse::<i64>().unwrap() * 30)), // estimate
            _ => Self::sub_duration_and_format(Duration::days(0))
        }
    }

    fn sub_duration_and_format(duration: Duration) -> String {
        Utc::now().checked_sub_signed(duration).unwrap().format("%Y-%m-%d").to_string()
    }
}
