//! The site module contains all website code.

use chrono::{Duration, Local};
use colored::Colorize;

use crate::repository::Job;
use crate::scraper::Error;

/// Job site URLs used for scraping.
pub const WEB3_CAREERS_URL: &str = "https://web3.career";
pub const USE_WEB3_URL: &str = "https://useweb3.xyz/jobs";
pub const CRYPTO_JOBS_LIST_URL: &str = "https://cryptojobslist.com";
pub const SOLANA_JOBS_URL: &str =
    "https://jobs.solana.com/jobs?filter=eyJqb2JfZnVuY3Rpb25zIjpbIlNvZnR3YXJlIEVuZ2luZWVyaW5nIl19";
pub const SUBSTRATE_JOBS_URL: &str =
    "https://careers.substrate.io/jobs?filter=eyJqb2JfZnVuY3Rpb25zIjpbIlNvZnR3YXJlIEVuZ2luZWVyaW5nIl19";
pub const NEAR_JOBS_URL: &str =
    "https://careers.near.org/jobs?filter=eyJqb2JfZnVuY3Rpb25zIjpbIlNvZnR3YXJlIEVuZ2luZWVyaW5nIl19";

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
    where
        Self: Sized,
    {
        let site = Self::new();
        println!(
            "{}",
            format!(
                "There was an error while scraping the site \"{}\".\n{:?}.\nJob Hunt will not be \
                able to include jobs from this site.",
                site.get_url(),
                err
            )
            .bold()
            .green()
        );
        site
    }
}

/// Generates a website struct and implements the Site trait.
macro_rules! generate_website_struct_and_impl {
    ($t:ident, $url:ident) => {
        #[derive(Default)]
        pub struct $t {
            url: &'static str,
            pub jobs: Vec<Job>,
        }

        impl Site for $t {
            fn new() -> Self {
                Self {
                    url: $url,
                    ..Default::default()
                }
            }

            fn get_url(&self) -> &'static str {
                self.url
            }
        }
    };
}

/// Website structs can implement the Formatter trait where needed.
pub trait Formatter {
    /// Formats a date from a given elapsed time string, e.g. "1 hour", "3 days", "today", "3d".
    fn format_date_from(time_elapsed: String) -> String;

    /// Formats a remuneration string.
    fn format_remuneration(r: String) -> String;

    /// Returns a formatted ("%Y-%m-%d") version of now minus a time duration.
    fn sub_duration_and_format(duration: Duration) -> String {
        Local::now()
            .checked_sub_signed(duration)
            .unwrap_or(Local::now())
            .format("%Y-%m-%d")
            .to_string()
    }

    /// Returns a formatted ("%Y-%m-%d") version of now.
    fn now_and_format() -> String {
        Local::now().format("%Y-%m-%d").to_string()
    }
}

// Represents the Web3 Careers website.
generate_website_struct_and_impl!(Web3Careers, WEB3_CAREERS_URL);

impl Web3Careers {
    /// Formats an onclick function (as an &str) into a URL path string.
    pub fn format_apply_link(a: &str) -> String {
        let v = a.split(' ').collect::<Vec<&str>>();
        match v.len() {
            2 => v[1].replace(['\'', ')'], ""),
            _ => "".into(),
        }
    }
}

// Represents the Use Web3 Jobs website.
generate_website_struct_and_impl!(UseWeb3, USE_WEB3_URL);

impl Formatter for UseWeb3 {
    fn format_date_from(time_elapsed: String) -> String {
        let v = time_elapsed.split(' ').collect::<Vec<&str>>();
        let now = Self::now_and_format();
        match v.len() {
            len if len >= 2 => match v[0].parse() {
                Ok(d) => match v[1] {
                    "hour" => Self::sub_duration_and_format(Duration::hours(d)),
                    "hours" => Self::sub_duration_and_format(Duration::hours(d)),
                    "day" => Self::sub_duration_and_format(Duration::days(d)),
                    "days" => Self::sub_duration_and_format(Duration::days(d)),
                    "week" => Self::sub_duration_and_format(Duration::weeks(d)),
                    "weeks" => Self::sub_duration_and_format(Duration::weeks(d)),
                    "month" => Self::sub_duration_and_format(Duration::days(31)),
                    "months" => Self::sub_duration_and_format(Duration::days(d * 30)),
                    _ => now,
                },
                Err(_) => now,
            },
            _ => now,
        }
    }

