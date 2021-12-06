use std::{
    collections::HashSet,
    ops::{Range, RangeInclusive},
    str::FromStr,
};

fn main() {
    let input: Vec<Line> = include_str!("input.txt")
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect();
    println!("{}", first(&input));
    println!("{}", second(&input));
}

fn first(lines: &[Line]) -> i32 {
    let mut overlapping_points: HashSet<Coordinates> = HashSet::new();

    for (idx, first) in lines.iter().enumerate() {
        for second in &lines[..idx] {
            match (first, second) {
                (Line::Horizontal(first), Line::Horizontal(second))
                    if first.y_coords == second.y_coords =>
                {
                    let y_axis = first.y_coords;
                    let range = range_overlap(first.start..=first.end, second.start..=second.end);

                    for x in range {
                        overlapping_points.insert(Coordinates { x, y: y_axis });
                    }
                }
                (Line::Horizontal(horizontal), Line::Vertical(vertical))
                | (Line::Vertical(vertical), Line::Horizontal(horizontal)) => {
                    let intersection = Coordinates {
                        x: vertical.x_coords,
                        y: horizontal.y_coords,
                    };

                    if vertical.contains(&intersection) && horizontal.contains(&intersection) {
                        overlapping_points.insert(intersection);
                    }
                }
                (Line::Vertical(first), Line::Vertical(second))
                    if first.x_coords == second.x_coords =>
                {
                    let x_axis = first.x_coords;
                    let range = range_overlap(first.start..=first.end, second.start..=second.end);

                    for y in range {
                        overlapping_points.insert(Coordinates { x: x_axis, y });
                    }
                }
                _ => {}
            }
        }
    }

    overlapping_points.len() as i32
}

fn second(lines: &[Line]) -> i32 {
    let mut overlapping_points: HashSet<Coordinates> = HashSet::new();

    for (idx, first) in lines.iter().enumerate() {
        for second in &lines[..idx] {
            match (first, second) {
                (Line::Horizontal(first), Line::Horizontal(second))
                    if first.y_coords == second.y_coords =>
                {
                    let y_axis = first.y_coords;
                    let range = range_overlap(first.start..=first.end, second.start..=second.end);

                    for x in range {
                        overlapping_points.insert(Coordinates { x, y: y_axis });
                    }
                }
                (Line::Horizontal(horizontal), Line::Vertical(vertical))
                | (Line::Vertical(vertical), Line::Horizontal(horizontal)) => {
                    let intersection = Coordinates {
                        x: vertical.x_coords,
                        y: horizontal.y_coords,
                    };

                    if vertical.contains(&intersection) && horizontal.contains(&intersection) {
                        overlapping_points.insert(intersection);
                    }
                }
                (Line::Vertical(first), Line::Vertical(second))
                    if first.x_coords == second.x_coords =>
                {
                    let x_axis = first.x_coords;
                    let range = range_overlap(first.start..=first.end, second.start..=second.end);

                    for y in range {
                        overlapping_points.insert(Coordinates { x: x_axis, y });
                    }
                }
                (Line::Horizontal(horizontal), Line::Diagonal(diagonal))
                | (Line::Diagonal(diagonal), Line::Horizontal(horizontal)) => {
                    let intersection = diagonal.extend_with_y(horizontal.y_coords);
                    if horizontal.contains(&intersection) && diagonal.contains(&intersection) {
                        overlapping_points.insert(intersection);
                    }
                }
                (Line::Vertical(vertical), Line::Diagonal(diagonal))
                | (Line::Diagonal(diagonal), Line::Vertical(vertical)) => {
                    let intersection = diagonal.extend_with_x(vertical.x_coords);
                    if vertical.contains(&intersection) && diagonal.contains(&intersection) {
                        overlapping_points.insert(intersection);
                    }
                }
                (Line::Diagonal(first), Line::Diagonal(second)) => {
                    if first.positive_gradient() == second.positive_gradient() {
                        let (_, intercept_1) = first.get_equation();
                        let (_, intercept_2) = second.get_equation();

                        if intercept_1 != intercept_2 {
                            // the gradient are the same because lines are all 45 degrees
                            // different intercept => lines do not touch
                            continue;
                        }

                        for coords in range_overlap_diagonal(first, second) {
                            overlapping_points.insert(coords);
                        }
                    } else {
                        // the intersection point lies on both lines
                        // y=m1*x+c1
                        // y=m2*x+c2
                        // -> m1*x+c1 = m2*x+c2
                        // -> x = (c2-c1) / (m1-m2)
                        let (gradient_1, intercept_1) = first.get_equation();
                        let (gradient_2, intercept_2) = second.get_equation();

                        let x = (intercept_2 - intercept_1) / (gradient_1 - gradient_2);

                        let y_1 = gradient_1 * x + intercept_1;
                        let y_2 = gradient_2 * x + intercept_2;

                        if y_1 == y_2
                            && first.start.x <= x
                            && x <= first.end.x
                            && second.start.x <= x
                            && x <= second.end.x
                        {
                            overlapping_points.insert(Coordinates { x, y: y_1 });
                        }
                    }
                }
                _ => {}
            }
        }
    }

    overlapping_points.len() as i32
}

