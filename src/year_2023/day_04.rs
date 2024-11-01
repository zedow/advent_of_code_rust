use regex::Regex;

pub fn solve_part_one(input: &str) -> String {
    input
        .lines()
        .map(parse_card)
        .filter(|card| card.matches > 0)
        .map(|card| (2 as u32).pow(card.matches - 1))
        .sum::<u32>()
        .to_string()
}

pub fn solve_part_two(input: &str) -> String {
    let cards: Vec<Card> = input.lines().map(parse_card).collect();
    let mut counts: Vec<u32> = cards.iter().map(|_| 1).collect();
    for (index, card) in cards.iter().enumerate() {
        let count = counts[index];
        for n in 1..=card.matches {
            counts[index + n as usize] += count;
        }
    }

    counts.iter().sum::<u32>().to_string()
}

fn parse_card(line: &str) -> Card {
    let split_regex = Regex::new(r"[:|]").unwrap();
    let mut parts = split_regex.split(&line).skip(1);
    let winning_numbers = parse_numbers(parts.next().unwrap());
    let played_numbers = parse_numbers(parts.next().unwrap());
    Card {
        matches: played_numbers
            .iter()
            .filter(|played| winning_numbers.contains(played))
            .count() as u32,
    }
}

fn parse_numbers(input: &str) -> Vec<&str> {
    let regex = Regex::new(r"\d+").unwrap();
    regex.find_iter(input).map(|mat| mat.as_str()).collect()
}

struct Card {
    matches: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let result = solve_part_one(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(result, "13");
    }

    #[test]
    fn part_two_example() {
        let result = solve_part_two(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(result, "30");
    }
}
