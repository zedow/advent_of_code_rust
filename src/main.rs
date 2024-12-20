use advent_of_code_rust::{
    commands::{scaffold, Args, Commands},
    parse_input,
    year_2023::day_05::{solve_part_one, solve_part_two},
};
use clap::Parser;

fn main() {
    let args = Args::parse();

    match args.cmd {
        Commands::Scaffold { year, day } => scaffold::scaffold(year, day),
        Commands::Solve => solve(),
    }
    .expect("Failed to handle command");
}

fn solve() -> Result<(), Box<dyn std::error::Error>> {
    let input = parse_input("puzzles/2023/05/input.txt")?;
    println!(
        "part one answer is {} and part two answer is {}",
        solve_part_one(&input),
        solve_part_two(&input)
    );
    Ok(())
}
