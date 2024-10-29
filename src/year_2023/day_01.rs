pub fn solve_part_one(input: &str) -> String {
    let sum: u32 = input
        .lines()
        .into_iter()
        .map(|line| {
            let first = line.chars().find(char::is_ascii_digit).unwrap();
            let last = line.chars().rfind(char::is_ascii_digit).unwrap();
            // multiply first by 10 to place the number in the tens position
            first.to_digit(10).unwrap() * 10 + last.to_digit(10).unwrap()
        })
        .sum();
    sum.to_string()
}

const PATTERNS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn solve_part_two(input: &str) -> String {
    let sum: u32 = input
        .lines()
        .into_iter()
        .map(|line| {
            let first = search_pattern(&line, false).unwrap();
            let last = search_pattern(&line, true).unwrap();
            // multiply first by 10 to place the number in the tens position
            first * 10 + last
        })
        .sum();
    sum.to_string()
}

fn search_pattern(line: &str, is_reversed: bool) -> Option<u32> {
    let line_last_index = line.len() - 1;
    (0..=line.len()).find_map(|len| {
        let substring = if is_reversed {
            &line[line_last_index - len..]
        } else {
            &line[..len]
        };

        let current_char = if is_reversed {
            &line.chars().nth(line_last_index - len).unwrap()
        } else {
            &line.chars().nth(len).unwrap()
        };

        if let Some(index) = PATTERNS.iter().position(|&s| substring.contains(s)) {
            let result = index as u32;
            return Some(result);
        }

        if char::is_ascii_digit(current_char) {
            return Some(current_char.to_digit(10).unwrap());
        }

        None
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let result = solve_part_one(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        );
        assert_eq!(result, "142");
    }

    #[test]
    fn part_two_example() {
        let result = solve_part_two(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );
        assert_eq!(result, "281");
    }
}
