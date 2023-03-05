use chrono::{Duration, Local};
use colored::Colorize;
use crate::scraper::Error;
use crate::repository::Job;

/// Job site URLs used for scraping.
pub const WEB3_CAREERS_URL: &str = "https://web3.career/";
pub const USE_WEB3_URL: &str = "https://useweb3.xyz/jobs/t/engineering/";
pub const CRYPTO_JOBS_LIST: &str = "https://cryptojobslist.com/engineering?sort=recent";

/// All website structs must implement the Site trait and conform to the structure:
/// ```
/// pub struct Website {
///    url: &'static str,
///    pub jobs: Vec<jobhunt::repository::Job>,
/// }
/// ```
pub trait Site {
    /// Creates a new instance - default values must be provided in the implementation.
    fn new() -> Self;

    /// Getter for non-public url value.
    fn get_url(&self) -> &'static str;

    /// Prints an error message for the user when a scrape error has occurred and returns a default
    /// for the website type.
    fn default_if_scrape_error(err: Error) -> Self
        where Self: Sized
    {
        let site = Self::new();
        println!(
            "{}",
            format!(
                "There was an error while scraping the site \"{}\".\n{}.\nJob Hunt will not be \
                able to include jobs from this site.",
                site.get_url(), err
            )
                .bold()
                .green()
        );
        site
    }
}

/// Websites structs can implement the Format trait where needed.
pub trait Format {
    /// Formats a date from a given elapsed time string, e.g. "1 hour", "3 days", "today", "3d".
    fn format_date_from(time_elapsed: String) -> String;

    /// Formats a remuneration string.
    fn format_remuneration(r: String) -> String;

    /// Returns a formatted ("%Y-%m-%d") version of now minus a time duration.
    fn sub_duration_and_format(duration: Duration) -> String {
        Local::now().checked_sub_signed(duration).unwrap().format("%Y-%m-%d").to_string()
    }

    /// Returns a formatted ("%Y-%m-%d") version of now.
    fn now_and_format() -> String { Local::now().format("%Y-%m-%d").to_string() }
}

/// Represents the Web3 Careers website.
pub struct Web3Careers {
    url: &'static str,
    pub jobs: Vec<Job>,
}

impl Site for Web3Careers {
    fn new() -> Self { Self { url: WEB3_CAREERS_URL, jobs: Vec::new() } }

    fn get_url(&self) -> &'static str { self.url }
}

/// Represents the Use Web3 Jobs website.
pub struct UseWeb3 {
    url: &'static str,
    pub jobs: Vec<Job>,
}

/// Helper functions for the UseWeb3 website scraper.
impl Format for UseWeb3 {
    fn format_date_from(time_elapsed: String) -> String {
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

    fn format_remuneration(mut r: String) -> String {
        r = r.replace("💰 ", "");
        let rem_v = r.split("-").map(|s| s.trim()).collect::<Vec<&str>>();
        if rem_v.len() != 2 { return "".to_string(); }
        format!("${} - ${}", rem_v[0], rem_v[1]).to_lowercase()
    }
}

impl Site for UseWeb3 {
    fn new() -> Self { Self { url: USE_WEB3_URL, jobs: Vec::new() } }

    fn get_url(&self) -> &'static str { self.url }
}

/// Represents the Crypto Jobs List website.
pub struct CryptoJobsList {
    url: &'static str,
    pub jobs: Vec<Job>,
}

impl Format for CryptoJobsList {
    fn format_date_from(time_elapsed: String) -> String {
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

    fn format_remuneration(mut r: String) -> String {
        r = r.replace("$", "");
        let rem_v = r
            .split("-")
            .map(|s| s.trim())
            .collect::<Vec<&str>>();
        if rem_v.len() != 2 { return "".to_string(); }
        format!("${} - ${}", rem_v[0], rem_v[1])
    }
}

impl Site for CryptoJobsList {
    fn new() -> Self { Self { url: CRYPTO_JOBS_LIST, jobs: Vec::new() } }

    fn get_url(&self) -> &'static str { self.url }
}

#[cfg(test)]
mod tests {
    use chrono::Duration;
    use crate::site::{CryptoJobsList, Format, UseWeb3};

    #[test]
    fn test_use_web3_get_date_from() {
        assert_eq!(
            UseWeb3::format_date_from("3 days".to_string()),
            UseWeb3::sub_duration_and_format(Duration::days(3))
        );
        assert_eq!(
            UseWeb3::format_date_from("1 week".to_string()),
            UseWeb3::sub_duration_and_format(Duration::weeks(1))
        );
        assert_eq!(
            UseWeb3::format_date_from("2 weeks".to_string()),
            UseWeb3::sub_duration_and_format(Duration::weeks(2))
        );
    }

    #[test]
    fn test_crypto_jobs_list_get_date_from() {
        assert_eq!(
            CryptoJobsList::format_date_from("today".to_string()),
            CryptoJobsList::now_and_format()
        );
        assert_eq!(
            CryptoJobsList::format_date_from("1d".to_string()),
            CryptoJobsList::sub_duration_and_format(Duration::days(1))
        );
        assert_eq!(
            CryptoJobsList::format_date_from("2w".to_string()),
            CryptoJobsList::sub_duration_and_format(Duration::weeks(2))
        );
    }

    #[test]
    fn test_use_web3_format_rem_string() {
        assert_eq!(
            UseWeb3::format_remuneration("💰 6K - 7.5K".to_string()),
            "$6k - $7.5k".to_string()
        );
    }

    #[test]
    fn test_crypto_jobs_list_format_rem_string() {
        assert_eq!(
            CryptoJobsList::format_remuneration("$ 90k-140k".to_string()),
            "$90k - $140k".to_string()
        );
    }
}
