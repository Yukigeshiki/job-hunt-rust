use std::fmt::{Debug, Formatter};

pub struct Job {
    pub job_title: String,
    pub company: String,
    pub date_posted: String,
    pub location: String,
    pub remuneration: String,
    pub tags: Vec<String>,
    pub job_site: &'static str,
}

impl Debug for Job {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\nPosition: {}\nCompany: {}\nDate Posted: {}\nLocation: {}\nRemuneration: {}\nTags: {:?}\nJob Site: {}",
            self.job_title, self.company, self.date_posted, self.location, self.remuneration, self.tags, self.job_site
        )
    }
}

pub trait Indexer {
    fn index();
}
