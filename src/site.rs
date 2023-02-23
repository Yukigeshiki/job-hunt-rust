use chrono::{Duration, Local};
use colored::Colorize;
use crate::repository::Job;

pub const WEB3_CAREERS_URL: &str = "https://web3.career/";
pub const USE_WEB3_URL: &str = "https://useweb3.xyz/jobs/t/engineering/";
pub const CRYPTO_JOBS_LIST: &str = "https://cryptojobslist.com/engineering?sort=recent";

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

    /// Returns a formatted ("%Y-%m-%d") version of now minus a time duration.
    fn sub_duration_and_format(duration: Duration) -> String {
        Local::now().checked_sub_signed(duration).unwrap().format("%Y-%m-%d").to_string()
    }

    /// Returns a formatted ("%Y-%m-%d") version of now.
    fn now_and_format() -> String { Local::now().format("%Y-%m-%d").to_string() }

    /// Prints an error message for the user when a scrape error has occurred and returns a default
    /// for the website type.
    fn default_if_scrape_error(url: &str, err: String) -> Self
        where Self: Sized
    {
        println!(
            "{}",
            format!(
                "There has was an error while scraping the site \"{}\": {}.\nJob Hunt will not be \
                able to include jobs from this site.",
                url, err
            )
                .bold()
                .green()
        );
        Self::new()
    }
}

/// Represents the Web3 Careers website.
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

/// Helper functions for the UseWeb3 website scraper.
impl UseWeb3 {
    /// Gets a formatted date from an elapsed time string, eg. "1 hour", "3 days".
    pub fn get_date_from(time_elapsed: String) -> String {
        let v = time_elapsed.split(" ").collect::<Vec<&str>>();
        if v.len() < 2 { return Self::now_and_format(); }
        let d: i64 = v[0].parse().unwrap_or(0);
        match v[1] {
            "hour" => Self::sub_duration_and_format(Duration::hours(d)),
            "hours" => Self::sub_duration_and_format(Duration::hours(d)),
            "day" => Self::sub_duration_and_format(Duration::days(d)),
            "days" => Self::sub_duration_and_format(Duration::days(d)),
            "week" => Self::sub_duration_and_format(Duration::weeks(d)),
            "weeks" => Self::sub_duration_and_format(Duration::weeks(d)),
            "month" => Self::sub_duration_and_format(Duration::days(31)),
            "months" => Self::sub_duration_and_format(Duration::days(d * 30)),
            _ => Self::now_and_format()
        }
    }
}

impl Site for UseWeb3 {
    fn new() -> Self { Self { url: USE_WEB3_URL, jobs: Vec::new() } }
}

/// Represents the Crypto Jobs List website.
pub struct CryptoJobsList {
    pub url: &'static str,
    pub jobs: Vec<Job>,
}

impl CryptoJobsList {
    /// Gets a formatted date from an elapsed time string, eg. "today", "3d".
    pub fn get_date_from(time_elapsed: String) -> String {
        let v = time_elapsed.chars().collect::<Vec<char>>();
        if v.len() > 2 { return Self::now_and_format(); }
        let d: i64 = v[0] as i64 - 0x30;
        match v[1].to_string().as_str() {
            "d" => Self::sub_duration_and_format(Duration::days(d)),
            "w" => Self::sub_duration_and_format(Duration::weeks(d)),
            "m" => Self::sub_duration_and_format(Duration::days(d * 30)),
            _ => Self::now_and_format()
        }
    }
}

impl Site for CryptoJobsList {
    fn new() -> Self { Self { url: CRYPTO_JOBS_LIST, jobs: Vec::new() } }
}

#[cfg(test)]
mod tests {
    use chrono::Duration;
    use crate::site::{CryptoJobsList, Site, UseWeb3};

    #[test]
    fn test_use_web3_get_date_from() {
        let date1 = UseWeb3::get_date_from("3 days".to_string());
        let date2 = UseWeb3::get_date_from("1 week".to_string());
        let date3 = UseWeb3::get_date_from("2 weeks".to_string());
        assert_eq!(date1, UseWeb3::sub_duration_and_format(Duration::days(3)));
        assert_eq!(date2, UseWeb3::sub_duration_and_format(Duration::weeks(1)));
        assert_eq!(date3, UseWeb3::sub_duration_and_format(Duration::weeks(2)));
    }

    #[test]
    fn test_crypto_jobs_list_get_date_from() {
        let date1 = CryptoJobsList::get_date_from("today".to_string());
        let date2 = CryptoJobsList::get_date_from("1d".to_string());
        let date3 = CryptoJobsList::get_date_from("2w".to_string());
        assert_eq!(date1, CryptoJobsList::now_and_format());
        assert_eq!(date2, CryptoJobsList::sub_duration_and_format(Duration::days(1)));
        assert_eq!(date3, CryptoJobsList::sub_duration_and_format(Duration::weeks(2)));
    }
}
