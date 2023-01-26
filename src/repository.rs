use std::fmt::{Debug, Formatter};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Jobs {
    pub all: Vec<Job>,
    pub date: HashMap<String, Vec<Job>>,
    pub company: HashMap<String, Vec<Job>>,
    pub job_type: HashMap<String, Vec<Job>>,
}

impl Jobs {
    pub fn default() -> Self {
        Jobs {
            all: vec![],
            date: HashMap::new(),
            company: HashMap::new(),
            job_type: HashMap::new(),
        }
    }

    pub fn aggregate_to_all(&mut self, jobs: Vec<&mut Vec<Job>>) -> &mut Self {
        self.all = Self::aggregate(jobs);
        self
    }

    pub fn aggregate(jobs: Vec<&mut Vec<Job>>) -> Vec<Job> {
        let mut all = vec![];
        for job_vec in jobs { all.append(job_vec) }
        all.sort_by(|a, b| a.job_title.cmp(&b.job_title));
        // when used to aggregate across sites for duplicate job posts, only the first site in the
        // sorted vector will be kept, the rest will be removed
        all.dedup_by(
            |a, b| a.job_title.eq_ignore_ascii_case(&b.job_title) &&
                a.company.eq_ignore_ascii_case(&b.company)
        );
        all
    }

    pub fn filter_software_jobs(&mut self) -> &mut Self {
        self.all = self.all
            .clone()
            .into_iter()
            .filter(
                |job| job.job_title.to_lowercase().contains("developer") ||
                    job.job_title.to_lowercase().contains("engineer")
            )
            .collect();
        self
    }

    pub fn index(&mut self) {
        for job in &self.all {

            // index by attribute
            self.date
                .entry(job.date_posted.clone())
                .and_modify(|job_vec| job_vec.push(job.clone()))
                .or_insert(vec![job.clone()]);
            self.company
                .entry(job.company.clone())
                .and_modify(|job_vec| job_vec.push(job.clone()))
                .or_insert(vec![job.clone()]);

            // index by job type
            let backend = self.all
                .clone()
                .into_iter()
                .filter(|job| job.job_title.to_lowercase().contains("backend"))
                .collect();
            self.job_type.insert("backend".to_string(), backend);
            let frontend = self.all
                .clone()
                .into_iter()
                .filter(|job| job.job_title.to_lowercase().contains("frontend"))
                .collect();
            self.job_type.insert("frontend".to_string(), frontend);
            let fullstack = self.all
                .clone()
                .into_iter()
                .filter(|job| job.job_title.to_lowercase().contains("fullstack"))
                .collect();
            self.job_type.insert("fullstack".to_string(), fullstack);
        }
    }
}

#[derive(Clone)]
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
            "Position: {}, Company: {}, Date Posted: {}, Location: {}, Remuneration: {}, Tags: {:?}, Job Site: {}",
            self.job_title, self.company, self.date_posted, self.location, self.remuneration, self.tags, self.job_site
        )
    }
}
