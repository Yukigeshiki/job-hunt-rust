use scraper::Html;
use scraper::Selector;
use crate::repository::Job;
use crate::site::{CryptocurrencyJobs, Web3Jobs};

const SELECTOR_ERROR: &str = "selector error";

/// All website structs must implement the Scraper trait.
pub trait Scraper {
    /// Scrapes the job website and adds Job instances to the site's jobs array - Job instances must
    /// conform to the structure, jobhunt::repository::Job.
    fn scrape(&mut self) -> Result<&mut Self, String>;

    /// A default method. Gets a selector for a specific HTML element.
    fn get_selector(selectors: &str) -> Result<Selector, String> {
        Selector::parse(selectors).map_err(|err| format!("{}: {}", SELECTOR_ERROR, err.to_string()))
    }
}

impl Scraper for Web3Jobs {
    fn scrape(&mut self) -> Result<&mut Self, String> {
        let response =
            reqwest::blocking::get(self.url).map_err(|err| format!("could not load url: {}", err.to_string()))?;
        assert!(response.status().is_success());
        let body =
            response.text().map_err(|err| format!("error getting response body: {}", err.to_string()))?;
        let document = Html::parse_document(&body);

        // HTML selectors
        let table_selector = Self::get_selector("tr.table_row")?;
        let element_selector = Self::get_selector("td")?;
        let date_posted_selector = Self::get_selector("time")?;
        let tag_selector = Self::get_selector("a")?;

        // iterate through document and populate jobs array
        for element in document.select(&table_selector) {
            let mut element_iterator = element.select(&element_selector);

            let title_element = element_iterator.next().ok_or("could not select job title")?;
            let title = title_element.text().collect::<String>().trim().to_string();

            let company_element = element_iterator.next().ok_or("could not select company")?;
            let company = company_element.text().collect::<String>().trim().to_string();

            let date_posted_element = element_iterator.next().ok_or("could not select time")?;
            let date_posted_element_select =
                date_posted_element.select(&date_posted_selector).next().ok_or("could not select time")?;
            let date_posted =
                date_posted_element_select.value().attr("datetime").unwrap_or("").to_string()
                    .split(" ").next().unwrap().to_string();

            let location_element = element_iterator.next().ok_or("could not select location")?;
            let location =
                location_element.text().collect::<String>().trim().to_string().replace("\n", " ");

            let remuneration_element = element_iterator.next().ok_or("could not select remuneration")?;
            let remuneration = remuneration_element.text().collect::<String>().trim().to_string();

            let mut tags = vec![];
            let tag_element = element_iterator.next().ok_or("could not select tags")?;
            for tag in tag_element.select(&tag_selector) {
                tags.push(tag.text().collect::<String>().trim().to_string());
            }

            self.jobs.push(
                Job { title, company, date_posted, location, remuneration, tags, site: self.url }
            );
        };

        Ok(self)
    }
}

impl Scraper for CryptocurrencyJobs {
    fn scrape(&mut self) -> Result<&mut Self, String> {
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use regex::Regex;
    use crate::scraper::Scraper;
    use crate::site::{Site, WEB3_JOBS_URL, Web3Jobs};

    #[test]
    fn test_scrape_web3jobs() {
        let mut scraper = Web3Jobs::new();
        let jobs = &scraper.scrape().unwrap().jobs;
        assert!(jobs.len() > 0);
        assert!(!jobs[0].title.is_empty());
        assert!(!jobs[0].company.is_empty());
        assert!(
            Regex::new(r"(\d{4})-(\d{2})-(\d{2})( (\d{2}):(\d{2}):(\d{2}))?")
                .unwrap()
                .is_match(&jobs[0].date_posted)
        );
        assert_eq!(jobs[0].site, WEB3_JOBS_URL);
    }
}
