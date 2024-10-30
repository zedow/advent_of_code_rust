use regex::Regex;

struct Tile {
    content: String,
    length: u32,
    x: u32,
    y: u32,
}

impl Tile {
    fn get_value(&self) -> u32 {
        self.content.parse().unwrap()
    }

    fn is_adjacent(&self, other: &Tile) -> bool {
        self.x.abs_diff(other.x) <= 1
            && self.y <= other.y + other.length
            && other.y <= self.y + self.length
    }
}

pub fn solve_part_one(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let symbols = parse(&lines, Regex::new(r"[^.0-9]").unwrap());
    let numbers = parse(&lines, Regex::new(r"\d+").unwrap());
    numbers
        .iter()
        .filter_map(|number| {
            symbols
                .iter()
                .any(|symbol| symbol.is_adjacent(&number))
                .then_some(number.get_value())
        })
        .sum::<u32>()
        .to_string()
}

pub fn solve_part_two(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let gears = parse(&lines, Regex::new(r"\*").unwrap());
    let numbers = parse(&lines, Regex::new(r"\d+").unwrap());
    gears
        .into_iter()
        .filter_map(|gear| {
            let adjacent_numbers: Vec<u32> = numbers
                .iter()
                .filter_map(|n| n.is_adjacent(&gear).then_some(n.get_value()))
                .collect();

            (adjacent_numbers.len() == 2).then_some(adjacent_numbers.iter().product::<u32>())
        })
        .sum::<u32>()
        .to_string()
}

fn parse(lines: &Vec<&str>, regex: Regex) -> Vec<Tile> {
    lines
        .into_iter()
        .enumerate()
        .map(|(index, line)| {
            let line_tiles: Vec<Tile> = regex
                .find_iter(line)
                .map(|mat| Tile {
                    content: String::from(mat.as_str()),
                    length: mat.len() as u32,
                    y: mat.start() as u32,
                    x: index as u32,
                })
                .collect();
            line_tiles
        })
        .flat_map(|tiles| tiles.into_iter())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let result = solve_part_one(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        assert_eq!(result, "4361");
    }

    #[test]
    fn part_two_example() {
        let result = solve_part_two(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        assert_eq!(result, "467835");
    }
}
