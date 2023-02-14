use std::error::Error;
use jobhunt::init_jobhunt;
use jobhunt::repository::SoftwareJobs;

fn main() -> Result<(), Box<dyn Error>> {
    init_jobhunt::<SoftwareJobs>()?;
    Ok(())
}
