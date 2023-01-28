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
    fn title_contains(&self, val: &str) -> bool {
        self.title.to_lowercase().contains(val)
    }

    fn location_contains(&self, val: &str) -> bool {
        self.location.to_lowercase().contains(val)
    }

    /// Adds a Job instance to an index map for type T.
    fn index_by<T>(&self, t: T, map: &mut HashMap<T, Vec<Job>>)
        where T: Sized + Eq + Hash
    {
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

/// All repository structs must implement the JobRepository builder trait.
pub trait JobRepository {
    /// Creates a repository instance with default fields.
    fn default() -> Self;

    /// Imports a vector of Job instances into the repository.
    fn import(&mut self, jobs: Vec<&mut Vec<Job>>) -> &mut Self;

    /// An optional filter to remove invalid jobs.
    fn filter(&mut self, on: fn(job: &Job) -> bool) -> &mut Self;

    /// Indexes Job instances for quick searching. This will depend on the structure of your
    /// repository, and how you choose to index the jobs it holds. The index method is the completing
    /// method for the JobRepository builder.
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

/// Represents a repository for Software jobs - this struct can be customised to represent jobs for
/// any market.
#[derive(Debug, Clone)]
pub struct SoftwareJobs {
    pub all: Vec<Job>,
    pub date: HashMap<String, Vec<Job>>,
    pub company: HashMap<String, Vec<Job>>,
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
            job.index_by(job.date_posted.clone(), &mut self.date);
            job.index_by(job.company.clone(), &mut self.company);

            // index by location
            if job.location_contains("remote") {
                job.index_by(Location::Remote, &mut self.location);
            } else {
                job.index_by(Location::Onsite, &mut self.location);
            }

            // index by skill
            if job.title_contains("backend") { job.index_by(Skill::Backend, &mut self.skill); }
            if job.title_contains("frontend") { job.index_by(Skill::Frontend, &mut self.skill); }
            if job.title_contains("fullstack") { job.index_by(Skill::Fullstack, &mut self.skill); }

            // index by level
            if job.title_contains("junior") { job.index_by(Level::Junior, &mut self.level); }
            if job.title_contains("intermediate") { job.index_by(Level::Intermediate, &mut self.level); }
            if job.title_contains("senior") || job.title_contains("snr") || job.title_contains("sr") {
                job.index_by(Level::Senior, &mut self.level);
            }
            if job.title_contains("staff") { job.index_by(Level::Staff, &mut self.level); }
            if job.title_contains("lead") { job.index_by(Level::Lead, &mut self.level); }
            if job.title_contains("principle") { job.index_by(Level::Principle, &mut self.level); }
            if job.title_contains("manager") { job.index_by(Level::Manager, &mut self.level); }
        }
    }
}
