use std::{collections::VecDeque, str::FromStr};

fn main() {
    let input: FloorHeights = include_str!("input.txt").parse().unwrap();

    println!("{}", first(&input));
    println!("{}", second(&input));
}

fn first(heights: &FloorHeights) -> i32 {
    heights
        .low_points()
        .iter()
        .map(|(x, y)| heights.0[*x][*y] + 1)
        .sum()
}

fn second(heights: &FloorHeights) -> i32 {
    let mut basin_sizes: Vec<i32> = heights
        .low_points()
        .iter()
        .map(|(x, y)| heights.basin_size_at(*x, *y))
        .collect();

    basin_sizes.sort_unstable();
    basin_sizes.reverse();

    basin_sizes[0] * basin_sizes[1] * basin_sizes[2]
}

#[derive(Debug)]
struct FloorHeights(Vec<Vec<i32>>);
impl FloorHeights {
    fn low_points(&self) -> Vec<(usize, usize)> {
        let rows = self.0.len();
        let cols = self.0[0].len();
        let mut low_points = Vec::new();

        for x in 0..rows {
            for y in 0..cols {
                let mut neighbours = Vec::with_capacity(4);
                if x > 0 {
                    neighbours.push(self.0[x - 1][y])
                }
                if x < rows - 1 {
                    neighbours.push(self.0[x + 1][y])
                }
                if y > 0 {
                    neighbours.push(self.0[x][y - 1])
                }
                if y < cols - 1 {
                    neighbours.push(self.0[x][y + 1])
                }

                let current = self.0[x][y];
                if neighbours.iter().all(|i| *i > current) {
                    low_points.push((x, y));
                }
            }
        }

        low_points
    }
    fn basin_size_at(&self, x: usize, y: usize) -> i32 {
        let rows = self.0.len();
        let cols = self.0[0].len();
        let mut visited = vec![vec![false; self.0[0].len()]; self.0.len()];
        let mut queue = VecDeque::new();
        queue.push_back((x, y));

        while let Some((x, y)) = queue.pop_front() {
            if self.0[x][y] == 9 {
                continue;
            }
            visited[x][y] = true;

            if x > 0 && !visited[x - 1][y] {
                queue.push_back((x - 1, y));
            }
            if x < rows - 1 && !visited[x + 1][y] {
                queue.push_back((x + 1, y));
            }
            if y > 0 && !visited[x][y - 1] {
                queue.push_back((x, y - 1));
            }
            if y < cols - 1 && !visited[x][y + 1] {
                queue.push_back((x, y + 1));
            }
        }

        visited
            .iter()
            .map(|row| row.iter().filter(|location| **location).count() as i32)
            .sum()
    }
}
impl FromStr for FloorHeights {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.as_bytes().iter().map(|i| (*i - b'0') as _).collect())
            .collect();

        Ok(Self(s))
    }
}

#[cfg(test)]
mod tests {
    use crate::{first, second, FloorHeights};

    fn load_test_data() -> FloorHeights {
        include_str!("test.txt").parse().unwrap()
    }

    #[test]
    fn day9_first() {
        let input = load_test_data();
        assert_eq!(first(&input), 15);
    }

    #[test]
    fn day9_second() {
        let input = load_test_data();
        assert_eq!(second(&input), 1134);
    }
}
