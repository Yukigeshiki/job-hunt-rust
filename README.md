# job-hunt-rust
A locally run job aggregator written in Rust, with in-memory datastore, query language (JHQL), and REPL.

I have used this project to try become more familiar with the Rust programming language. I'm sure there are many pieces of code that can be improved, and I'd love to hear any feedback from more experienced Rust programmers!
My hope is that Job Hunt will be easily customisable to suit any job market, but currently it is directed at Web3/Crypto engineering jobs.

### Items
- Scrapers
  - So far there are scrapers for https://web3.career/ and https://useweb3.xyz/jobs, but I intend to add many more.
- In memory datastore
  - I'm sure improvements can be made here, but it is essentially complete. Jobs are filtered and then indexed. For the small amount of data, indexing is not really needed, but I decided to do it anyway just to make things more interesting.
- REPL
  - Again, I'm sure there are improvements that can be made, but it is also complete.
- JHQL (Job Hunt Query Language) - 🚧 under construction 🚧
  - I am currently working on a query language (using pest) that will query the indexed data; for example, a specific job skill, seniority level, or both.
  - Currently, there is only one query command; `fetch jobs`. This will fetch all jobs and print them to your terminal (ordered descending by date posted). There is also the `refresh` command which will re-scrape, then re-initialise the in memory datastore,
    and the `exit` command to exit out of Job Hunt.

### How to Run Job Hunt

From the project root directory run:

```bash
cargo build --release
```

Then run:

```bash
./target/release/jobhunt
```

You should see the below info messages followed by a prompt. Happy Job Hunting!

```
Populating/indexing local datastore...
Population/indexing completed successfully! Please begin your job hunt by entering a query:
```
