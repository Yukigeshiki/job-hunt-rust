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

    /// Prints an error message for the user and returns a default for the website type.
    fn default_if_scrape_error(err: String) -> Self;

    fn sub_duration_and_format(duration: Duration) -> String {
        Local::now().checked_sub_signed(duration).unwrap().format("%Y-%m-%d").to_string()
    }

    fn now_and_format() -> String { Local::now().format("%Y-%m-%d").to_string() }
}

/// Represents the Web3 Careers website.
pub struct Web3Careers {
    pub url: &'static str,
    pub jobs: Vec<Job>,
}

impl Site for Web3Careers {
    fn new() -> Self { Self { url: WEB3_CAREERS_URL, jobs: Vec::new() } }

    /// This function currently exists per website. This should be moved to a custom derive
    /// macro later.
    fn default_if_scrape_error(err: String) -> Self {
        let def = Self::new();
        println!(
            "{}",
            format!(
                "There has was an error while scraping the site \"{}\": {}.\nJob Hunt will not be \
                able to include jobs from this site.",
                def.url, err
            ).bold().green()
        );
        def
    }
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

    fn default_if_scrape_error(err: String) -> Self {
        let def = Self::new();
        println!(
            "{}",
            format!(
                "There was an error while scraping the site \"{}\": {}.\nJob Hunt will not be \
                able to include jobs from this site.",
                def.url, err
            ).bold().green()
        );
        def
    }
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
        let d: i64 = v[0] as i64;
        match v[1].to_string().as_str() {
            "d" => Self::sub_duration_and_format(Duration::hours(d)),
            "w" => Self::sub_duration_and_format(Duration::hours(d)),
            "m" => Self::sub_duration_and_format(Duration::hours(d)),
            _ => Self::now_and_format()
        }
    }
}

impl Site for CryptoJobsList {
    fn new() -> Self { Self { url: CRYPTO_JOBS_LIST, jobs: Vec::new() } }

    fn default_if_scrape_error(err: String) -> Self {
        let def = Self::new();
        println!(
            "{}",
            format!(
                "There has was an error while scraping the site \"{}\": {}.\nJob Hunt will not be \
                able to include jobs from this site.",
                def.url, err
            ).bold().green()
        );
        def
    }
}
