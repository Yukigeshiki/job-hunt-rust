use std::fmt::{Debug, Formatter};
use std::collections::HashMap;
use std::hash::Hash;

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

impl Job {
    fn location_contains(&self, val: &str) -> bool { self.location.to_lowercase().contains(val) }

    fn title_contains(&self, val: &str) -> bool { self.title.to_lowercase().contains(val) }

    fn index_with_type<T>(&self, mut map: HashMap<T, Vec<Job>>, t: T) -> HashMap<T, Vec<Job>>
        where T: Sized + Eq + Hash
    {
        map
            .entry(t)
            .and_modify(|job_vec| job_vec.push(self.clone()))
            .or_insert(vec![self.clone()]);
        map
    }
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

#[derive(Debug, Eq, Hash, Clone, PartialEq)]
pub enum Skill {
    Backend,
    Frontend,
    Fullstack,
}

#[derive(Debug, Eq, Hash, Clone, PartialEq)]
pub enum Level {
    Junior,
    Intermediate,
    Senior,
    Staff,
    Lead,
    Principle,
    Manager,
}

#[derive(Debug, Eq, Hash, Clone, PartialEq)]
pub enum Location {
    Remote,
    Onsite,
}

#[derive(Debug, Clone)]
pub struct SoftwareJobs {
    pub all: Vec<Job>,
    // attrs
    pub date: HashMap<String, Vec<Job>>,
    pub company: HashMap<String, Vec<Job>>,
    // types
    pub location: HashMap<Location, Vec<Job>>,
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
            if job.location_contains("remote") {
                self.location = job.index_with_type(self.clone().location, Location::Remote);
            } else {
                self.location = job.index_with_type(self.clone().location, Location::Onsite);
            }

            // index by skill
            if job.title_contains("backend") {
                self.skill = job.index_with_type(self.clone().skill, Skill::Backend);
            }
            if job.title_contains("frontend") {
                self.skill = job.index_with_type(self.clone().skill, Skill::Frontend);
            }
            if job.title_contains("fullstack") {
                self.skill = job.index_with_type(self.clone().skill, Skill::Fullstack);
            }

            // index by level
            if job.title_contains("junior") {
                self.level = job.index_with_type(self.clone().level, Level::Junior);
            }
            if job.title_contains("intermediate") {
                self.level = job.index_with_type(self.clone().level, Level::Intermediate);
            }
            if job.title_contains("senior") || job.title_contains("snr") || job.title_contains("sr") {
                self.level = job.index_with_type(self.clone().level, Level::Senior);
            }
            if job.title_contains("staff") {
                self.level = job.index_with_type(self.clone().level, Level::Staff);
            }
            if job.title_contains("lead") {
                self.level = job.index_with_type(self.clone().level, Level::Lead);
            }
            if job.title_contains("principle") {
                self.level = job.index_with_type(self.clone().level, Level::Principle);
            }
            if job.title_contains("manager") {
                self.level = job.index_with_type(self.clone().level, Level::Manager);
            }
        }
    }
}
