//! The scraper module contains all website scraper code.
//! Websites often change, so the scrapers should be tested often and updated when needed.
//! Currently each scraper only scrapes the first page of it's site; this can be changed by creating
//! a loop and adding a page number query string, e.g. `https://jobsite.com/engineering?page=1` for
//! as many pages as needed.

use regex::Regex;
use scraper::Html;
use scraper::Selector;
use thiserror::Error;

use crate::repository::Job;
use crate::site::{
    CryptoJobsList, Formatter, NearJobs, Site, SolanaJobs, SubstrateJobs, UseWeb3, Web3Careers,
};

type BoxedError = Box<dyn std::error::Error + Send>;

/// Represents specific errors that can occur during the scraping process.
#[derive(Error, Debug)]
pub enum Error<'a> {
    #[error("Selector error: {0}")]
    Selector(String),
    #[error("Could not load url: {0}")]
    Request(#[source] BoxedError),
    #[error("Request failed with code: {0}")]
    Response(u16),
    #[error("Error getting response body: {0}")]
    Parser(#[source] BoxedError),
    #[error("Could not get {0}")]
    Iterator(&'a str),
}

/// All website structs must implement the Scraper trait.
pub trait Scraper {
    /// Scrapes the job website and adds Job instances to the site's jobs array - Job instances must
    /// conform to the structure defined by crate::repository::Job.
    fn scrape(self) -> Result<Self, Error<'static>> where Self: Sized;

    /// A default method. Gets a selector for a specific HTML element.
    fn get_selector(selectors: &str) -> Result<Selector, Error<'static>> {
        Selector::parse(selectors).map_err(|err| Error::Selector(err.to_string()))
    }
}

impl Scraper for Web3Careers {
    fn scrape(mut self) -> Result<Self, Error<'static>> {
        let response = reqwest::blocking::get(self.get_url())
            .map_err(|err| Error::Request(Box::new(err)))?;
        if !response.status().is_success() {
            Err(Error::Response(response.status().as_u16()))?;
        }
        let body = response.text().map_err(|err| Error::Parser(Box::new(err)))?;
        let document = Html::parse_document(&body);

        // HTML selectors
        let table_row_selector = Self::get_selector("tr.table_row")?;
        let td_selector = Self::get_selector("td")?;
        let time_selector = Self::get_selector("time")?;
        let a_selector = Self::get_selector("a")?;

        // iterate through document and populate jobs array
        for el in document.select(&table_row_selector) {
            let mut element_iterator = el.select(&td_selector);

            let title_element = element_iterator
                .next()
                .ok_or(Error::Iterator("job title"))?;
            let title = title_element.text().collect::<String>().trim().to_string();

            let company_element = element_iterator
                .next()
                .ok_or(Error::Iterator("company"))?;
            let company = company_element.text().collect::<String>().trim().to_string();

            let date_posted_element = element_iterator
                .next()
                .ok_or(Error::Iterator("time"))?;
            let date_posted_element_select = date_posted_element
                .select(&time_selector)
                .next()
                .ok_or(Error::Iterator("time"))?;
            let date_posted = date_posted_element_select
                .value()
                .attr("datetime")
                .unwrap_or("")
                .to_string()
                .split(" ")
                .next()
                .unwrap_or("")
                .to_string();

            let location_element = element_iterator
                .next()
                .ok_or(Error::Iterator("location"))?;
            let location = location_element
                .text()
                .collect::<String>()
                .trim()
                .to_string()
                .replace("\n", " ");

            let remuneration_element = element_iterator
                .next()
                .ok_or(Error::Iterator("remuneration"))?;
            let remuneration = remuneration_element.text().collect::<String>().trim().to_string();

            let mut tags = Vec::new();
            let tag_element = element_iterator.next().ok_or(Error::Iterator("tags"))?;
            tag_element
                .select(&a_selector)
                .for_each(|tag| tags.push(tag.text().collect::<String>().trim().to_string()));

            self.jobs.push(
                Job {
                    title,
                    company,
                    date_posted,
                    location,
                    remuneration,
                    tags,
                    site: self.get_url(),
                }
            );
        };

        Ok(self)
    }
}

