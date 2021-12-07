use std::{num::ParseIntError, str::FromStr};

fn main() {
    let input: Crabs = include_str!("input.txt").parse().unwrap();
    println!("{}", first(&input));
    println!("{}", second(&input));
}

fn first(initial_state: &Crabs) -> i32 {
    let target = median(&initial_state.0);
    initial_state.0.iter().map(|i| (i - target).abs()).sum()
}

fn median(list: &[i32]) -> i32 {
    match list.len() {
        0 => {
            panic!("empty list")
        }
        1 => list[0],
        n if n % 2 == 0 => {
            let mid = n / 2 - 1;
            (list[mid] + list[mid + 1]) / 2
        }
        n => {
            let mid = n / 2;
            list[mid]
        }
    }
}

fn second(initial_state: &Crabs) -> i32 {
    let mean = mean(&initial_state.0);
    let cost = |distance| distance * (distance + 1) / 2;

    [mean - 1, mean, mean + 1]
        .iter()
        .map(|target| {
            initial_state
                .0
                .iter()
                .map(|i| cost((i - target).abs()))
                .sum()
        })
        .min()
        .unwrap()
}

fn mean(list: &[i32]) -> i32 {
    let sum = list.iter().sum::<i32>();
    let len = list.len() as i32;
    let mean = sum / len;

    if ((mean + 1) * len - sum).abs() > (mean * len - sum).abs() {
        mean
    } else {
        mean + 1
    }
}

struct Crabs(Vec<i32>); // inner vec should be sorted
impl Crabs {
    fn new(mut crabs: Vec<i32>) -> Self {
        crabs.sort_unstable();
        Self(crabs)
    }
}
impl FromStr for Crabs {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(
            s.trim()
                .split(',')
                .map(|num| num.parse())
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::{first, second, Crabs};

    fn load_test_data() -> Crabs {
        include_str!("test.txt").parse::<Crabs>().unwrap()
    }

    #[test]
    fn day7_first() {
        let input = load_test_data();
        assert_eq!(first(&input), 37);
    }

    #[test]
    fn day7_second() {
        let input = load_test_data();
        assert_eq!(second(&input), 168);
    }
}
