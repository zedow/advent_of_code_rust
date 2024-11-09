use itertools::Itertools;

use crate::common::parsing::parse_numbers;

struct Row(i64, i64, i64);
type Map = Vec<Row>;
type Seed = (i64, i64);

pub fn solve_part_one(input: &str) -> String {
    let (seeds, maps) = parse_maps(input);
    let seeds = seeds
        .iter()
        .map(|seed| {
            let base_seed = *seed;
            let mut seeds: Vec<Seed> = vec![(base_seed, base_seed + 1)];
            for map in &maps {
                seeds = process_seeds(seeds, map);
            }
            (*seed, *seed)
        })
        .collect();
    solve(seeds, &maps)
}

pub fn solve_part_two(input: &str) -> String {
    let (seeds, maps) = parse_maps(input);
    let seeds = seeds
        .iter()
        .chunks(2)
        .into_iter()
        .map(|mut chunk| {
            let base_seed = *chunk.next().unwrap();
            (base_seed, base_seed + *chunk.next().unwrap() - 1)
        })
        .collect();
    solve(seeds, &maps)
}

fn solve(seeds: Vec<Seed>, maps: &Vec<Vec<Row>>) -> String {
    seeds
        .iter()
        .flat_map(|seed| {
            let mut seeds: Vec<Seed> = vec![*seed];
            for map in maps {
                seeds = process_seeds(seeds, map);
            }
            seeds
        })
        .map(|seed: Seed| seed.0)
        .min()
        .unwrap()
        .to_string()
}

fn process_seeds(seed: Vec<Seed>, rows: &Vec<Row>) -> Vec<Seed> {
    let mut output = Vec::<Seed>::new();
    let mut input = seed;
    while let Some((seed, max_seed)) = input.pop() {
        let intersects = rows
            .iter()
            .find(|Row(_, src, src_max)| intersects(&(*src, *src_max), &(seed, max_seed)));
        match intersects {
            Some(Row(intersected_dest, intersected_src, intersected_src_max)) => {
                if intersected_src <= &seed && &max_seed <= intersected_src_max {
                    let shift = intersected_dest - intersected_src;
                    output.push((seed + shift, max_seed + shift));
                } else if &seed < intersected_src {
                    input.push((seed, intersected_src - 1));
                    input.push((*intersected_src, max_seed));
                } else {
                    input.push((seed, *intersected_src_max));
                    input.push((intersected_src_max + 1, max_seed));
                }
            }
            None => {
                output.push((seed, max_seed));
            }
        }
    }

    output
}

fn intersects(seed: &Seed, other: &Seed) -> bool {
    seed.0 <= other.1 && other.0 <= seed.1
}

fn parse_maps(input: &str) -> (Vec<i64>, Vec<Map>) {
    let sections: Vec<&str> = input.split("\n\n").collect();
    let seeds = parse_numbers::<i64>(sections.iter().nth(0).unwrap());
    let maps: Vec<Map> = sections
        .iter()
        .skip(1)
        .map(|map| {
            let maps: Vec<Row> = map
                .lines()
                .skip(1)
                .map(|line| {
                    let numbers = parse_numbers::<i64>(line);
                    Row(numbers[0], numbers[1], numbers[1] + numbers[2] - 1)
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
        let result = solve_part_two(
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
        assert_eq!(result, "46");
    }
}
