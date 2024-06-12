# Castiron
A desktop podcast player application written in Rust.




https://github.com/StevieC7/castiron/assets/46426105/2b50a4c2-5829-4b76-844d-b8f2e740d71c




## Features
- Add podcasts via RSS feed
- Queue episodes to have them automatically play when the current one finishes
- Customize the player's look using themes

## Disclaimer
This is still being written. It is not full-featured. This is a project being built for the purpose of learning the Rust programming language and exploring its ecosystem.

## How to Run
- Clone code
- `cargo run`

## Running Tests
Because the unit tests each request a connection to the same database, you might need to run tests in single thread mode, like this:

`cargo test -- --test-threads=1`
