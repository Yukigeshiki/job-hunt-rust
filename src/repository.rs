//! The repository module contains all datastore code.

use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::hash::Hash;
use std::thread;

use colored::Colorize;

use crate::scraper::Scraper;
use crate::site::{
    CryptoJobsList, NearJobs, Site, SolanaJobs, SubstrateJobs, UseWeb3, Web3Careers,
};

pub const THREAD_ERROR: &str = "Error in Scraper thread";
const NOT_AVAILABLE: &str = "Not available";

/// The Job struct is the repository primitive.
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Job {
    pub title: String,
    pub company: String,
    pub date_posted: String,
    pub location: String,
    pub remuneration: String,
    pub tags: Vec<String>,
    pub apply: String,
    pub site: &'static str,
}

/// Helper methods for indexing Job instances. These can be customised to fit the relevant jobs
/// type.
impl Job {
    fn title_contains(&self, pat: &str) -> bool {
        self.title.to_lowercase().contains(pat)
    }

    fn title_contains_any(&self, v: Vec<&str>) -> bool {
        for pat in v {
            if self.title.to_lowercase().contains(pat) {
                return true;
            }
        }
        false
    }

    fn location_contains(&self, pat: &str) -> bool {
        self.location.to_lowercase().contains(pat)
    }

    /// Adds a Job instance to an index map for type T.
    fn index_by<T>(&self, t: T, map: &mut HashMap<T, Vec<Job>>)
    where
        T: Sized + Eq + Hash,
    {
        map.entry(t)
            .and_modify(|vec| vec.push(self.clone()))
            .or_insert(vec![self.clone()]);
    }
}

/// Pretty print Job for debug.
impl Debug for Job {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let remuneration = if self.remuneration.is_empty() {
            NOT_AVAILABLE
        } else {
            &self.remuneration
        };
        let location = if self.location.is_empty() {
            NOT_AVAILABLE
        } else {
            &self.location
        };
        let tags = if !self.tags.is_empty() {
            format!("[ {} ]", self.tags.join(", "))
        } else {
            NOT_AVAILABLE.into()
        };
        let apply = if self.apply.is_empty() {
            NOT_AVAILABLE.green()
        } else {
            self.apply.bright_blue()
        };
        write!(
            f,
            "{} {}\n{} {}\n{} {}\n{} {}\n{} {}\n{} {}\n{} {}\n{} {}\n\n{}",
            "Position:".bold().bright_green(),
            self.title.green(),
            "Company:".bold().bright_green(),
            self.company.green(),
            "Date Posted:".bold().bright_green(),
            self.date_posted.green(),
            "Location:".bold().bright_green(),
            location.green(),
            "Remuneration:".bold().bright_green(),
            remuneration.green(),
            "Tags:".bold().bright_green(),
            tags.green(),
            "Apply:".bold().bright_green(),
            apply,
            "Site:".bold().bright_green(),
            self.site.bright_blue(),
            "+-----------------------------------------------------------------------------------\
            ---------------------------------+\n"
                .green()
        )
    }
}

/// All repository builder structs must implement the Builder trait for some repository
/// type Output. This provides the basic ETL operations.
pub trait Builder {
    /// The Output type for the builder.
    type Output: Debug;

    /// Initialises the repository with default fields.
    fn new() -> Self;

    /// Takes a vector of Job vectors (one per website scraped) and imports all Jobs into the
    /// repository.
    fn import(self, jobs: Vec<Vec<Job>>) -> Self;

    /// An optional filter to include only jobs of interest.
    fn filter<F: Fn(&Job) -> bool>(self, condition: F) -> Self;

    /// Indexes Job instances for quick searching. This will depend on the structure of your
    /// repository, and how you choose to index the jobs it holds. The index method is the
    /// completing method for the repository builder and must return the repository type Output.
    fn index(self) -> Self::Output;
}

