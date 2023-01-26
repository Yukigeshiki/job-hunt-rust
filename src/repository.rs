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

pub trait JobRepository {
    fn default() -> Self;

    fn import(&mut self, jobs: Vec<&mut Vec<Job>>) -> &mut Self;

    fn filter(&mut self, on: fn(job: &Job) -> bool) -> &mut Self;

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

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Location {
    Remote,
    Onsite,
}

#[derive(Debug)]
pub struct SoftwareJobs {
    pub all: Vec<Job>,
    // attrs
    pub date: HashMap<String, Vec<Job>>,
    pub company: HashMap<String, Vec<Job>>,
    pub location: HashMap<Location, Vec<Job>>,
    // types
    pub skill: HashMap<Skill, Vec<Job>>,
    pub level: HashMap<Level, Vec<Job>>,
}

impl JobRepository for SoftwareJobs {
    fn default() -> Self {
        Self {
            all: vec![],
            date: HashMap::new(),
            company: HashMap::new(),
            location: HashMap::new(),
            skill: HashMap::new(),
            level: HashMap::new(),
        }
    }

    fn import(&mut self, jobs: Vec<&mut Vec<Job>>) -> &mut Self {
        // allow duplicate job posts if they are from different sites - user can choose which site
        // to apply from
        for job_vec in jobs { self.all.append(job_vec) }
        self
    }

    fn filter(&mut self, on: fn(job: &Job) -> bool) -> &mut Self {
        self.all = self.all
            .clone()
            .into_iter()
            .filter(|job| on(job))
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

            // index by location
            if job.location.to_lowercase().contains("remote") {
                self.location
                    .entry(Location::Remote)
                    .and_modify(|job_vec| job_vec.push(job.clone()))
                    .or_insert(vec![job.clone()]);
            } else {
                self.location
                    .entry(Location::Onsite)
                    .and_modify(|job_vec| job_vec.push(job.clone()))
                    .or_insert(vec![job.clone()]);
            }

            // index by skill
            if job.title.to_lowercase().contains("backend") {
                self.skill
                    .entry(Skill::Backend)
                    .and_modify(|job_vec| job_vec.push(job.clone()))
                    .or_insert(vec![job.clone()]);
            }
            if job.title.to_lowercase().contains("frontend") {
                self.skill
                    .entry(Skill::Frontend)
                    .and_modify(|job_vec| job_vec.push(job.clone()))
                    .or_insert(vec![job.clone()]);
            }
            if job.title.to_lowercase().contains("fullstack") {
                self.skill
                    .entry(Skill::Fullstack)
                    .and_modify(|job_vec| job_vec.push(job.clone()))
                    .or_insert(vec![job.clone()]);
            }

            // index by level
            if job.title.to_lowercase().contains("junior") {
                self.level
                    .entry(Level::Junior)
                    .and_modify(|job_vec| job_vec.push(job.clone()))
                    .or_insert(vec![job.clone()]);
            }
            if job.title.to_lowercase().contains("intermediate") {
                self.level
                    .entry(Level::Intermediate)
                    .and_modify(|job_vec| job_vec.push(job.clone()))
                    .or_insert(vec![job.clone()]);
            }
            if job.title.to_lowercase().contains("senior") ||
                job.title.to_lowercase().contains("snr") ||
                job.title.to_lowercase().contains("sr") {
                self.level
                    .entry(Level::Senior)
                    .and_modify(|job_vec| job_vec.push(job.clone()))
                    .or_insert(vec![job.clone()]);
            }
            if job.title.to_lowercase().contains("staff") {
                self.level
                    .entry(Level::Staff)
                    .and_modify(|job_vec| job_vec.push(job.clone()))
                    .or_insert(vec![job.clone()]);
            }
            if job.title.to_lowercase().contains("lead") {
                self.level
                    .entry(Level::Lead)
                    .and_modify(|job_vec| job_vec.push(job.clone()))
                    .or_insert(vec![job.clone()]);
            }
            if job.title.to_lowercase().contains("principle") {
                self.level
                    .entry(Level::Principle)
                    .and_modify(|job_vec| job_vec.push(job.clone()))
                    .or_insert(vec![job.clone()]);
            }
            if job.title.to_lowercase().contains("manager") {
                self.level
                    .entry(Level::Manager)
                    .and_modify(|job_vec| job_vec.push(job.clone()))
                    .or_insert(vec![job.clone()]);
            }
        }
    }
}