impl Scraper for UseWeb3 {
    fn scrape(mut self) -> Result<Self, Error<'static>> {
        let response = reqwest::blocking::get(self.get_url())
            .map_err(|err| Error::Request(Box::new(err)))?;
        if !response.status().is_success() {
            Err(Error::Response(response.status().as_u16()))?;
        }
        let body = response.text().map_err(|err| Error::Parser(Box::new(err)))?;
        let document = Html::parse_document(&body);

        // HTML selectors
        let panel_inner_selector = Self::get_selector("div.panel_inner__YQLRW")?;
        let a_selector = Self::get_selector("a")?;
        let span_selector = Self::get_selector("span")?;
        let panel_border_selector = Self::get_selector("div.panel_border___58nj")?;

        for el in document.select(&panel_inner_selector) {
            let mut element_iterator = el.select(&a_selector);

            let title_element = element_iterator
                .next()
                .ok_or(Error::Iterator("job title"))?;
            let title = title_element.text().collect::<String>().trim().to_string();

            let company_element = element_iterator
                .next()
                .ok_or(Error::Iterator("company"))?;
            let company = company_element.text().collect::<String>().trim().to_string();

            let mut element_iterator = el.select(&span_selector);

            let location_element = element_iterator
                .next()
                .ok_or(Error::Iterator("location"))?;
            let mut location = location_element.text().collect::<String>().trim().to_string();

            let time_elapsed_element = element_iterator
                .next()
                .ok_or(Error::Iterator("elapsed time"))?;
            let time_elapsed = time_elapsed_element.text().collect::<String>().trim().to_string();
            let date_posted = Self::format_date_from(time_elapsed);

            let mut remuneration = "".to_string();
            el
                .select(&panel_border_selector)
                .for_each(|item| {
                    let i = item.text().collect::<String>().trim().to_string();
                    if i.contains("????") && !location.to_lowercase().contains("remote") {
                        location = format!("{}, {}", location, i.replace("???? ", ""));
                    }
                    if i.contains("????") {
                        remuneration = Self::format_remuneration(i);
                    }
                });

            self.jobs.push(
                Job {
                    title,
                    company,
                    date_posted,
                    location,
                    remuneration,
                    tags: Vec::new(),
                    site: self.get_url(),
                }
            );
        }

        Ok(self)
    }
}

impl Scraper for CryptoJobsList {
    fn scrape(mut self) -> Result<Self, Error<'static>> {
        let response = reqwest::blocking::get(self.get_url())
            .map_err(|err| Error::Request(Box::new(err)))?;
        if !response.status().is_success() {
            Err(Error::Response(response.status().as_u16()))?;
        }
        let body = response.text().map_err(|err| Error::Parser(Box::new(err)))?;
        let document = Html::parse_document(&body);

        // HTML selectors
        let li_selector = Self::get_selector("section ul li")?;
        let a_selector = Self::get_selector("a")?;
        let span_selector = Self::get_selector("span span span")?;
        let span_a_selector = Self::get_selector("span span a")?;
        let span_class_selector = Self::get_selector("span.JobPreviewInline_createdAt__wbWS0")?;

