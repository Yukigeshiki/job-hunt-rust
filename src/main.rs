use jobhunt::{Initializer, SoftwareJobsInitializer};

fn main() {
    let repo = SoftwareJobsInitializer::init().unwrap();

    println!("{}", repo.all.len());
    println!("{:?}", repo.all);
    println!("{:?}", repo.date);
    println!("{:?}", repo.company);
    println!("{:?}", repo.location);
    println!("{:?}", repo.skill);
    println!("{:?}", repo.level);
}
