use advent_of_code_rust::{
    parse_input,
    year_2023::day_02::{solve_part_one, solve_part_two},
};
fn main() {
    let input = parse_input("puzzles/2023/02/input.txt");
    println!(
        "part one answer is {} and part two answer is {}",
        solve_part_one(&input),
        solve_part_two(&input)
    );
}