        for el in document.select(&li_selector) {
            let mut a_element = el.select(&a_selector);

            let title_element = a_element.next().ok_or(Error::Iterator("job title"))?;
            let title = title_element.text().collect::<String>().trim().to_string();

            let company_element = a_element.next().ok_or(Error::Iterator("company"))?;
            let company = company_element.text().collect::<String>().trim().to_string();

            let mut span_class_element = el.select(&span_class_selector);
            let time_elapsed_element = span_class_element
                .next()
                .ok_or(Error::Iterator("elapsed time"))?;
            let time_elapsed = time_elapsed_element.text().collect::<String>().trim().to_string();
            let date_posted = Self::format_date_from(time_elapsed);

            let mut span_element = el.select(&span_selector);
            let onsite_or_rem_element = span_element
                .next()
                .ok_or(Error::Iterator("location or remuneration"))?;
            let onsite_or_rem = onsite_or_rem_element.text().collect::<String>().trim().to_string();
            let mut remuneration = "".to_string();
            let mut onsite = "".to_string();
            if onsite_or_rem.contains("$") {
                remuneration = Self::format_remuneration(onsite_or_rem);
            } else if !Regex::new(r"[0-9]").unwrap().is_match(&onsite_or_rem) {
                onsite = onsite_or_rem;
            }

            let mut tags = Vec::new();
            el
                .select(&span_a_selector)
                .for_each(|tag| tags.push(tag.text().collect::<String>().trim().to_string()));

            let remote_string = "Remote".to_string();
            let location = if !onsite.is_empty() && tags.contains(&remote_string) {
                format!("{}, {}", onsite, remote_string)
            } else if tags.contains(&remote_string) {
                remote_string
            } else {
                onsite
            };

            self.jobs.push(
                Job {
                    title,
                    company,
                    date_posted,
                    location,
                    remuneration,
                    tags,
                    site: self.get_url(),
                }
            );
        }

        Ok(self)
    }
}

/// Provides a common scrape implementation for a number of web3/blockchain job sites built with the
/// same HTML structure.
trait Common {
    type Input: Site + Scraper;

    /// Returns a selector from the Input type's `get_selector` method.
    fn _get_selector(selectors: &str) -> Result<Selector, Error<'static>>;

    /// A common scrape implementation for a number of web3/blockchain job sites.
    fn _scrape(input: &Self::Input) -> Result<Vec<Job>, Error<'static>> {
        let mut jobs = vec![];
        let response = reqwest::blocking::get(input.get_url())
            .map_err(|err| Error::Request(Box::new(err)))?;
        if !response.status().is_success() {
            Err(Error::Response(response.status().as_u16()))?;
        }
        let body = response.text().map_err(|err| Error::Parser(Box::new(err)))?;
        let document = Html::parse_document(&body);

        // HTML selectors
        let div1_selector = Self::_get_selector(
            "div.infinite-scroll-component__outerdiv>div>div"
        )?;
        let div2_selector = Self::_get_selector(r#"div[itemprop=title]"#)?;
        let meta1_selector = Self::_get_selector(r#"meta[itemprop=name]"#)?;
        let span_selector = Self::_get_selector("span")?;
        let meta2_selector = Self::_get_selector(r#"meta[itemprop=datePosted]"#)?;

        for el in document.select(&div1_selector) {
            let mut div2_selector = el.select(&div2_selector);

            if let Some(element) = div2_selector.next() {
                let title = element.text().collect::<String>().trim().to_string();

                let mut meta1_element = el.select(&meta1_selector);
                let company_element = meta1_element.next().ok_or(Error::Iterator("company"))?;
                let company = company_element
                    .value()
                    .attr("content")
                    .unwrap_or("")
                    .to_string();

                let mut span_element = el.select(&span_selector);
                let remuneration = "".to_string();
                let mut location = "".to_string();
                if let Some(element) = span_element.next() {
                    location = element.text().collect::<String>().trim().to_string();
                    if let Some(element) = span_element.next() {
                        location = format!(
                            "{}, {}",
                            location,
                            element.text().collect::<String>().trim().to_string()
                        );
                    }
                }

                let mut meta2_element = el.select(&meta2_selector);
                let date_posted_element = meta2_element
                    .next()
                    .ok_or(Error::Iterator("date posted"))?;
                let date_posted = date_posted_element
                    .value()
                    .attr("content")
                    .unwrap_or("")
                    .to_string();

                jobs.push(
                    Job {
                        title,
                        company,
                        date_posted,
                        location,
                        remuneration,
                        tags: Vec::new(),
                        site: input.get_url(),
                    }
                );
            }
        }

        Ok(jobs)
    }
}

impl Common for SolanaJobs {
    type Input = SolanaJobs;

    fn _get_selector(selectors: &str) -> Result<Selector, Error<'static>> {
        Self::Input::get_selector(selectors)
    }
}

impl Scraper for SolanaJobs {
    fn scrape(mut self) -> Result<Self, Error<'static>> {
        self.jobs = Self::_scrape(&self)?;
        Ok(self)
    }
}

