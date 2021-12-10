use std::collections::HashMap;

fn main() {
    let input: Vec<&str> = include_str!("input.txt")
        .lines()
        .filter(|line| !line.is_empty())
        .collect();

    println!("{}", first(&input));
    println!("{}", second(&input));
}

fn first(strings: &[&str]) -> i32 {
    let points: HashMap<char, i32> = {
        let mut points = HashMap::new();
        points.insert(')', 3);
        points.insert(']', 57);
        points.insert('}', 1197);
        points.insert('>', 25137);
        points
    };

    strings
        .iter()
        .map(|brackets| {
            if let Err(chr) = check_brackets(brackets) {
                points[&chr]
            } else {
                0
            }
        })
        .sum()
}

fn second(strings: &[&str]) -> i64 {
    let points: HashMap<char, i64> = {
        let mut points = HashMap::new();
        points.insert(')', 1);
        points.insert(']', 2);
        points.insert('}', 3);
        points.insert('>', 4);
        points
    };

    let mut scores: Vec<i64> = strings
        .iter()
        .filter_map(|brackets| {
            if let Ok(stack) = check_brackets(brackets) {
                Some(stack.iter().fold(0, |acc, chr| acc * 5 + points[chr]))
            } else {
                None
            }
        })
        .collect();
    scores.sort_unstable();

    scores[scores.len() / 2]
}

fn check_brackets(brackets: &str) -> Result<Vec<char>, char> {
    // returns ok if incomplete with a list of required closing brackets in order
    // returns err if corrupted
    let matching_pair: HashMap<char, char> = {
        let mut matching_pair = HashMap::new();
        matching_pair.insert('(', ')');
        matching_pair.insert('[', ']');
        matching_pair.insert('{', '}');
        matching_pair.insert('<', '>');
        matching_pair
    };
    let mut stack = Vec::new();
    for chr in brackets.chars() {
        match chr {
            '(' | '[' | '{' | '<' => {
                stack.push(chr);
            }
            ')' | ']' | '}' | '>' => {
                if chr != matching_pair[&stack.pop().unwrap()] {
                    return Err(chr);
                }
            }
            _ => unreachable!(),
        }
    }

    // no corrupted brackets, just incomplete line
    Ok(stack
        .into_iter()
        .rev()
        .map(|chr| matching_pair[&chr])
        .collect())
}

#[cfg(test)]
mod tests {
    use crate::{first, second};

    fn load_test_data() -> Vec<&'static str> {
        include_str!("test.txt")
            .lines()
            .filter(|line| !line.is_empty())
            .collect()
    }

    #[test]
    fn day10_first() {
        let input = load_test_data();
        assert_eq!(first(&input), 26397);
    }

    #[test]
    fn day10_second() {
        let input = load_test_data();
        assert_eq!(second(&input), 288957);
    }
}