    fn format_remuneration(mut r: String) -> String {
        r = r.replace("💰 ", "");
        let rem_v = r.split('-').map(|s| s.trim()).collect::<Vec<&str>>();
        match rem_v.len() {
            2 => format!("${} - ${}", rem_v[0], rem_v[1]).to_lowercase(),
            _ => "".into(),
        }
    }
}

// Represents the Crypto Jobs List website.
generate_website_struct_and_impl!(CryptoJobsList, CRYPTO_JOBS_LIST_URL);

impl Formatter for CryptoJobsList {
    fn format_date_from(time_elapsed: String) -> String {
        let v = time_elapsed.chars().collect::<Vec<char>>();
        match v.len() {
            len if len >= 2 => {
                let d: i64 = v[0] as i64 - 0x30;
                match v[1] {
                    'd' => Self::sub_duration_and_format(Duration::days(d)),
                    'w' => Self::sub_duration_and_format(Duration::weeks(d)),
                    'm' => Self::sub_duration_and_format(Duration::days(d * 30)),
                    _ => Self::now_and_format(),
                }
            }
            _ => Self::now_and_format(),
        }
    }

    fn format_remuneration(mut r: String) -> String {
        r = r.replace('$', "");
        let rem_v = r.split('-').map(|s| s.trim()).collect::<Vec<&str>>();
        match rem_v.len() {
            2 => format!("${} - ${}", rem_v[0], rem_v[1]),
            _ => "".into(),
        }
    }
}

// Represents the Solana Jobs website.
generate_website_struct_and_impl!(SolanaJobs, SOLANA_JOBS_URL);

// Represents the Substrate Jobs website.
generate_website_struct_and_impl!(SubstrateJobs, SUBSTRATE_JOBS_URL);

// Represents the Near Jobs website.
generate_website_struct_and_impl!(NearJobs, NEAR_JOBS_URL);

/// Time elapsed and remuneration test examples taken from specific job sites
#[cfg(test)]
mod tests {
    use chrono::Duration;

    use crate::site::{CryptoJobsList, Formatter, UseWeb3};

    #[test]
    fn test_use_web3_get_date_from() {
        assert_eq!(
            UseWeb3::format_date_from("3 days".into()),
            UseWeb3::sub_duration_and_format(Duration::days(3))
        );
        assert_eq!(
            UseWeb3::format_date_from("1 week".into()),
            UseWeb3::sub_duration_and_format(Duration::weeks(1))
        );
        assert_eq!(
            UseWeb3::format_date_from("2 weeks".into()),
            UseWeb3::sub_duration_and_format(Duration::weeks(2))
        );
    }

    #[test]
    fn test_crypto_jobs_list_get_date_from() {
        assert_eq!(
            CryptoJobsList::format_date_from("today".into()),
            CryptoJobsList::now_and_format()
        );
        assert_eq!(
            CryptoJobsList::format_date_from("1d".into()),
            CryptoJobsList::sub_duration_and_format(Duration::days(1))
        );
        assert_eq!(
            CryptoJobsList::format_date_from("2w".into()),
            CryptoJobsList::sub_duration_and_format(Duration::weeks(2))
        );
    }

    #[test]
    fn test_use_web3_format_rem_string() {
        assert_eq!(
            UseWeb3::format_remuneration("💰 6K - 7.5K".into()),
            "$6k - $7.5k"
        );
    }

    #[test]
    fn test_crypto_jobs_list_format_rem_string() {
        assert_eq!(
            CryptoJobsList::format_remuneration("$ 90k-140k".into()),
            "$90k - $140k"
        );
    }
}
