//! Websites often change, so the scrapers should be tested often and updated when needed.

use regex::Regex;
use scraper::Html;
use scraper::Selector;
use crate::repository::Job;
use crate::site::{CryptoJobsList, Site, UseWeb3, Web3Careers};

/// All website structs must implement the Scraper trait.
pub trait Scraper {
    /// Scrapes the job website and adds Job instances to the site's jobs array - Job instances must
    /// conform to the structure defined by crate::repository::Job.
    fn scrape(self) -> Result<Self, String> where Self: Sized;

    /// A default method. Gets a selector for a specific HTML element.
    fn get_selector(selectors: &str) -> Result<Selector, String> {
        Selector::parse(selectors).map_err(|err| format!("selector error: {}", err.to_string()))
    }
}

impl Scraper for Web3Careers {
    fn scrape(mut self) -> Result<Self, String> {
        let response = reqwest::blocking::get(self.get_url())
            .map_err(|err| format!("could not load url: {}", err.to_string()))?;
        if !response.status().is_success() {
            Err(format!("request failed with code: {}", response.status().to_string()))?;
        }
        let body = response
            .text()
            .map_err(|err| format!("error getting response body: {}", err.to_string()))?;
        let document = Html::parse_document(&body);

        // HTML selectors
        let table_row_selector = Self::get_selector("tr.table_row")?;
        let td_selector = Self::get_selector("td")?;
        let time_selector = Self::get_selector("time")?;
        let a_selector = Self::get_selector("a")?;

        // iterate through document and populate jobs array
        for el in document.select(&table_row_selector) {
            let mut element_iterator = el.select(&td_selector);

            let title_element = element_iterator.next().ok_or("could not get job title")?;
            let title = title_element.text().collect::<String>().trim().to_string();

            let company_element = element_iterator.next().ok_or("could not get company")?;
            let company = company_element.text().collect::<String>().trim().to_string();

            let date_posted_element = element_iterator.next().ok_or("could not get time")?;
            let date_posted_element_select =
                date_posted_element.select(&time_selector).next().ok_or("could not get time")?;
            let date_posted =
                date_posted_element_select.value().attr("datetime").unwrap_or("").to_string()
                    .split(" ").next().unwrap().to_string();

            let location_element = element_iterator.next().ok_or("could not get location")?;
            let location =
                location_element.text().collect::<String>().trim().to_string().replace("\n", " ");

            let remuneration_element = element_iterator.next().ok_or("could not get remuneration")?;
            let remuneration = remuneration_element.text().collect::<String>().trim().to_string();

            let mut tags = Vec::new();
            let tag_element = element_iterator.next().ok_or("could not get tags")?;
            tag_element
                .select(&a_selector)
                .for_each(|tag| tags.push(tag.text().collect::<String>().trim().to_string()));

            self.jobs.push(
                Job { title, company, date_posted, location, remuneration, tags, site: self.get_url() }
            );
        };

        Ok(self)
    }
}

impl Scraper for UseWeb3 {
    fn scrape(mut self) -> Result<Self, String> {
        let response = reqwest::blocking::get(self.get_url())
            .map_err(|err| format!("could not load url: {}", err.to_string()))?;
        if !response.status().is_success() {
            Err(format!("request failed with code: {}", response.status().to_string()))?;
        }
        let body = response
            .text()
            .map_err(|err| format!("error getting response body: {}", err.to_string()))?;
        let document = Html::parse_document(&body);

        // HTML selectors
        let panel_inner_selector = Self::get_selector("div.panel_inner__YQLRW")?;
        let a_selector = Self::get_selector("a")?;
        let span_selector = Self::get_selector("span")?;
        let panel_border_selector = Self::get_selector("div.panel_border___58nj")?;

        for el in document.select(&panel_inner_selector) {
            let mut element_iterator = el.select(&a_selector);

            let title_element = element_iterator.next().ok_or("could not get job title")?;
            let title = title_element.text().collect::<String>().trim().to_string();

            let company_element = element_iterator.next().ok_or("could not get company")?;
            let company = company_element.text().collect::<String>().trim().to_string();

            let mut element_iterator = el.select(&span_selector);

            let location_element = element_iterator.next().ok_or("could not get location")?;
            let mut location = location_element.text().collect::<String>().trim().to_string();

            let time_elapsed_element = element_iterator.next().ok_or("could not get elapsed time")?;
            let time_elapsed = time_elapsed_element.text().collect::<String>().trim().to_string();
            let date_posted = Self::get_date_from(time_elapsed);

            let mut remuneration = "".to_string();
            el
                .select(&panel_border_selector)
                .for_each(|item| {
                    let i = item.text().collect::<String>().trim().to_string();
                    if i.contains("🌐") && !location.to_lowercase().contains("remote") {
                        location = format!("{}, {}", location, i.replace("🌐 ", ""));
                    }
                    if i.contains("💰") {
                        remuneration = i.replace("💰 ", "");
                    }
                });

            self.jobs.push(
                Job { title, company, date_posted, location, remuneration, tags: Vec::new(), site: self.get_url() }
            );
        }

        Ok(self)
    }
}

