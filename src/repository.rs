use std::fmt::{Debug, Formatter};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Job {
    pub title: String,
    pub company: String,
    pub date_posted: String,
    pub location: String,
    pub remuneration: String,
    pub tags: Vec<String>,
    pub site: &'static str,
}

impl Debug for Job {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Position: {}, Company: {}, Date Posted: {}, Location: {}, Remuneration: {}, Tags: {:?}, Job Site: {}",
            self.title, self.company, self.date_posted, self.location, self.remuneration, self.tags, self.site
        )
    }
}

pub trait Repository {
    fn default() -> Self;

    fn import_into_repository(&mut self, jobs: Vec<&mut Vec<Job>>) -> &mut Self;

    fn filter_job_type(&mut self) -> &mut Self;

    fn index(&mut self);
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Skill {
    Backend,
    Frontend,
    Fullstack,
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Level {
    Junior,
    Intermediate,
    Senior,
    Staff,
    Lead,
    Principle,
    Manager,
}

#[derive(Debug)]
pub struct SoftwareJobs {
    pub all: Vec<Job>,
    // attrs
    pub date: HashMap<String, Vec<Job>>,
    pub company: HashMap<String, Vec<Job>>,
    // types
    pub skill: HashMap<Skill, Vec<Job>>,
    pub level: HashMap<Level, Vec<Job>>,
}

impl Repository for SoftwareJobs {
    fn default() -> Self {
        Self {
            all: vec![],
            date: HashMap::new(),
            company: HashMap::new(),
            skill: HashMap::new(),
            level: HashMap::new(),
        }
    }

    fn import_into_repository(&mut self, jobs: Vec<&mut Vec<Job>>) -> &mut Self {
        // allow duplicate job posts if they are from different sites - can choose which site
        // to apply on
        for job_vec in jobs { self.all.append(job_vec) }
        self
    }

    fn filter_job_type(&mut self) -> &mut Self {
        self.all = self.all
            .clone()
            .into_iter()
            .filter(
                |job| job.title.to_lowercase().contains("developer") ||
                    job.title.to_lowercase().contains("engineer") ||
                    job.title.to_lowercase().contains("engineering")
            )
            .collect();
        self
    }

    fn index(&mut self) {
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

            // index by type

            // index by skill
            let backend =
                self.all
                    .clone()
                    .into_iter()
                    .filter(|job| job.title.to_lowercase().contains("backend"))
                    .collect();
            self.skill.insert(Skill::Backend, backend);
            let frontend =
                self.all
                    .clone()
                    .into_iter()
                    .filter(|job| job.title.to_lowercase().contains("frontend"))
                    .collect();
            self.skill.insert(Skill::Frontend, frontend);
            let fullstack =
                self.all
                    .clone()
                    .into_iter()
                    .filter(|job| job.title.to_lowercase().contains("fullstack"))
                    .collect();
            self.skill.insert(Skill::Fullstack, fullstack);

            // index by level
            let junior =
                self.all
                    .clone()
                    .into_iter()
                    .filter(|job| job.title.to_lowercase().contains("junior"))
                    .collect();
            self.level.insert(Level::Junior, junior);
            let intermediate =
                self.all
                    .clone()
                    .into_iter()
                    .filter(|job| job.title.to_lowercase().contains("intermediate"))
                    .collect();
            self.level.insert(Level::Intermediate, intermediate);
            let senior =
                self.all
                    .clone()
                    .into_iter()
                    .filter(
                        |job|
                            job.title.to_lowercase().contains("senior") ||
                                job.title.to_lowercase().contains("snr") ||
                                job.title.to_lowercase().contains("sr")
                    )
                    .collect();
            self.level.insert(Level::Senior, senior);
            let staff =
                self.all
                    .clone()
                    .into_iter()
                    .filter(|job| job.title.to_lowercase().contains("staff"))
                    .collect();
            self.level.insert(Level::Staff, staff);
            let lead =
                self.all
                    .clone()
                    .into_iter()
                    .filter(|job| job.title.to_lowercase().contains("lead"))
                    .collect();
            self.level.insert(Level::Lead, lead);
            let principle =
                self.all
                    .clone()
                    .into_iter()
                    .filter(|job| job.title.to_lowercase().contains("principle"))
                    .collect();
            self.level.insert(Level::Principle, principle);
            let manager =
                self.all
                    .clone()
                    .into_iter()
                    .filter(|job| job.title.to_lowercase().contains("manager"))
                    .collect();
            self.level.insert(Level::Manager, manager);
        }
    }
}
