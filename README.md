# Castiron
A podcast player written in Rust.



https://github.com/StevieC7/castiron/assets/46426105/a2d8f29b-1a03-4e73-b70f-4e6009670569



## Disclaimer
This is still being written. It is not full-featured. This is a project being built for the purpose of learning the Rust programming language and exploring its ecosystem.

## How to Run
- Clone code
- `cargo run`

## Running Tests
Because the unit tests each request a connection to the same database, you might need to run tests in single thread mode, like this:

`cargo test -- --test-threads=1`