fn range_overlap(first: RangeInclusive<i32>, second: RangeInclusive<i32>) -> Range<i32> {
    let first_start = *first.start();
    let first_end = *first.end();
    let second_start = *second.start();
    let second_end = *second.end();

    if first_end < second_start || first_start > second_end {
        // not overlapping
        0..0
    } else if first_start <= second_start && first_end >= second_end {
        second_start..second_end + 1
    } else if first_start >= second_start && first_end <= second_end {
        first_start..first_end + 1
    } else if second_start >= first_start && second_start <= first_end {
        second_start..first_end + 1
    } else if first_start >= second_start && first_start <= second_end {
        first_start..second_end + 1
    } else {
        panic!("unknown {:?} {:?}", first, second);
    }
}

fn range_overlap_diagonal(first: &Diagonal, second: &Diagonal) -> Vec<Coordinates> {
    let (first, second) = if first.start.x <= second.start.x {
        (first, second)
    } else {
        (second, first)
    };

    if first.end.x < second.start.x {
        vec![]
    } else if first.end.x >= second.end.x {
        make_coordinates(&second.start, &second.end)
    } else if first.start.x == second.start.x && first.end.x <= second.end.x {
        make_coordinates(&first.start, &first.end)
    } else if second.start.x <= first.end.x {
        make_coordinates(&second.start, &first.end)
    } else if first.start.x == second.start.x && second.end.x <= first.end.x {
        make_coordinates(&first.start, &second.end)
    } else {
        unreachable!()
    }
}

fn make_coordinates(start: &Coordinates, end: &Coordinates) -> Vec<Coordinates> {
    // end.x must be >= start.x
    let x_start = start.x;
    let y_start = start.y;
    let x_end = end.x;
    let y_end = end.y;
    let x_range = x_start..=x_end;

    if y_end > y_start {
        x_range
            .zip(y_start..=y_end)
            .map(|(x, y)| Coordinates { x, y })
            .collect()
    } else {
        x_range
            .zip((y_end..=y_start).rev())
            .map(|(x, y)| Coordinates { x, y })
            .collect()
    }
}

#[derive(Debug, Clone)]
struct Horizontal {
    start: i32,
    end: i32,
    y_coords: i32,
}
impl Horizontal {
    fn contains(&self, coords: &Coordinates) -> bool {
        coords.y == self.y_coords && self.start <= coords.x && coords.x <= self.end
    }
}

#[derive(Debug, Clone)]
struct Vertical {
    start: i32,
    end: i32,
    x_coords: i32,
}
impl Vertical {
    fn contains(&self, coords: &Coordinates) -> bool {
        coords.x == self.x_coords && self.start <= coords.y && coords.y <= self.end
    }
}

#[derive(Debug, Clone)]
struct Diagonal {
    // start.x must be <= end.x
    start: Coordinates,
    end: Coordinates,
}
impl Diagonal {
    fn get_equation(&self) -> (i32, i32) {
        let gradient = (self.end.y - self.start.y) / (self.end.x - self.start.x);
        let intercept = self.start.y - self.start.x * gradient;

        (gradient, intercept)
    }
    fn positive_gradient(&self) -> bool {
        self.end.y >= self.start.y
    }
    fn extend_with_x(&self, x: i32) -> Coordinates {
        let (gradient, intercept) = self.get_equation();
        Coordinates {
            x,
            y: x * gradient + intercept,
        }
    }
    fn extend_with_y(&self, y: i32) -> Coordinates {
        let (gradient, intercept) = self.get_equation();
        Coordinates {
            x: (y - intercept) / gradient,
            y,
        }
    }
    fn contains(&self, coords: &Coordinates) -> bool {
        let (gradient, intercept) = self.get_equation();

        coords.y == coords.x * gradient + intercept
            && self.start.x <= coords.x
            && coords.x <= self.end.x
    }
}

#[derive(Debug, Clone)]
enum Line {
    Horizontal(Horizontal),
    Vertical(Vertical),
    Diagonal(Diagonal), // must be 45 degree angle
}
impl FromStr for Line {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split("->");
        let start: Coordinates = s.next().unwrap().parse()?;
        let end: Coordinates = s.next().unwrap().parse()?;

        Ok(if start.x == end.x {
            let x_coords = start.x;
            let (start, end) = if start.y > end.y {
                (end.y, start.y)
            } else {
                (start.y, end.y)
            };
            Self::Vertical(Vertical {
                start,
                end,
                x_coords,
            })
        } else if start.y == end.y {
            let y_coords = start.y;
            let (start, end) = if start.x > end.x {
                (end.x, start.x)
            } else {
                (start.x, end.x)
            };
            Self::Horizontal(Horizontal {
                start,
                end,
                y_coords,
            })
        } else {
            let (start, end) = if start.x > end.x {
                (end, start)
            } else {
                (start, end)
            };
            Self::Diagonal(Diagonal { start, end })
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Coordinates {
    x: i32,
    y: i32,
}
impl FromStr for Coordinates {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.trim().split(',');
        let x = s
            .next()
            .unwrap()
            .parse()
            .map_err(|_| "Error parsing x coordinate".to_string())?;
        let y = s
            .next()
            .unwrap()
            .parse()
            .map_err(|_| "Error parsing y coordinate".to_string())?;

        Ok(Self { x, y })
    }
}

#[cfg(test)]
mod tests {
    use crate::{first, second, Line};

    fn load_test_data() -> Vec<Line> {
        include_str!("test.txt")
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.parse().unwrap())
            .collect()
    }

    #[test]
    fn day5_first() {
        let input = load_test_data();
        assert_eq!(first(&input), 5);
    }

    #[test]
    fn day5_second() {
        let input = load_test_data();
        assert_eq!(second(&input), 12);
    }
}
