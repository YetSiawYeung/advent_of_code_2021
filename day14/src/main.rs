use std::{collections::HashMap, str::FromStr};

fn main() {
    let input: Polymer = include_str!("input.txt").parse().unwrap();

    println!("{}", first(input.clone()));
    println!("{}", second(input));
}

fn first(mut polymer: Polymer) -> i64 {
    const STEPS: i32 = 10;

    for _ in 0..STEPS {
        polymer.extend_polymer();
    }

    let counts = polymer.element_counts();

    let mut counts = counts
        .into_iter()
        .map(|(_chr, count)| count)
        .collect::<Vec<_>>();
    counts.sort_unstable_by_key(|count| *count);

    counts.last().unwrap() - counts.first().unwrap()
}

fn second(mut polymer: Polymer) -> i64 {
    const STEPS: i32 = 40;

    for _ in 0..STEPS {
        polymer.extend_polymer();
    }

    let counts = polymer.element_counts();

    let mut counts = counts
        .into_iter()
        .map(|(_chr, count)| count)
        .collect::<Vec<_>>();
    counts.sort_unstable_by_key(|count| *count);

    counts.last().unwrap() - counts.first().unwrap()
}

#[derive(Debug, Clone)]
struct Polymer {
    sequences: HashMap<Vec<char>, i64>,
    insertion_rules: HashMap<[char; 2], [[char; 2]; 2]>,
    // the first and last character of input will remain the first or last because
    // chracters are added to the middle of a sequence
    first: char,
    last: char,
}
impl Polymer {
    fn extend_polymer(&mut self) {
        let mut new_seq = HashMap::with_capacity(self.sequences.len() * 2);

        for (seq, count) in &self.sequences {
            if self.insertion_rules.contains_key(seq.as_slice()) {
                for output in self.insertion_rules[seq.as_slice()] {
                    *new_seq.entry(output.to_vec()).or_insert(0) += count;
                }
            } else {
                *new_seq.entry(seq.clone()).or_insert(0) += count;
            }
        }

        self.sequences = new_seq;
    }
    fn element_counts(&self) -> HashMap<char, i64> {
        let mut counts = HashMap::new();
        for (seq, count) in &self.sequences {
            for chr in seq.iter() {
                *counts.entry(*chr).or_insert(0) += count;
            }
        }
        *counts.entry(self.first).or_insert(0) += 1;
        *counts.entry(self.last).or_insert(0) += 1;

        for count in counts.values_mut() {
            *count /= 2;
        }

        counts
    }
}
impl FromStr for Polymer {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert!(s.is_ascii());

        let mut s = s.lines();
        let mut start = None;

        for line in &mut s {
            if line.is_empty() {
                break;
            }
            start = Some(line);
        }
        let start: Vec<char> = start.unwrap().chars().collect();
        let mut sequences = HashMap::new();
        for window in start.windows(2) {
            let window = window.to_vec();
            *sequences.entry(window).or_insert(0) += 1;
        }

        let insertion_rules = s
            .map(|line| {
                let mut line = line.split("->");
                let input = line.next().unwrap().trim().chars().collect::<Vec<_>>();
                let output = line.next().unwrap().trim().chars().collect::<Vec<_>>();
                (
                    [input[0], input[1]],
                    [[input[0], output[0]], [output[0], input[1]]],
                )
            })
            .collect();

        Ok(Self {
            sequences,
            insertion_rules,
            first: *start.first().unwrap(),
            last: *start.last().unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{first, second, Polymer};

    fn load_test_data() -> Polymer {
        include_str!("test.txt").parse().unwrap()
    }

    #[test]
    fn day14_first() {
        let input = load_test_data();
        assert_eq!(first(input), 1588);
    }
    #[test]
    fn day14_second() {
        let input = load_test_data();
        assert_eq!(second(input), 2_188_189_693_529);
    }
}