impl Common for SubstrateJobs {
    type Input = SubstrateJobs;

    fn _get_selector(selectors: &str) -> Result<Selector, Error<'static>> {
        Self::Input::get_selector(selectors)
    }
}

impl Scraper for SubstrateJobs {
    fn scrape(mut self) -> Result<Self, Error<'static>> {
        self.jobs = Self::_scrape(&self)?;
        Ok(self)
    }
}

impl Common for NearJobs {
    type Input = NearJobs;

    fn _get_selector(selectors: &str) -> Result<Selector, Error<'static>> {
        Self::Input::get_selector(selectors)
    }
}

impl Scraper for NearJobs {
    fn scrape(mut self) -> Result<Self, Error<'static>> {
        self.jobs = Self::_scrape(&self)?;
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    use crate::repository::Job;
    use crate::site::{
        CRYPTO_JOBS_LIST_URL, CryptoJobsList,
        NEAR_JOBS_URL, NearJobs,
        Site,
        SOLANA_JOBS_URL, SolanaJobs,
        SUBSTRATE_JOBS_URL, SubstrateJobs,
        USE_WEB3_URL, UseWeb3,
        WEB3_CAREERS_URL,
        Web3Careers,
    };

    use super::Scraper;

    const DATE_REGEX: &str = r"(\d{4})-(\d{2})-(\d{2})( (\d{2}):(\d{2}):(\d{2}))?";

    #[test]
    fn test_scrape_web3careers() {
        let jobs = Web3Careers::new().scrape().unwrap().jobs;
        assert_eq!(jobs[0].site, WEB3_CAREERS_URL);
        job_assertions(jobs)
    }

    #[test]
    fn test_scrape_use_web3() {
        let jobs = UseWeb3::new().scrape().unwrap().jobs;
        assert_eq!(jobs[0].site, USE_WEB3_URL);
        job_assertions(jobs)
    }

    #[test]
    fn test_scrape_crypto_jobs_list() {
        let jobs = CryptoJobsList::new().scrape().unwrap().jobs;
        assert_eq!(jobs[0].site, CRYPTO_JOBS_LIST_URL);
        job_assertions(jobs)
    }

    #[test]
    fn test_scrape_solana_jobs() {
        let jobs = SolanaJobs::new().scrape().unwrap().jobs;
        assert_eq!(jobs[0].site, SOLANA_JOBS_URL);
        job_assertions(jobs)
    }

    #[test]
    fn test_scrape_substrate_jobs() {
        let jobs = SubstrateJobs::new().scrape().unwrap().jobs;
        assert_eq!(jobs[0].site, SUBSTRATE_JOBS_URL);
        job_assertions(jobs)
    }

    #[test]
    fn test_scrape_near_jobs() {
        let jobs = NearJobs::new().scrape().unwrap().jobs;
        assert_eq!(jobs[0].site, NEAR_JOBS_URL);
        job_assertions(jobs)
    }

    fn job_assertions(jobs: Vec<Job>) {
        assert!(jobs.len() > 0);
        jobs
            .iter()
            .for_each(|job| {
                assert!(!job.title.is_empty());
                assert!(!job.company.is_empty());
                assert!(
                    Regex::new(DATE_REGEX)
                        .unwrap()
                        .is_match(&job.date_posted)
                );
                assert!(
                    job.remuneration.to_lowercase().contains("k")
                        && job.remuneration.to_lowercase().contains("$")
                        || job.remuneration.is_empty())
            })
    }
}
