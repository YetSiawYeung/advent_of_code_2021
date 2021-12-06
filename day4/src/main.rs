use std::str::FromStr;

fn main() {
    let input = include_str!("input.txt");
    let input: Bingo = input.parse().unwrap();

    println!("{}", first(input.clone()));
    println!("{}", second(input));
}

fn first(mut bingo_boards: Bingo) -> i32 {
    for next_number in &bingo_boards.rolls {
        for board in &mut bingo_boards.boards {
            board.mark(*next_number);
        }

        if let Some(winning_board) = bingo_boards.boards.iter().position(Board::is_winner) {
            return next_number * bingo_boards.boards[winning_board].sum_unmarked();
        }
    }

    panic!("no winning boards after all numbers used")
}

fn second(mut bingo_boards: Bingo) -> i32 {
    let mut i = 0;
    while i < bingo_boards.rolls.len() {
        if bingo_boards.boards.len() == 1 {
            break;
        }
        let next_number = bingo_boards.rolls[i];
        for board in &mut bingo_boards.boards {
            board.mark(next_number);
        }

        bingo_boards.remove_winners();
        i += 1
    }

    let last_board = &mut bingo_boards.boards[0];
    while i < bingo_boards.rolls.len() {
        let next_number = bingo_boards.rolls[i];
        last_board.mark(next_number);
        if last_board.is_winner() {
            return next_number * last_board.sum_unmarked();
        }

        i += 1
    }

    panic!("no winning boards after all numbers used")
}

#[derive(Debug, Clone)]
struct Bingo {
    rolls: Vec<i32>,
    boards: Vec<Board>,
}
impl Bingo {
    fn remove_winners(&mut self) {
        self.boards.retain(|board| !board.is_winner());
    }
}
impl FromStr for Bingo {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.lines();

        let rolls = s
            .next()
            .unwrap()
            .split(',')
            .map(|n| n.trim().parse().unwrap())
            .collect();

        let mut s = s.peekable();
        let mut boards = Vec::new();

        fn parse_board_row(s: &str) -> Vec<(bool, i32)> {
            s.trim()
                .split_whitespace()
                .map(|i| (false, i.trim().parse().unwrap()))
                .collect()
        }
        while s.peek().is_some() {
            if s.peek().unwrap().is_empty() {
                let _empty_line = s.next();
                continue;
            }

            let first = parse_board_row(s.next().unwrap());
            let second = parse_board_row(s.next().unwrap());
            let third = parse_board_row(s.next().unwrap());
            let fourth = parse_board_row(s.next().unwrap());
            let fifth = parse_board_row(s.next().unwrap());

            if let Some(next) = s.peek() {
                if next.trim().is_empty() {
                    let _empty_line = s.next();
                }
            }

            boards.push(Board(vec![first, second, third, fourth, fifth]))
        }

        Ok(Self { rolls, boards })
    }
}

#[derive(Debug, Clone)]
struct Board(Vec<Vec<(bool, i32)>>);
impl Board {
    fn mark(&mut self, number: i32) {
        for x in 0..self.0.len() {
            for y in 0..self.0.get(0).map(|row| row.len()).unwrap_or(0) {
                if self.0[x][y].1 == number {
                    self.0[x][y].0 = true;
                    return;
                }
            }
        }

        // if number is not found, don't do anything
    }
    fn is_winner(&self) -> bool {
        // check rows
        for row in &self.0 {
            if row.iter().all(|(marked, _)| *marked) {
                return true;
            }
        }

        // check columns
        for col in 0..self.0.get(0).map(|row| row.len()).unwrap_or(0) {
            if self.0.iter().all(|row| row[col].0) {
                return true;
            }
        }

        false
    }
    fn sum_unmarked(&self) -> i32 {
        self.0
            .iter()
            .map(|row| {
                row.iter()
                    .map(|(is_marked, num)| if !*is_marked { *num } else { 0 })
                    .sum::<i32>()
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::{first, second, Bingo};

    fn load_test_data() -> Bingo {
        include_str!("test.txt").parse::<Bingo>().unwrap()
    }

    #[test]
    fn day4_first() {
        let input = load_test_data();
        assert_eq!(first(input), 4512);
    }

    #[test]
    fn day4_second() {
        let input = load_test_data();
        assert_eq!(second(input), 1924);
    }
}
