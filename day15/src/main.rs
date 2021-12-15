use std::{collections::BinaryHeap, str::FromStr};

fn main() {
    let input: Cavern = include_str!("input.txt").parse().unwrap();

    println!("{}", first(input.clone()));
    println!("{}", second(input));
}

fn first(cavern: Cavern) -> i32 {
    lowest_risk_to_end(&cavern)
}

fn second(mut cavern: Cavern) -> i32 {
    cavern.expand(5);

    lowest_risk_to_end(&cavern)
}

fn lowest_risk_to_end(cavern: &Cavern) -> i32 {
    let mut current = (0, 0);
    let dst = (cavern.0.len() - 1, cavern.0[0].len() - 1);

    let mut lowest_risk =
        vec![vec![Status::Unvisited(i32::MAX); cavern.0[0].len()]; cavern.0.len()];
    lowest_risk[0][0] = Status::Visited(0);
    let mut queue: BinaryHeap<NavigationState> = BinaryHeap::new();

    while current != dst {
        let (x, y) = current;

        let current_dist = lowest_risk[x][y].distance();

        let mut neighbours = Vec::with_capacity(4);
        if x > 0 && lowest_risk[x - 1][y].is_unvisited() {
            neighbours.push((x - 1, y));
        }
        if x < dst.0 && lowest_risk[x + 1][y].is_unvisited() {
            neighbours.push((x + 1, y));
        }
        if y > 0 && lowest_risk[x][y - 1].is_unvisited() {
            neighbours.push((x, y - 1));
        }
        if y < dst.1 && lowest_risk[x][y + 1].is_unvisited() {
            neighbours.push((x, y + 1));
        }

        for (x, y) in neighbours {
            let distance = current_dist + cavern.0[x][y];
            if distance < lowest_risk[x][y].distance() {
                lowest_risk[x][y] = Status::Unvisited(distance);
            }
            queue.push(NavigationState {
                distance,
                coordinates: (x, y),
            });
        }

        lowest_risk[x][y] = Status::Visited(current_dist);
        loop {
            let state = queue.pop().unwrap();
            let (x, y) = state.coordinates;
            if lowest_risk[x][y].is_unvisited() {
                current = (x, y);
                break;
            }
        }
    }

    lowest_risk[dst.0][dst.1].distance()
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Status {
    Visited(i32),
    Unvisited(i32),
}
impl Status {
    fn distance(&self) -> i32 {
        match self {
            Status::Visited(i) | Status::Unvisited(i) => *i,
        }
    }
    fn is_unvisited(&self) -> bool {
        matches!(self, Status::Unvisited(_))
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct NavigationState {
    distance: i32,
    coordinates: (usize, usize),
}
impl PartialOrd for NavigationState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for NavigationState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance).reverse()
    }
}

#[derive(Debug, Clone)]
struct Cavern(Vec<Vec<i32>>);
impl Cavern {
    fn expand(&mut self, ratio: usize) {
        // extends self.0 by `ratio` times in both directions
        let original_max_x = self.0.len();
        let original_max_y = self.0[0].len();

        for row in &mut self.0 {
            *row = row.repeat(ratio);
            for (i, chunk) in row.chunks_exact_mut(original_max_y).enumerate() {
                let i = i as i32;
                for ele in chunk {
                    *ele += i;
                }
            }
        }

        let mut new_vec = Vec::with_capacity(ratio * self.0.len());

        for _ in 0..ratio {
            for row in &self.0 {
                new_vec.push(row.clone());
            }
        }
        for (i, rows) in new_vec.chunks_exact_mut(original_max_x).enumerate() {
            let i = i as i32;
            for row in rows {
                for ele in row {
                    *ele += i;
                }
            }
        }

        // set all numbers to the range 1..=9
        for row in &mut new_vec {
            for i in row {
                *i %= 9;
                if *i == 0 {
                    *i = 9;
                }
            }
        }

        self.0 = new_vec;

        assert!(self
            .0
            .iter()
            .all(|row| row.iter().all(|num| (1..=9).contains(num))),);
    }
}
impl FromStr for Cavern {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .filter(|line| !line.is_empty())
                .map(|line| line.bytes().map(|i| i32::from(i - b'0')).collect())
                .collect(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::{first, second, Cavern};

    fn load_test_data() -> Cavern {
        include_str!("test.txt").parse().unwrap()
    }

    #[test]
    fn day15_first() {
        let input = load_test_data();
        assert_eq!(first(input), 40);
    }
    #[test]
    fn day15_second() {
        let input = load_test_data();
        assert_eq!(second(input), 315);
    }
}
