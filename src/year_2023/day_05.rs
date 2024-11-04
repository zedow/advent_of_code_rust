use crate::common::parsing::parse_numbers;

struct Row(u64, u64, u64);
type Map = Vec<Row>;

pub fn solve_part_one(input: &str) -> String {
    let (seeds, maps) = parse_maps(input);
    seeds
        .iter()
        .map(|seed| {
            maps.iter().fold(*seed, |processed_seed, map| {
                let map_result = map.iter().find_map(|Row(dest, source_min, source_max)| {
                    if &processed_seed >= source_min && &processed_seed < &source_max {
                        return Some(processed_seed - source_min + dest);
                    }
                    None
                });
                map_result.or_else(|| Some(processed_seed)).unwrap()
            })
        })
        .min()
        .unwrap()
        .to_string()
}

pub fn solve_part_two(input: &str) -> String {
    String::new()
}

fn parse_maps(input: &str) -> (Vec<u64>, Vec<Map>) {
    let sections: Vec<&str> = input.split("\n\n").collect();
    let seeds = parse_numbers::<u64>(sections.iter().nth(0).unwrap());
    let maps: Vec<Map> = sections
        .iter()
        .skip(1)
        .map(|map| {
            let maps: Vec<Row> = map
                .lines()
                .skip(1)
                .map(|line| {
                    let numbers = parse_numbers::<u64>(line);
                    Row(numbers[0], numbers[1], numbers[1] + numbers[2])
                })
                .collect();
            maps
        })
        .collect();
    (seeds, maps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let result = solve_part_one(
            "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
        );
        assert_eq!(result, "35");
    }

    #[test]
    fn part_two_example() {
        let result = solve_part_two("");
        assert_eq!(result, "");
    }
}