/// Represents specific skills for Software jobs.
#[derive(Debug, Eq, Hash, Clone, PartialEq)]
pub enum Skill {
    Backend,
    Frontend,
    Fullstack,
    DevOps,
    Blockchain,
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

/// Represents a repository for Software jobs. A repository for any job type can be created.
#[derive(Debug, Default)]
pub struct SoftwareJobs {
    pub all: Vec<Job>,
    pub date: HashMap<String, Vec<Job>>,
    pub company: HashMap<String, Vec<Job>>,
    pub location: HashMap<Location, Vec<Job>>,
    pub skill: HashMap<Skill, Vec<Job>>,
    pub level: HashMap<Level, Vec<Job>>,
}

impl SoftwareJobs {
    /// Initialises a repository for Software jobs.
    pub fn init_repo() -> Self {
        let web3_careers = thread::spawn(|| Web3Careers::new().scrape());
        let use_web3 = thread::spawn(|| UseWeb3::new().scrape());
        let crypto_jobs_list = thread::spawn(|| CryptoJobsList::new().scrape());
        let solana_jobs = thread::spawn(|| SolanaJobs::new().scrape());
        let substrate_jobs = thread::spawn(|| SubstrateJobs::new().scrape());
        let near_jobs = thread::spawn(|| NearJobs::new().scrape());

        SoftwareJobsBuilder::new()
            .import(vec![
                web3_careers
                    .join()
                    .expect(THREAD_ERROR)
                    .unwrap_or_else(Web3Careers::default_if_scrape_error)
                    .jobs,
                use_web3
                    .join()
                    .expect(THREAD_ERROR)
                    .unwrap_or_else(UseWeb3::default_if_scrape_error)
                    .jobs,
                crypto_jobs_list
                    .join()
                    .expect(THREAD_ERROR)
                    .unwrap_or_else(CryptoJobsList::default_if_scrape_error)
                    .jobs,
                solana_jobs
                    .join()
                    .expect(THREAD_ERROR)
                    .unwrap_or_else(SolanaJobs::default_if_scrape_error)
                    .jobs,
                substrate_jobs
                    .join()
                    .expect(THREAD_ERROR)
                    .unwrap_or_else(SubstrateJobs::default_if_scrape_error)
                    .jobs,
                near_jobs
                    .join()
                    .expect(THREAD_ERROR)
                    .unwrap_or_else(NearJobs::default_if_scrape_error)
                    .jobs,
            ])
            .filter(|job| {
                job.title_contains_any(vec!["developer", "engineer", "engineering", "technical"])
            }) // optional filter - in this case filter on engineering jobs
            .index()
    }
}

/// Represents a repository builder for Software jobs. A repository builder for any job type can be
/// created.
struct SoftwareJobsBuilder(SoftwareJobs);

impl Builder for SoftwareJobsBuilder {
    type Output = SoftwareJobs;

    fn new() -> Self {
        Self(Default::default())
    }

    fn import(mut self, jobs: Vec<Vec<Job>>) -> Self {
        // allow duplicate job posts if they are from different sites - user can choose which site
        // to apply from
        for mut vec in jobs {
            self.0.all.append(&mut vec)
        }
        self
    }

    fn filter<F>(mut self, condition: F) -> Self
    where
        F: Fn(&Job) -> bool,
    {
        self.0.all.retain(|job| condition(job));
        self
    }

