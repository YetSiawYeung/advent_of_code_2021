use std::{collections::HashMap, num::ParseIntError, str::FromStr};

fn main() {
    let input: Vec<ClockDisplay> = include_str!("input.txt")
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<ClockDisplay>().unwrap())
        .collect();

    println!("{}", first(&input));
    println!("{}", second(&input));
}

fn first(displays: &[ClockDisplay]) -> i32 {
    displays
        .iter()
        .map(|display| {
            display
                .outputs
                .iter()
                .filter(|segments| {
                    [
                        2, // to display 1
                        4, // to display 4
                        3, // to display 7
                        7, // to display 8
                    ]
                    .contains(&segments.len())
                })
                .count() as i32
        })
        .sum()
}

fn second(displays: &[ClockDisplay]) -> i32 {
    // segments -> display number
    // 2 -> 1
    // 3 -> 7
    // 4 -> 4
    // 5 -> 2 or 3 or 5
    // 6 -> 0 or 6 or 9
    // 7 -> 8

    // segments by count
    // top = 8
    // top left = 6
    // top right = 8
    // middle = 7
    // bottom left = 4
    // bottom right = 9
    // bottom = 7
    displays
        .iter()
        .map(|display| {
            let one_signal = display.signals.iter().find(|sig| sig.len() == 2).unwrap();
            let four_signal = display.signals.iter().find(|sig| sig.len() == 4).unwrap();
            let seven_signal = display.signals.iter().find(|sig| sig.len() == 3).unwrap();

            let mut signal_count = HashMap::new();
            for sig in &display.signals {
                for chr in sig.chars() {
                    *signal_count.entry(chr).or_insert(0) += 1;
                }
            }

            let bottom_left = signal_count
                .iter()
                .find_map(|(chr, count)| if *count == 4 { Some(*chr) } else { None })
                .unwrap();
            let top_left = signal_count
                .iter()
                .find_map(|(chr, count)| if *count == 6 { Some(*chr) } else { None })
                .unwrap();
            let bottom_right = signal_count
                .iter()
                .find_map(|(chr, count)| if *count == 9 { Some(*chr) } else { None })
                .unwrap();
            let top_right = one_signal.chars().find(|chr| *chr != bottom_right).unwrap();
            let top = seven_signal
                .chars()
                .find(|chr| !one_signal.contains(*chr))
                .unwrap();
            let middle = four_signal
                .chars()
                .find(|chr| *chr != top_left && *chr != top_right && *chr != bottom_right)
                .unwrap();
            let bottom = ('a'..='g')
                .find(|chr| {
                    ![top, top_left, top_right, middle, bottom_left, bottom_right].contains(chr)
                })
                .unwrap();

            let fixed_signals = CorrectedSignals::new(&[
                (top, Token::Top),
                (top_left, Token::TopLeft),
                (top_right, Token::TopRight),
                (middle, Token::Middle),
                (bottom_left, Token::BottomLeft),
                (bottom_right, Token::BottomRight),
                (bottom, Token::Bottom),
            ]);

            display
                .outputs
                .iter()
                .map(|digit| fixed_signals.decode(digit))
                .fold(0, |acc, num| acc * 10 + num)
        })
        .sum()
}

struct CorrectedSignals(HashMap<char, Token>);
impl CorrectedSignals {
    fn new(mappings: &[(char, Token)]) -> Self {
        let mut hm = HashMap::new();
        for (chr, token) in mappings {
            hm.insert(*chr, *token);
        }
        Self(hm)
    }
    fn decode(&self, q: &str) -> i32 {
        // output single number
        let mut output = Segments::new();
        for chr in q.chars() {
            output.set(self.0[&chr]);
        }
        output.decode()
    }
}

#[derive(Debug, Clone, Copy)]
enum Token {
    Top,
    TopLeft,
    TopRight,
    Middle,
    BottomLeft,
    BottomRight,
    Bottom,
}

struct Segments {
    top: bool,
    top_left: bool,
    top_right: bool,
    middle: bool,
    bottom_left: bool,
    bottom_right: bool,
    bottom: bool,
}
impl Segments {
    fn new() -> Self {
        Self {
            top: false,
            top_left: false,
            top_right: false,
            middle: false,
            bottom_left: false,
            bottom_right: false,
            bottom: false,
        }
    }
    fn set(&mut self, token: Token) {
        match token {
            Token::Top => self.top = true,
            Token::TopLeft => self.top_left = true,
            Token::TopRight => self.top_right = true,
            Token::Middle => self.middle = true,
            Token::BottomLeft => self.bottom_left = true,
            Token::BottomRight => self.bottom_right = true,
            Token::Bottom => self.bottom = true,
        }
    }
    fn decode(&self) -> i32 {
        let Segments {
            top,
            top_left,
            top_right,
            middle,
            bottom_left,
            bottom_right,
            bottom,
        } = self;

        match (
            top,
            top_left,
            top_right,
            middle,
            bottom_left,
            bottom_right,
            bottom,
        ) {
            (true, true, true, false, true, true, true) => 0,
            (false, false, true, false, false, true, false) => 1,
            (true, false, true, true, true, false, true) => 2,
            (true, false, true, true, false, true, true) => 3,
            (false, true, true, true, false, true, false) => 4,
            (true, true, false, true, false, true, true) => 5,
            (true, true, false, true, true, true, true) => 6,
            (true, false, true, false, false, true, false) => 7,
            (true, true, true, true, true, true, true) => 8,
            (true, true, true, true, false, true, true) => 9,
            _ => panic!("invalid state"),
        }
    }
}

struct ClockDisplay {
    signals: Vec<String>,
    outputs: Vec<String>,
}
impl FromStr for ClockDisplay {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split('|');
        let signal = s
            .next()
            .unwrap()
            .trim()
            .split(' ')
            .map(ToString::to_string)
            .collect();
        let output = s
            .next()
            .unwrap()
            .trim()
            .split(' ')
            .map(ToString::to_string)
            .collect();

        Ok(Self {
            signals: signal,
            outputs: output,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{first, second, ClockDisplay};

    fn load_test_data() -> Vec<ClockDisplay> {
        include_str!("test.txt")
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.parse::<ClockDisplay>().unwrap())
            .collect()
    }

    #[test]
    fn day8_first() {
        let input = load_test_data();
        assert_eq!(first(&input), 26);
    }

    #[test]
    fn day8_second() {
        let input = load_test_data();
        assert_eq!(second(&input), 61229);
    }
}
