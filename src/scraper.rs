use scraper::Html;
use scraper::Selector;
use crate::repository::Job;
use crate::site::{CryptocurrencyJobs, WEB3_JOBS_URL, Web3Jobs};

const SELECTOR_ERROR: &str = "selector error";

pub trait Scraper {
    fn scrape(&mut self) -> Result<&mut Self, String>;

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

            let job_title_element = element_iterator.next().ok_or("could not select job title")?;
            let job_title = job_title_element.text().collect::<String>().trim().to_string();

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
                Job { job_title, company, date_posted, location, remuneration, tags, job_site: WEB3_JOBS_URL }
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
