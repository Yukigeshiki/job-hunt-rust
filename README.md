# job-hunt-rust
A locally run job aggregator written in Rust, with in-memory datastore, query language (JHQL), and REPL.

I have used this project to try become more familiar with the Rust programming language. I'm sure there are many pieces of code that can be improved, and I'd love to hear any feedback from more experienced Rust programmers!

### Items
- Scrapers
  - So far there are scrapers for https://web3.career/ and https://useweb3.xyz/jobs, but I intend to add many more.
- In memory datastore
  - I'm sure improvements can be made here, but it is essentially complete. Jobs are filtered and then indexed. For the small amount of data, indexing is not really needed, but I decided to do it anyway just to make things more interesting.
- REPL
  - Again, I'm sure there are improvements that can be made, but it is also complete.
- JHQL (Job Hunt Query Language) - 🚧 under construction 🚧
  - I am currently working on a query language (using pest) that will query the indexed data; for example, a specific job skill, seniority level, or both.
  - Currently, there are only 2 query commands; `fetch jobs` and `refresh`. The first will fetch all jobs and print them to your terminal (ordered descending by date posted), and the second will re-scrape, then re-initialise the in memory datastore.

### How to Run Job Hunt

From the project root directory run:

```bash
cargo build
```

Then run:

```bash
./target/debug/jobhunt
```

You should see the below info messages followed by a prompt. Happy Job Hunting!

```
Populating/indexing local datastore...
Population/indexing completed successfully! Please begin your job hunt by entering a query:
```