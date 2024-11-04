use regex::Regex;
use std::fmt::Debug;
use std::str::FromStr;

pub fn parse_numbers<T>(input: &str) -> Vec<T>
where
    T: FromStr + Debug,
    T::Err: Debug,
{
    let regex = Regex::new(r"\d+").unwrap();
    regex
        .find_iter(input)
        .map(|mat| mat.as_str().parse::<T>().unwrap())
        .collect()
}
