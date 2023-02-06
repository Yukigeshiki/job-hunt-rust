use scraper::Html;
use scraper::Selector;
use crate::repository::Job;
use crate::site::{UseWeb3, Web3Jobs};

const SELECTOR_ERROR: &str = "selector error";

/// All website structs must implement the Scraper trait.
pub trait Scraper {
    /// Scrapes the job website and adds Job instances to the site's jobs array - Job instances must
    /// conform to the structure defined by crate::repository::Job.
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
        if !response.status().is_success() {
            Err(format!("request failed with code: {}", response.status().to_string()))?;
        }
        let body =
            response.text().map_err(|err| format!("error getting response body: {}", err.to_string()))?;
        let document = Html::parse_document(&body);

        // HTML selectors
        let table_selector = Self::get_selector("tr.table_row")?;
        let td_selector = Self::get_selector("td")?;
        let time_selector = Self::get_selector("time")?;
        let a_selector = Self::get_selector("a")?;

        // iterate through document and populate jobs array
        for element in document.select(&table_selector) {
            let mut element_iterator = element.select(&td_selector);

            let title_element = element_iterator.next().ok_or("could not select job title")?;
            let title = title_element.text().collect::<String>().trim().to_string();

            let company_element = element_iterator.next().ok_or("could not select company")?;
            let company = company_element.text().collect::<String>().trim().to_string();

            let date_posted_element = element_iterator.next().ok_or("could not select time")?;
            let date_posted_element_select =
                date_posted_element.select(&time_selector).next().ok_or("could not select time")?;
            let date_posted =
                date_posted_element_select.value().attr("datetime").unwrap_or("").to_string()
                    .split(" ").next().unwrap().to_string();

            let location_element = element_iterator.next().ok_or("could not select location")?;
            let location =
                location_element.text().collect::<String>().trim().to_string().replace("\n", " ");

            let remuneration_element = element_iterator.next().ok_or("could not select remuneration")?;
            let remuneration = remuneration_element.text().collect::<String>().trim().to_string();

            let mut tags = Vec::new();
            let tag_element = element_iterator.next().ok_or("could not select tags")?;
            for tag in tag_element.select(&a_selector) {
                tags.push(tag.text().collect::<String>().trim().to_string());
            }

            self.jobs.push(
                Job { title, company, date_posted, location, remuneration, tags, site: self.url }
            );
        };

        Ok(self)
    }
}

impl Scraper for UseWeb3 {
    fn scrape(&mut self) -> Result<&mut Self, String> {
        let response =
            reqwest::blocking::get(self.url).map_err(|err| format!("could not load url: {}", err.to_string()))?;
        if !response.status().is_success() {
            Err(format!("request failed with code: {}", response.status().to_string()))?;
        }
        let body =
            response.text().map_err(|err| format!("error getting response body: {}", err.to_string()))?;
        let document = Html::parse_document(&body);

        // HTML selectors
        let table_selector = Self::get_selector("div.panel_inner__YQLRW")?;
        let a_selector = Self::get_selector("a")?;
        let span_selector = Self::get_selector("span")?;
        let div_selector = Self::get_selector("div.panel_border___58nj")?;


        // let items = document.select(&table_selector).next().unwrap();
        for element in document.select(&table_selector) {
            let mut element_iterator = element.select(&a_selector);

            let title_element = element_iterator.next().ok_or("could not get job title")?;
            let title = title_element.text().collect::<String>().trim().to_string();

            let company_element = element_iterator.next().ok_or("could not get company")?;
            let company = company_element.text().collect::<String>().trim().to_string();

            let mut element_iterator = element.select(&span_selector);

            let location_element = element_iterator.next().ok_or("could not get location")?;
            let mut location = location_element.text().collect::<String>().trim().to_string();

            let time_elapsed_element = element_iterator.next().ok_or("could not get elapsed time")?;
            let time_elapsed = time_elapsed_element.text().collect::<String>().trim().to_string();
            let date_posted = Self::get_date(time_elapsed);

            let mut remuneration = "".to_string();
            for item in element.select(&div_selector) {
                let i = item.text().collect::<String>().trim().to_string();
                if i.contains("🌐") && !location.to_lowercase().contains("remote") {
                    location = format!("{}, {}", location, i.replace("🌐 ", ""));
                }
                if i.contains("💰") {
                    remuneration = i.replace("💰 ", "");
                }
            }

            self.jobs.push(
                Job { title, company, date_posted, location, remuneration, tags: Vec::new(), site: self.url }
            );
        }


        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use regex::Regex;
    use crate::scraper::Scraper;
    use crate::site::{Site, USE_WEB3_URL, UseWeb3, WEB3_JOBS_URL, Web3Jobs};

    const DATE_REGEX: &str = r"(\d{4})-(\d{2})-(\d{2})( (\d{2}):(\d{2}):(\d{2}))?";

    #[test]
    fn test_scrape_web3jobs() {
        let mut scraper = Web3Jobs::new();
        let jobs = &scraper.scrape().unwrap().jobs;
        assert!(jobs.len() > 0);
        assert!(!jobs[0].title.is_empty());
        assert!(!jobs[0].company.is_empty());
        assert!(
            Regex::new(DATE_REGEX)
                .unwrap()
                .is_match(&jobs[0].date_posted)
        );
        assert_eq!(jobs[0].site, WEB3_JOBS_URL);
    }

    #[test]
    fn test_scrape_use_web3() {
        let mut scraper = UseWeb3::new();
        let jobs = &scraper.scrape().unwrap().jobs;
        assert!(jobs.len() > 0);
        assert!(!jobs[0].title.is_empty());
        assert!(!jobs[0].company.is_empty());
        assert!(
            Regex::new(DATE_REGEX)
                .unwrap()
                .is_match(&jobs[0].date_posted)
        );
        assert_eq!(jobs[0].site, USE_WEB3_URL);
    }
}
