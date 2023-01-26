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

trait Type {
    fn into_map(self, job: &Job, mut map: HashMap<Self, Vec<Job>>) -> HashMap<Self, Vec<Job>>
        where Self: Sized + Eq + Hash
    {
        map
            .entry(self)
            .and_modify(|job_vec| job_vec.push(job.clone()))
            .or_insert(vec![job.clone()]);
        map
    }
}

#[derive(Debug, Eq, Hash, Clone, PartialEq)]
pub enum Skill {
    Backend,
    Frontend,
    Fullstack,
}

impl Type for Skill {}

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

impl Type for Level {}

#[derive(Debug, Eq, Hash, Clone, PartialEq)]
pub enum Location {
    Remote,
    Onsite,
}

impl Type for Location {}

#[derive(Debug, Clone)]
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
            if job.location_contains("remote") {
                self.location = Location::Remote.into_map(job, self.clone().location);
            } else {
                self.location = Location::Onsite.into_map(job, self.clone().location);
            }

            // index by skill
            if job.title_contains("backend") { self.skill = Skill::Backend.into_map(job, self.clone().skill); }
            if job.title_contains("frontend") { self.skill = Skill::Frontend.into_map(job, self.clone().skill); }
            if job.title_contains("fullstack") { self.skill = Skill::Fullstack.into_map(job, self.clone().skill); }

            // index by level
            if job.title_contains("junior") { self.level = Level::Junior.into_map(job, self.clone().level); }
            if job.title_contains("intermediate") { self.level = Level::Intermediate.into_map(job, self.clone().level); }
            if job.title_contains("senior") || job.title_contains("snr") || job.title_contains("sr") {
                self.level = Level::Senior.into_map(job, self.clone().level);
            }
            if job.title_contains("staff") { self.level = Level::Staff.into_map(job, self.clone().level); }
            if job.title_contains("lead") { self.level = Level::Lead.into_map(job, self.clone().level); }
            if job.title_contains("principle") { self.level = Level::Principle.into_map(job, self.clone().level); }
            if job.title_contains("manager") { self.level = Level::Manager.into_map(job, self.clone().level); }
        }
    }
}
