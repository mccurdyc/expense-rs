# expense-rs

For a few years, I've used [Airtable](https://www.airtable.com) to track my purchases
and have really enjoyed it!

However, I've exceeded the free tier limits and the paid tier that I am willing to
pay for wouldn't buy me much room to grow.

I've been reading a lot about Rust and
have been trying to find a good first project and figured this would be good.
Also, this year, I've been interested in self-hosting things (e.g., pictures via
[Perkeep](https://perkeep.org/), notes via [Obsidian](https://obsidian.md/),
compute via an Intel NUC, and now my purchases via [NocoDB](https://www.nocodb.com)).

## Usage

1. Setup the environment

  ```bash
  # set required environment variables
  source expense-rc
  ```

1. Run

  ```bash
  cargo run
  ```

## TODOs

- [ ] [git-crypt](https://github.com/AGWA/git-crypt) to encrypt backup of
  expense SQLite DB.
- [ ] rate limit and paginate through Airtable results.
- [ ] implement loading merchants.
- [ ] implement loading purchases - will have to query Airtable to get merchant
  and tag names from IDs.
- [ ] familiarize myself with testing in Rust.
- [ ] familiarize myself with documenation in Rust.
