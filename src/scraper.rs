//! The scraper module contains all website scraper code.
//! Websites often change, so the scrapers should be tested often and updated when needed.
//! Currently most scrapers only scrape the first page of their site; this can be changed by creating
//! a loop and adding a page number query string, e.g. `https://jobsite.com/engineering?page=1` for
//! as many pages as required.

use std::thread;

use itertools::Itertools;
use regex::Regex;
use scraper::Html;
use scraper::Selector;
use thiserror::Error;

use crate::repository::{Job, THREAD_ERROR};
use crate::site::{
    CryptoJobsList, Formatter, NearJobs, Site, SolanaJobs, SubstrateJobs, UseWeb3, Web3Careers,
};

type BoxedError = Box<dyn std::error::Error + Send>;

/// Represents specific errors that can occur during the scraping process.
#[derive(Error, Debug)]
pub enum Error {
    #[error("Selector error: {0}")]
    Selector(String),
    #[error("Could not load url: {0}")]
    Request(#[source] BoxedError),
    #[error("Request failed with code: {0}")]
    Response(u16),
    #[error("Error getting response body: {0}")]
    Parser(#[source] BoxedError),
    #[error("Could not get {0}")]
    Iterator(&'static str),
}

/// All website structs must implement the Scraper trait.
pub trait Scraper {
    /// Scrapes the job website and adds Job instances to the site's jobs array - Job instances have
    /// the structure:
    /// ```
    ///struct Job {
    ///     pub title: String,
    ///     pub company: String,
    ///     pub date_posted: String,
    ///     pub location: String,
    ///     pub remuneration: String,
    ///     pub tags: Vec<String>,
    ///     pub apply: String,
    ///     pub site: &'static str,
    /// }
    /// ```
    /// as defined in repository module.
    fn scrape(self) -> Result<Self, Error>
    where
        Self: Sized;

    /// A default method. Gets a selector for a specific HTML element.
    fn get_selector(selectors: &str) -> Result<Selector, Error> {
        Selector::parse(selectors).map_err(|err| Error::Selector(err.to_string()))
    }
}

impl Web3Careers {
    /// A stand alone scrape function for Web3Careers that can be moved into a new thread.
    /// This function is used to scrape a specific page, e.g. .../?page=1.
    fn _scrape(i: i32, site: &'static str) -> Result<Vec<Job>, Error> {
        let mut jobs = vec![];
        let response = reqwest::blocking::get(format!("{}?page={}", site, i))
            .map_err(|err| Error::Request(Box::new(err)))?;
        if !response.status().is_success() {
            Err(Error::Response(response.status().as_u16()))?;
        }
        let body = response
            .text()
            .map_err(|err| Error::Parser(Box::new(err)))?;
        let document = Html::parse_document(&body);

        // HTML selectors
        let table_row_selector = Self::get_selector("tr.table_row")?;
        let td_selector = Self::get_selector("td")?;
        let time_selector = Self::get_selector("time")?;
        let a_selector = Self::get_selector("a")?;

        for el in document.select(&table_row_selector) {
            let apply = format!(
                "{}{}",
                site,
                Self::format_apply_link(el.value().attr("onclick").unwrap_or(""))
            );

            let mut element_iterator = el.select(&td_selector);

            let title_element = element_iterator
                .next()
                .ok_or(Error::Iterator("job title"))?;
            let title = title_element.text().collect::<String>().trim().to_owned();

            let company_element = element_iterator.next().ok_or(Error::Iterator("company"))?;
            let company = company_element.text().collect::<String>().trim().to_owned();

            let date_posted_element = element_iterator.next().ok_or(Error::Iterator("time"))?;
            let date_posted_element = date_posted_element
                .select(&time_selector)
                .next()
                .ok_or(Error::Iterator("time"))?;
            let date_posted = date_posted_element
                .value()
                .attr("datetime")
                .unwrap_or("")
                .split(' ')
                .next()
                .unwrap_or("")
                .to_owned();

            let location_element = element_iterator.next().ok_or(Error::Iterator("location"))?;
            let location = location_element
                .text()
                .collect::<String>()
                .trim()
                .replace('\n', " ");

            let remuneration_element = element_iterator
                .next()
                .ok_or(Error::Iterator("remuneration"))?;
            let remuneration = remuneration_element
                .text()
                .collect::<String>()
                .trim()
                .to_owned();

            let mut tags = Vec::new();
            let tag_element = element_iterator.next().ok_or(Error::Iterator("tags"))?;
            tag_element
                .select(&a_selector)
                .for_each(|tag| tags.push(tag.text().collect::<String>().trim().to_owned()));

            jobs.push(Job {
                title,
                company,
                date_posted,
                location,
                remuneration,
                tags,
                apply,
                site,
            });
        }

        Ok(jobs)
    }
}

impl Scraper for Web3Careers {
    fn scrape(mut self) -> Result<Self, Error> {
        let mut handles = vec![];
        let url = self.get_url();

        (1..6).for_each(|i| handles.push(thread::spawn(move || Self::_scrape(i, url))));
        for h in handles {
            self.jobs.extend(h.join().expect(THREAD_ERROR)?)
        }

        self.jobs = self.jobs.into_iter().unique().collect();

        Ok(self)
    }
}

impl Scraper for UseWeb3 {
    fn scrape(mut self) -> Result<Self, Error> {
        let response = reqwest::blocking::get(format!("{}{}", self.get_url(), "/t/engineering/"))
            .map_err(|err| Error::Request(Box::new(err)))?;
        if !response.status().is_success() {
            Err(Error::Response(response.status().as_u16()))?;
        }
        let body = response
            .text()
            .map_err(|err| Error::Parser(Box::new(err)))?;
        let document = Html::parse_document(&body);

        // HTML selectors
        let panel_inner_selector = Self::get_selector("div.panel_inner__YQLRW")?;
        let panel_actions_selector = Self::get_selector("div.panel_actions__T498Q>div>a")?;
        let a_selector = Self::get_selector("a")?;
        let span_selector = Self::get_selector("span")?;
        let panel_border_selector = Self::get_selector("div.panel_border___58nj")?;

        for el in document.select(&panel_inner_selector) {
            let mut element_iterator = el.select(&a_selector);

            let title_element = element_iterator
                .next()
                .ok_or(Error::Iterator("job title"))?;
            let title = title_element.text().collect::<String>().trim().to_string();

            let company_element = element_iterator.next().ok_or(Error::Iterator("company"))?;
            let company = company_element.text().collect::<String>().trim().to_owned();

            let mut element_iterator = el.select(&span_selector);

            let location_element = element_iterator.next().ok_or(Error::Iterator("location"))?;
            let mut location = location_element
                .text()
                .collect::<String>()
                .trim()
                .to_owned();

            let time_elapsed_element = element_iterator
                .next()
                .ok_or(Error::Iterator("elapsed time"))?;
            let time_elapsed = time_elapsed_element
                .text()
                .collect::<String>()
                .trim()
                .to_owned();
            let date_posted = Self::format_date_from(time_elapsed);

            let mut remuneration = "".to_string();
            el.select(&panel_border_selector).for_each(|item| {
                let i = item.text().collect::<String>().trim().to_owned();
                if i.contains('🌐') && !location.to_lowercase().contains("remote") {
                    location = format!("{}, {}", location, i.replace("🌐 ", ""));
                }
                if i.contains('💰') {
                    remuneration = Self::format_remuneration(i);
                }
            });

            let mut apply_iterator = el.select(&panel_actions_selector);
            let apply_element = apply_iterator.next().ok_or(Error::Iterator("apply link"))?;
            let apply = apply_element.value().attr("href").unwrap_or("").to_owned();

            self.jobs.push(Job {
                title,
                company,
                date_posted,
                location,
                remuneration,
                tags: Vec::new(),
                apply,
                site: self.get_url(),
            });
        }

        self.jobs = self.jobs.into_iter().unique().collect();

        Ok(self)
    }
}

impl Scraper for CryptoJobsList {
    fn scrape(mut self) -> Result<Self, Error> {
        let response =
            reqwest::blocking::get(format!("{}{}", self.get_url(), "/engineering?sort=recent"))
                .map_err(|err| Error::Request(Box::new(err)))?;
        if !response.status().is_success() {
            Err(Error::Response(response.status().as_u16()))?;
        }
        let body = response
            .text()
            .map_err(|err| Error::Parser(Box::new(err)))?;
        let document = Html::parse_document(&body);

        // HTML selectors
        let li_selector = Self::get_selector("section>ul>li")?;
        let a_selector = Self::get_selector("a")?;
        let span_selector = Self::get_selector("span>span>span")?;
        let span_a_selector = Self::get_selector("span>span>a")?;
        let span_class_selector = Self::get_selector("span.JobPreviewInline_createdAt__wbWS0")?;

        for el in document.select(&li_selector) {
            let mut a_element = el.select(&a_selector);

            let title_element = a_element.next().ok_or(Error::Iterator("job title"))?;
            let title = title_element.text().collect::<String>().trim().to_owned();

            let apply = format!(
                "{}{}",
                self.get_url(),
                title_element.value().attr("href").unwrap_or("")
            );

            let company_element = a_element.next().ok_or(Error::Iterator("company"))?;
            let company = company_element.text().collect::<String>().trim().to_owned();

            let mut span_class_element = el.select(&span_class_selector);
            let time_elapsed_element = span_class_element
                .next()
                .ok_or(Error::Iterator("elapsed time"))?;
            let time_elapsed = time_elapsed_element
                .text()
                .collect::<String>()
                .trim()
                .to_owned();

            let date_posted = Self::format_date_from(time_elapsed);

            let mut span_element = el.select(&span_selector);
            let onsite_or_rem_element = span_element
                .next()
                .ok_or(Error::Iterator("location or remuneration"))?;
            let onsite_or_rem = onsite_or_rem_element
                .text()
                .collect::<String>()
                .trim()
                .to_owned();
            let mut remuneration = "".to_string();
            let mut onsite = "".to_string();
            if onsite_or_rem.contains('$') {
                remuneration = Self::format_remuneration(onsite_or_rem);
            } else if !Regex::new(r"[0-9]").unwrap().is_match(&onsite_or_rem)
                && onsite_or_rem != "Be the first to apply!"
            {
                onsite = onsite_or_rem;
            }

            let mut tags = Vec::new();
            el.select(&span_a_selector)
                .for_each(|tag| tags.push(tag.text().collect::<String>().trim().to_owned()));

            let remote_string = "Remote".to_string();
            let location = if !onsite.is_empty() && tags.contains(&remote_string) {
                format!("{}, {}", onsite, remote_string)
            } else if tags.contains(&remote_string) {
                remote_string
            } else {
                onsite
            };

            self.jobs.push(Job {
                title,
                company,
                date_posted,
                location,
                remuneration,
                tags,
                apply,
                site: self.get_url(),
            });
        }

        self.jobs = self.jobs.into_iter().unique().collect();

        Ok(self)
    }
}

/// Provides a common scrape implementation for a number of web3/blockchain job sites built with the
/// same HTML structure.
trait Common {
    type Input: Site + Scraper;

    /// Returns a selector from the Input type's `get_selector` method.
    fn _get_selector(selectors: &str) -> Result<Selector, Error>;

    /// A common scrape implementation for a number of web3/blockchain job sites.
    fn _scrape(input: &Self::Input) -> Result<Vec<Job>, Error> {
        let mut jobs = vec![];
        let response =
            reqwest::blocking::get(input.get_url()).map_err(|err| Error::Request(Box::new(err)))?;
        if !response.status().is_success() {
            Err(Error::Response(response.status().as_u16()))?;
        }
        let body = response
            .text()
            .map_err(|err| Error::Parser(Box::new(err)))?;
        let document = Html::parse_document(&body);

        // HTML selectors
        let div1_selector = Self::_get_selector("div.infinite-scroll-component__outerdiv>div>div")?;
        let div2_selector = Self::_get_selector(r#"div[itemprop=title]"#)?;
        let meta1_selector = Self::_get_selector(r#"meta[itemprop=name]"#)?;
        let span_selector = Self::_get_selector("span")?;
        let meta2_selector = Self::_get_selector(r#"meta[itemprop=datePosted]"#)?;
        let a_selector = Self::_get_selector(r#"a[data-testid=read-more]"#)?;

        for el in document.select(&div1_selector) {
            let mut div2_selector = el.select(&div2_selector);

            if let Some(element) = div2_selector.next() {
                let title = element.text().collect::<String>().trim().to_owned();

                let mut meta1_element = el.select(&meta1_selector);
                let company_element = meta1_element.next().ok_or(Error::Iterator("company"))?;
                let company = company_element
                    .value()
                    .attr("content")
                    .unwrap_or("")
                    .to_owned();

                let mut span_element = el.select(&span_selector);
                let remuneration = "".to_string();
                let mut location = "".to_string();
                if let Some(element) = span_element.next() {
                    location = element.text().collect::<String>().trim().to_owned();
                    if let Some(element) = span_element.next() {
                        location = format!(
                            "{}, {}",
                            location,
                            element.text().collect::<String>().trim()
                        );
                    }
                }

                let mut meta2_element = el.select(&meta2_selector);
                let date_posted_element =
                    meta2_element.next().ok_or(Error::Iterator("date posted"))?;
                let date_posted = date_posted_element
                    .value()
                    .attr("content")
                    .unwrap_or("")
                    .to_owned();

                let mut a_element = el.select(&a_selector);
                let apply_element = a_element.next().ok_or(Error::Iterator("apply link"))?;
                let mut apply = apply_element.value().attr("href").unwrap_or("").to_owned();
                apply = if apply.starts_with("https") {
                    apply
                } else {
                    "".into()
                };

                jobs.push(Job {
                    title,
                    company,
                    date_posted,
                    location,
                    remuneration,
                    tags: Vec::new(),
                    apply,
                    site: input.get_url(),
                });
            }
        }

        Ok(jobs.into_iter().unique().collect())
    }
}

impl Common for SolanaJobs {
    type Input = SolanaJobs;

    fn _get_selector(selectors: &str) -> Result<Selector, Error> {
        Self::Input::get_selector(selectors)
    }
}

impl Scraper for SolanaJobs {
    fn scrape(mut self) -> Result<Self, Error> {
        self.jobs = Self::_scrape(&self)?;
        Ok(self)
    }
}

impl Common for SubstrateJobs {
    type Input = SubstrateJobs;

    fn _get_selector(selectors: &str) -> Result<Selector, Error> {
        Self::Input::get_selector(selectors)
    }
}

impl Scraper for SubstrateJobs {
    fn scrape(mut self) -> Result<Self, Error> {
        self.jobs = Self::_scrape(&self)?;
        Ok(self)
    }
}

impl Common for NearJobs {
    type Input = NearJobs;

    fn _get_selector(selectors: &str) -> Result<Selector, Error> {
        Self::Input::get_selector(selectors)
    }
}

impl Scraper for NearJobs {
    fn scrape(mut self) -> Result<Self, Error> {
        self.jobs = Self::_scrape(&self)?;
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    use crate::repository::Job;
    use crate::site::{
        CryptoJobsList, NearJobs, Site, SolanaJobs, SubstrateJobs, UseWeb3, Web3Careers,
        CRYPTO_JOBS_LIST_URL, NEAR_JOBS_URL, SOLANA_JOBS_URL, SUBSTRATE_JOBS_URL, USE_WEB3_URL,
        WEB3_CAREERS_URL,
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
        jobs.iter().for_each(|job| {
            assert!(!job.title.is_empty());
            assert!(!job.company.is_empty());
            assert!(Regex::new(DATE_REGEX).unwrap().is_match(&job.date_posted));
            assert!(
                job.remuneration.to_lowercase().contains("k")
                    && job.remuneration.to_lowercase().contains("$")
                    || job.remuneration.is_empty()
            );
            assert!(
                job.apply.starts_with("https")
                    || job.apply.starts_with("mailto")
                    || job.apply.is_empty()
            )
        })
    }
}
