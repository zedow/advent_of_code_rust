# Advent of code

The purpose of this project is to solve every [Advent of code](https://adventofcode.com/) puzzle in Rust 🦀.
I'm still learning Rust, and it's a great opportunity to improve my skills and learn more about the language.
The idea is to solve the puzzles from each year in this project and add features over time:

- [x] Create a template system to scaffold a puzzle for each day
- [ ] Run the solution for a specific day
- [ ] Run all solutions
- [ ] Benchmark solutions and updating the README to reflect the latest benchmarks

# Commands

The command system is based on the Clap crate, which parses commands defined in the Commands enum in src/commands.rs.

```rust
#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Scaffold { year: u16, day: String },
    Solve,
}
```

## Scaffold

```console
cargo run scaffold {year} {day}
```

to create a Rust file based on src/template/template.txt, which contains the required functions and unit test setup.