    fn index(mut self) -> Self::Output {
        self.0.all.iter().for_each(|job| {
            // index by attribute
            job.index_by(job.date_posted.clone(), &mut self.0.date);
            job.index_by(job.company.clone(), &mut self.0.company);

            // index by location
            let locations_map = &mut self.0.location;
            if job.location_contains("remote") {
                job.index_by(Location::Remote, locations_map);
            } else {
                job.index_by(Location::Onsite, locations_map);
            }

            // index by skill
            let skills_map = &mut self.0.skill;
            if job.title_contains("backend") {
                job.index_by(Skill::Backend, skills_map);
            }
            if job.title_contains("frontend") {
                job.index_by(Skill::Frontend, skills_map);
            }
            if job.title_contains("fullstack") {
                job.index_by(Skill::Fullstack, skills_map);
            }
            if job.title_contains_any(vec!["devops", "platform", "infra"]) {
                job.index_by(Skill::DevOps, skills_map);
            }
            if job.title_contains_any(vec!["blockchain", "smart contract"]) {
                job.index_by(Skill::Blockchain, skills_map);
            }

            // index by level
            let levels_map = &mut self.0.level;
            if job.title_contains("junior") {
                job.index_by(Level::Junior, levels_map);
            }
            if job.title_contains("intermediate") {
                job.index_by(Level::Intermediate, levels_map);
            }
            if job.title_contains_any(vec!["senior", "snr", "sr"]) {
                job.index_by(Level::Senior, levels_map);
            }
            if job.title_contains("staff") {
                job.index_by(Level::Staff, levels_map);
            }
            if job.title_contains("lead") {
                job.index_by(Level::Lead, levels_map);
            }
            if job.title_contains("principle") {
                job.index_by(Level::Principle, levels_map);
            }
            if job.title_contains("manager") {
                job.index_by(Level::Manager, levels_map);
            }
        });
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::{Builder, Job, Level, Location, Skill, SoftwareJobsBuilder};

    #[test]
    fn test_software_jobs_repository() {
        let repo = SoftwareJobsBuilder::new()
            .import(vec![
                vec![
                    Job {
                        title: "Engineering Manager".into(),
                        company: "Company_2".into(),
                        date_posted: "2022-07-28".into(),
                        location: "Remote".into(),
                        remuneration: "$165k - $200k".into(),
                        tags: vec!["tag1".into(), "tag2".into()],
                        apply: "https://site1.com".into(),
                        site: "https://site1.com",
                    },
                    Job {
                        title: "Senior Marketer".into(),
                        company: "Company_3".into(),
                        date_posted: "2022-07-29".into(),
                        location: "Remote".into(),
                        remuneration: "$165k - $200k".into(),
                        tags: vec!["tag1".into(), "tag2".into()],
                        apply: "https://site1.com".into(),
                        site: "https://site1.com",
                    },
                    Job {
                        title: "Platform Engineer".into(),
                        company: "Company_3".into(),
                        date_posted: "2022-07-29".into(),
                        location: "Remote".into(),
                        remuneration: "$165k - $200k".into(),
                        tags: vec!["tag1".into(), "tag2".into()],
                        apply: "https://site1.com".into(),
                        site: "https://site1.com",
                    },
                ],
                vec![
                    Job {
                        title: "Junior Fullstack Developer".into(),
                        company: "Company_1".into(),
                        date_posted: "2022-07-27".into(),
                        location: "Remote".into(),
                        remuneration: "$165k - $200k".into(),
                        tags: vec!["tag1".into(), "tag2".into()],
                        apply: "https://site2.com".into(),
                        site: "https://site2.com",
                    },
                    Job {
                        title: "Senior Backend Engineer".into(),
                        company: "Company_1".into(),
                        date_posted: "2022-07-27".into(),
                        location: "Onsite".into(),
                        remuneration: "$165k - $200k".into(),
                        tags: vec!["tag1".into(), "tag2".into()],
                        apply: "https://site2.com".into(),
                        site: "https://site2.com",
                    },
                    Job {
                        title: "Snr Backend Engineer".into(),
                        company: "Company_1".into(),
                        date_posted: "2022-07-27".into(),
                        location: "Onsite".into(),
                        remuneration: "$165k - $200k".into(),
                        tags: vec!["tag1".into(), "tag2".into()],
                        apply: "https://site2.com".into(),
                        site: "https://site2.com",
                    },
                ],
            ])
            .filter(|job| {
                job.title_contains_any(vec!["developer", "engineer", "engineering", "technical"])
            }) // optional filter - in this case filter on software jobs
            .index();

        // check index map keys
        assert_eq!(repo.all.len(), 5);
        assert_eq!(repo.date.len(), 3);
        assert_eq!(repo.company.len(), 3);
        assert_eq!(repo.location.len(), 2);
        assert_eq!(repo.skill.len(), 3);
        assert_eq!(repo.level.len(), 3);

        // check index map values
        assert_eq!(repo.location.get(&Location::Remote).unwrap().len(), 3);
        assert_eq!(repo.skill.get(&Skill::Backend).unwrap().len(), 2);
        assert_eq!(repo.skill.get(&Skill::DevOps).unwrap().len(), 1);
        assert_eq!(repo.level.get(&Level::Senior).unwrap().len(), 2);
    }
}
