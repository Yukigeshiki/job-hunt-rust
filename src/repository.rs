use std::fmt::{Debug, Formatter};
use std::collections::HashMap;
use std::hash::Hash;

/// The repository primitive.
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

/// Helper methods for indexing Job instances - these can be customised to fit the relevant job market.
impl Job {
    fn location_contains(&self, val: &str) -> bool { self.location.to_lowercase().contains(val) }

    fn title_contains(&self, val: &str) -> bool { self.title.to_lowercase().contains(val) }

    /// Adds a Job instance to an index map for type T.
    fn index_with_type<T: Sized + Eq + Hash>(&self, map: &mut HashMap<T, Vec<Job>>, t: T) {
        map
            .entry(t)
            .and_modify(|job_vec| job_vec.push(self.clone()))
            .or_insert(vec![self.clone()]);
    }
}

/// Pretty print Job for debug.
impl Debug for Job {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Position: {}, Company: {}, Date Posted: {}, Location: {}, Remuneration: {}, Tags: {:?}, Job Site: {}",
            self.title, self.company, self.date_posted, self.location, self.remuneration, self.tags, self.site
        )
    }
}

/// All repository structs must implement the JobRepository trait.
pub trait JobRepository {
    /// Creates a repository instance with default fields.
    fn default() -> Self;

    /// Imports a vector of Jobs instances into the repository.
    fn import(&mut self, jobs: Vec<&mut Vec<Job>>) -> &mut Self;

    /// An optional filter to remove invalid jobs.
    fn filter(&mut self, on: fn(job: &Job) -> bool) -> &mut Self;

    /// Indexes Job instances for quick searching - this will depend on the structure of your repository
    /// and how you choose to index the jobs it holds.
    fn index(&mut self);
}

/// Represents specific skills for Software jobs.
#[derive(Debug, Eq, Hash, Clone, PartialEq)]
pub enum Skill {
    Backend,
    Frontend,
    Fullstack,
}

/// Represents skill levels for Software jobs.
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

/// Represents locations for Software jobs.
#[derive(Debug, Eq, Hash, Clone, PartialEq)]
pub enum Location {
    Remote,
    Onsite,
}

/// Represents a repository for Software jobs - this can be customised to fit the relevant job market.
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
        self.all.retain(|job| on(job));
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
                job.index_with_type(&mut self.location, Location::Remote);
            } else {
                job.index_with_type(&mut self.location, Location::Onsite);
            }

            // index by skill
            if job.title_contains("backend") { job.index_with_type(&mut self.skill, Skill::Backend); }
            if job.title_contains("frontend") { job.index_with_type(&mut self.skill, Skill::Frontend); }
            if job.title_contains("fullstack") { job.index_with_type(&mut self.skill, Skill::Fullstack); }

            // index by level
            if job.title_contains("junior") { job.index_with_type(&mut self.level, Level::Junior); }
            if job.title_contains("intermediate") { job.index_with_type(&mut self.level, Level::Intermediate); }
            if job.title_contains("senior") || job.title_contains("snr") || job.title_contains("sr") {
                job.index_with_type(&mut self.level, Level::Senior);
            }
            if job.title_contains("staff") { job.index_with_type(&mut self.level, Level::Staff); }
            if job.title_contains("lead") { job.index_with_type(&mut self.level, Level::Lead); }
            if job.title_contains("principle") { job.index_with_type(&mut self.level, Level::Principle); }
            if job.title_contains("manager") { job.index_with_type(&mut self.level, Level::Manager); }
        }
    }
}
