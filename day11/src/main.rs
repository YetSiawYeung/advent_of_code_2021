use std::str::FromStr;

fn main() {
    let input: Octupuses = include_str!("input.txt").parse().unwrap();

    println!("{}", first(input.clone()));
    println!("{}", second(input));
}

fn first(mut octopuses: Octupuses) -> i32 {
    const STEPS: i32 = 100;
    let mut flash_count = 0;

    for _ in 0..STEPS {
        flash_count += octopuses.count_flashes();
    }

    flash_count
}

fn second(mut octopuses: Octupuses) -> i32 {
    for steps in 1.. {
        octopuses.count_flashes();
        if octopuses
            .0
            .iter()
            .all(|row| row.iter().all(|octupus| *octupus == 0))
        {
            return steps;
        }
    }

    panic!("the octopuses never all flashed at the same time")
}

#[derive(Debug, Clone)]
struct Octupuses(Vec<Vec<i32>>);
impl Octupuses {
    fn count_flashes(&mut self) -> i32 {
        let mut flashed = vec![vec![false; self.0[0].len()]; self.0.len()];
        for row in &mut self.0 {
            for octopus in row {
                *octopus += 1;
            }
        }

        loop {
            let mut flashers = Vec::new();
            for (x, row) in self.0.iter_mut().enumerate() {
                for (y, octupus) in row.iter_mut().enumerate() {
                    if *octupus > 9 {
                        *octupus = 0;
                        flashers.push((x, y));
                        flashed[x][y] = true;
                    }
                }
            }

            if flashers.is_empty() {
                break;
            }
            for (x, y) in flashers {
                let x_min = x.saturating_sub(1);
                let x_max = x.checked_add(1).unwrap_or(x).min(self.0.len() - 1);
                let y_min = y.saturating_sub(1);
                let y_max = y.checked_add(1).unwrap_or(y).min(self.0[0].len() - 1);

                for (x, row) in flashed.iter().enumerate().take(x_max + 1).skip(x_min) {
                    for (y, is_flashed) in row.iter().enumerate().take(y_max + 1).skip(y_min) {
                        if !is_flashed {
                            self.0[x][y] += 1;
                        }
                    }
                }
            }
        }

        flashed
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|octopus_flashed| **octopus_flashed)
                    .count() as i32
            })
            .sum()
    }
}
impl FromStr for Octupuses {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .filter(|line| !line.is_empty())
                .map(|row| {
                    row.bytes()
                        .map(|i| match i {
                            i @ b'0'..=b'9' => Ok((i - b'0') as i32),
                            _ => Err(format!("Not a number: 0b{}", i)),
                        })
                        .collect::<Result<Vec<i32>, Self::Err>>()
                })
                .collect::<Result<Vec<_>, Self::Err>>()?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::{first, second, Octupuses};

    fn load_test_data() -> Octupuses {
        include_str!("test.txt").parse().unwrap()
    }

    #[test]
    fn day11_first() {
        let input = load_test_data();
        assert_eq!(first(input), 1656);
    }

    #[test]
    fn day11_second() {
        let input = load_test_data();
        assert_eq!(second(input), 195);
    }
}
