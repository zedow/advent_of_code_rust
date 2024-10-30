use itertools::Itertools;

pub fn solve_part_one(input: &str) -> String {
    let games = parse_game_sets(input);
    let (r_max, g_max, b_max) = (12, 13, 14);
    let sum: usize = games
        .into_iter()
        .enumerate()
        .filter_map(|(index, Game(r, g, b))| {
            (r > r_max || g > g_max || b > b_max).then_some(index + 1)
        })
        .sum();
    sum.to_string()
}

pub fn solve_part_two(input: &str) -> String {
    let games = parse_game_sets(input);
    let sum: u32 = games.into_iter().map(|Game(r, g, b)| r * g * b).sum();
    sum.to_string()
}

struct Game(u32, u32, u32);

fn parse_game_sets(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .chunks(2)
                .into_iter()
                // skip the chunk containing ("Game", "1:")
                .skip(1)
                .fold(Game(0, 0, 0), |Game(r, g, b), chunk| {
                    let set: Vec<&str> = chunk.collect();
                    let amount: u32 = set[0].parse().unwrap();
                    let color = set[1];
                    match color.as_bytes()[0] {
                        b'r' => Game(r.max(amount), g, b),
                        b'g' => Game(r, g.max(amount), b),
                        b'b' => Game(r, g, b.max(amount)),
                        _ => unreachable!(),
                    }
                })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let result = solve_part_one(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, "8");
    }

    #[test]
    fn part_two_example() {
        let result = solve_part_two(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, "2286");
    }
}
