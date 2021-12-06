use std::{collections::HashMap, num::ParseIntError, str::FromStr};

fn main() {
    let input: Langernfishes = include_str!("input.txt").parse().unwrap();
    println!("{}", first(&input));
    println!("{}", second(&input));
}

fn first(initial_state: &Langernfishes) -> i64 {
    const DAYS: i32 = 80;
    let mut fishies: HashMap<i32, i64> = HashMap::new();
    for fish in &initial_state.0 {
        *fishies.entry(*fish).or_default() += 1;
    }

    for _ in 0..DAYS {
        let mut new_map = HashMap::new();
        for (timer, count) in fishies.into_iter() {
            if timer == 0 {
                *new_map.entry(8).or_default() += count;
                *new_map.entry(6).or_default() += count;
            } else {
                *new_map.entry(timer - 1).or_default() += count;
            }
        }

        fishies = new_map
    }

    fishies.values().sum()
}

fn second(initial_state: &Langernfishes) -> i64 {
    const DAYS: i32 = 256;
    let mut fishies: HashMap<i32, i64> = HashMap::new();
    for fish in &initial_state.0 {
        *fishies.entry(*fish).or_default() += 1;
    }

    for _ in 0..DAYS {
        let mut new_map = HashMap::new();
        for (timer, count) in fishies.into_iter() {
            if timer == 0 {
                *new_map.entry(8).or_default() += count;
                *new_map.entry(6).or_default() += count;
            } else {
                *new_map.entry(timer - 1).or_default() += count;
            }
        }

        fishies = new_map
    }

    fishies.values().sum()
}

struct Langernfishes(Vec<i32>);
impl FromStr for Langernfishes {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.trim()
                .split(',')
                .map(|num| num.parse())
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::{first, second, Langernfishes};

    fn load_test_data() -> Langernfishes {
        include_str!("test.txt").parse::<Langernfishes>().unwrap()
    }

    #[test]
    fn day6_first() {
        let input = load_test_data();
        assert_eq!(first(&input), 5934);
    }

    #[test]
    fn day6_second() {
        let input = load_test_data();
        assert_eq!(second(&input), 26984457539);
    }
}
