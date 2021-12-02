use std::{num::ParseIntError, str::FromStr};

fn main() {
    let input = include_str!("input.txt");
    let input = input
        .lines()
        .enumerate()
        .filter(|(_linenum, line)| !line.is_empty())
        .map(|(linenum, line)| match line.parse::<Command>() {
            Ok(command) => command,
            Err(err) => panic!("Got {:?} error at line {} of input.txt", err, linenum),
        })
        .collect::<Vec<Command>>();

    println!("{}", first(&input));
    println!("{}", second(&input));
}

fn first(input: &[Command]) -> i32 {
    let mut sub = Submarine::new();
    for command in input {
        match command {
            Command::Forward(n) => sub.horizontal_position += n,
            Command::Down(n) => sub.depth += n,
            Command::Up(n) => sub.depth -= n,
        };
    }

    sub.horizontal_position * sub.depth
}

struct Submarine {
    horizontal_position: i32,
    depth: i32, // bigger depth means the submarine is deeper
    aim: i32,
}
impl Submarine {
    fn new() -> Self {
        Self {
            horizontal_position: 0,
            depth: 0,
            aim: 0,
        }
    }
}

fn second(input: &[Command]) -> i32 {
    let mut sub = Submarine::new();
    for command in input {
        match command {
            Command::Forward(n) => {
                sub.horizontal_position += n;
                sub.depth += sub.aim * n;
            }
            Command::Down(n) => sub.aim += n,
            Command::Up(n) => sub.aim -= n,
        };
    }

    sub.horizontal_position * sub.depth
}

enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}
impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split_whitespace();
        match s.next() {
            Some(command) => {
                let dist: i32 = s
                    .next()
                    .ok_or_else(|| "Missing value".to_string())?
                    .parse()
                    .map_err(|err: ParseIntError| err.to_string())?;
                match command {
                    "forward" => Ok(Command::Forward(dist)),
                    "down" => Ok(Command::Down(dist)),
                    "up" => Ok(Command::Up(dist)),
                    _ => Err(format!("Invalid command: {}", command)),
                }
            }
            None => Err("Missing command".to_string()),
        }
    }
}

#[test]
fn day2_first() {
    let input: Vec<Command> = vec![
        "forward 5",
        "down 5",
        "forward 8",
        "up 3",
        "down 8",
        "forward 2",
    ]
    .iter()
    .map(|s| s.parse().unwrap())
    .collect();
    assert_eq!(first(&input), 150);
}

#[test]
fn day2_second() {
    let input: Vec<Command> = vec![
        "forward 5",
        "down 5",
        "forward 8",
        "up 3",
        "down 8",
        "forward 2",
    ]
    .iter()
    .map(|s| s.parse().unwrap())
    .collect();
    assert_eq!(second(&input), 900);
}
