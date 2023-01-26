use std::fmt::{Debug, Formatter};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Repository {
    pub all: Vec<Job>,
    pub date: HashMap<String, Vec<Job>>,
    pub company: HashMap<String, Vec<Job>>,
    pub skill: HashMap<String, Vec<Job>>,
    pub level: HashMap<String, Vec<Job>>,
}

impl Repository {
    pub fn default() -> Self {
        Self {
            all: vec![],
            date: HashMap::new(),
            company: HashMap::new(),
            skill: HashMap::new(),
            level: HashMap::new(),
        }
    }

    pub fn import_into_repository(&mut self, jobs: Vec<&mut Vec<Job>>) -> &mut Self {
        // allow duplicate job posts if they are from different sites - can choose which site
        // to apply on
        for job_vec in jobs { self.all.append(job_vec) }
        self
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

            // index by skill
            let backend =
                self.all
                    .clone()
                    .into_iter()
                    .filter(|job| job.job_title.to_lowercase().contains("backend"))
                    .collect();
            self.skill.insert("backend".to_string(), backend);
            let frontend =
                self.all
                    .clone()
                    .into_iter()
                    .filter(|job| job.job_title.to_lowercase().contains("frontend"))
                    .collect();
            self.skill.insert("frontend".to_string(), frontend);
            let fullstack =
                self.all
                    .clone()
                    .into_iter()
                    .filter(|job| job.job_title.to_lowercase().contains("fullstack"))
                    .collect();
            self.skill.insert("fullstack".to_string(), fullstack);

            // index by level
            let junior =
                self.all
                    .clone()
                    .into_iter()
                    .filter(|job| job.job_title.to_lowercase().contains("junior"))
                    .collect();
            self.level.insert("junior".to_string(), junior);
            let intermediate =
                self.all
                    .clone()
                    .into_iter()
                    .filter(|job| job.job_title.to_lowercase().contains("intermediate"))
                    .collect();
            self.level.insert("intermediate".to_string(), intermediate);
            let senior =
                self.all
                    .clone()
                    .into_iter()
                    .filter(
                        |job|
                            job.job_title.to_lowercase().contains("senior") ||
                                job.job_title.to_lowercase().contains("snr") ||
                                job.job_title.to_lowercase().contains("sr")
                    )
                    .collect();
            self.level.insert("senior".to_string(), senior);
            let staff =
                self.all
                    .clone()
                    .into_iter()
                    .filter(|job| job.job_title.to_lowercase().contains("staff"))
                    .collect();
            self.level.insert("staff".to_string(), staff);
            let lead =
                self.all
                    .clone()
                    .into_iter()
                    .filter(|job| job.job_title.to_lowercase().contains("lead"))
                    .collect();
            self.level.insert("lead".to_string(), lead);
            let principle =
                self.all
                    .clone()
                    .into_iter()
                    .filter(|job| job.job_title.to_lowercase().contains("principle"))
                    .collect();
            self.level.insert("principle".to_string(), principle);
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
