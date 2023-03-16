# job-hunt-rust
A locally run recent job aggregator written in Rust, with in-memory datastore, query language (JHQL), and REPL.

I have used this project to try become more familiar with the Rust programming language. I'm sure there are many pieces of code that can be improved, and I'd love to hear any feedback from more experienced Rust programmers!
My hope is that Job Hunt will be easily customisable to suit any job market, but currently it is directed at Web3/Crypto engineering jobs.

### Items
- Scrapers
  - So far there are scrapers for the below URLs, but I intend to add more.
    - https://web3.career/
    - https://useweb3.xyz/jobs/t/engineering/
    - https://cryptojobslist.com/engineering?sort=recent
    - https://jobs.solana.com/jobs?filter=eyJqb2JfZnVuY3Rpb25zIjpbIlNvZnR3YXJlIEVuZ2luZWVyaW5nIl19
    - https://careers.substrate.io/jobs?filter=eyJqb2JfZnVuY3Rpb25zIjpbIlNvZnR3YXJlIEVuZ2luZWVyaW5nIl19
    - https://careers.near.org/jobs?filter=eyJqb2JfZnVuY3Rpb25zIjpbIlNvZnR3YXJlIEVuZ2luZWVyaW5nIl19
- In memory datastore
  - I'm sure improvements can be made here, but it is essentially complete. Jobs are filtered and then indexed. For the small amount of data, indexing is really not needed, but I decided to do it anyway just to make things more interesting.
- REPL
  - Again, I'm sure there are improvements that can be made, but it is also complete.
- JHQL (Job Hunt Query Language) - 🚧 under construction 🚧
  - I am currently working on a query language and parser (using pest), that will query the indexed data; for example, a specific job skill, seniority level, or both.
  - Currently, there is only one query command; `fetch jobs`. This will fetch all jobs and print them to your terminal (ordered ascending by date posted and descending by company name). There is also the `refresh` command which will re-scrape,
    then re-initialise the in-memory datastore, and the `exit` command to exit out of Job Hunt.

### How to Run Job Hunt

First make sure you have Rust installed. To do this you can follow the instructions found [here](https://www.rust-lang.org/tools/install).

Once installation is complete, clone this repo and from the root directory run:

```bash
cargo build --release
```

Then run:

```bash
./target/release/jobhunt
```

Or if you'd prefer not to build the application first, simply run:

```bash
cargo run
```

You should see the below info messages followed by a prompt. Happy Job Hunting!

```
Populating/indexing local datastore...
Population/indexing completed successfully! Welcome, please begin your job hunt by entering a query:
```