impl Scraper for CryptoJobsList {
    fn scrape(mut self) -> Result<Self, String> where Self: Sized {
        let response = reqwest::blocking::get(self.get_url())
            .map_err(|err| format!("could not load url: {}", err.to_string()))?;
        if !response.status().is_success() {
            Err(format!("request failed with code: {}", response.status().to_string()))?;
        }
        let body = response
            .text()
            .map_err(|err| format!("error getting response body: {}", err.to_string()))?;
        let document = Html::parse_document(&body);

        // HTML selectors
        let li_selector = Self::get_selector("section ul li")?;
        let a_selector = Self::get_selector("a")?;
        let span_selector = Self::get_selector("span span span")?;
        let span_a_selector = Self::get_selector("span span a")?;
        let span_class_selector = Self::get_selector("span.JobPreviewInline_createdAt__wbWS0")?;

        for el in document.select(&li_selector) {
            let mut a_element = el.select(&a_selector);

            let title_element = a_element.next().ok_or("could not get job title")?;
            let title = title_element.text().collect::<String>().trim().to_string();

            let company_element = a_element.next().ok_or("could not get company")?;
            let company = company_element.text().collect::<String>().trim().to_string();

            let mut span_class_element = el.select(&span_class_selector);
            let time_elapsed_element = span_class_element.next().ok_or("could not get time elapsed")?;
            let time_elapsed = time_elapsed_element.text().collect::<String>().trim().to_string();
            let date_posted = Self::get_date_from(time_elapsed);

            let mut span_element = el.select(&span_selector);
            let onsite_location_element = span_element.next().ok_or("could not get onsite location")?;
            let mut onsite_location = onsite_location_element.text().collect::<String>().trim().to_string();
            if Regex::new(r"[0-9]").unwrap().is_match(&onsite_location) {
                onsite_location = "".to_string();
            }

            let mut tags = Vec::new();
            el
                .select(&span_a_selector)
                .for_each(|tag| tags.push(tag.text().collect::<String>().trim().to_string()));

            let location_element = a_element.next().ok_or("could not get location")?;
            let mut location = location_element.text().collect::<String>().trim().to_string();
            if !onsite_location.is_empty() {
                if tags.contains(&"Remote".to_string()) {
                    location = format!("{}, {}", onsite_location, location)
                } else {
                    location = onsite_location;
                }
            }

            self.jobs.push(
                Job { title, company, date_posted, location, remuneration: "".to_string(), tags, site: self.get_url() }
            );
        }

        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use regex::Regex;
    use super::Scraper;
    use crate::site::{
        CRYPTO_JOBS_LIST, CryptoJobsList, Site, USE_WEB3_URL, UseWeb3, WEB3_CAREERS_URL, Web3Careers,
    };

    const DATE_REGEX: &str = r"(\d{4})-(\d{2})-(\d{2})( (\d{2}):(\d{2}):(\d{2}))?";

    #[test]
    fn test_scrape_web3careers() {
        let jobs = Web3Careers::new().scrape().unwrap().jobs;
        assert!(jobs.len() > 0);
        assert_eq!(jobs[0].site, WEB3_CAREERS_URL);
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
            });
    }

    #[test]
    fn test_scrape_use_web3() {
        let jobs = UseWeb3::new().scrape().unwrap().jobs;
        assert!(jobs.len() > 0);
        assert_eq!(jobs[0].site, USE_WEB3_URL);
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
            })
    }

    #[test]
    fn test_scrape_crypto_jobs_list() {
        let jobs = CryptoJobsList::new().scrape().unwrap().jobs;
        assert!(jobs.len() > 0);
        assert_eq!(jobs[0].site, CRYPTO_JOBS_LIST);
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
            })
    }
}
