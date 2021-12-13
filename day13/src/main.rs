use std::{fmt::Display, str::FromStr};

fn main() {
    let input: PaperFolds = include_str!("input.txt").parse().unwrap();

    println!("{}", first(input.clone()));
    second(input);
}

fn first(mut paper: PaperFolds) -> i32 {
    paper.fold_n(1);
    paper.count_dots()
}

fn second(mut paper: PaperFolds) {
    paper.fold();
    println!("{}", paper.paper);
}

#[derive(Debug, Clone)]
struct Paper(Vec<Vec<bool>>);
impl Paper {
    fn from_dots(dots: &[(usize, usize)]) -> Self {
        let max_x = dots.iter().map(|(x, _y)| x).max().copied().unwrap_or(0);
        let max_y = dots.iter().map(|(_x, y)| y).max().copied().unwrap_or(0);

        let mut paper = vec![vec![false; max_x + 1]; max_y + 1];
        for (x, y) in dots {
            paper[*y][*x] = true;
        }

        Self(paper)
    }
    fn count_dots(&self) -> i32 {
        self.0
            .iter()
            .map(|row| row.iter().filter(|dot| **dot).count() as i32)
            .sum()
    }
    fn fold(&mut self, fold: &Fold) {
        match fold {
            Fold::FoldAtX(i) => self.fold_x(*i),
            Fold::FoldAtY(i) => self.fold_y(*i),
        }
    }
    fn fold_x(&mut self, column: usize) {
        assert!(column >= self.0[0].len() / 2);

        for row in &mut self.0 {
            for i in 0..column.min(row.len() - column) {
                let i = i + 1;
                row[column - i] |= row[column + i];
            }
            row.truncate(column);
        }
    }
    fn fold_y(&mut self, column: usize) {
        assert!(column >= self.0.len() / 2);

        for i in 0..column.min(self.0.len() - column) {
            let i = i + 1;
            for j in 0..self.0[0].len() {
                self.0[column - i][j] |= self.0[column + i][j];
            }
        }

        self.0.truncate(column);
    }
}
impl Display for Paper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            for dot in row {
                if *dot {
                    write!(f, "#")?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f,)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
enum Fold {
    FoldAtX(usize),
    FoldAtY(usize),
}
impl FromStr for Fold {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let error = "Invalid format. Expected `fold along <axis>=<number>`".to_string();
        if let Some(s) = s.strip_prefix("fold along") {
            let mut s = s.trim().split('=');

            match s.next() {
                Some("x") => Ok(Fold::FoldAtX(
                    s.next()
                        .ok_or_else(|| error.clone())?
                        .parse()
                        .map_err(|_| error)?,
                )),
                Some("y") => Ok(Fold::FoldAtY(
                    s.next()
                        .ok_or_else(|| error.clone())?
                        .parse()
                        .map_err(|_| error)?,
                )),
                _ => Err(error),
            }
        } else {
            Err(error)
        }
    }
}

#[derive(Debug, Clone)]
struct PaperFolds {
    paper: Paper,
    folds: Vec<Fold>,
}
impl PaperFolds {
    fn fold_n(&mut self, n: usize) {
        for fold in &self.folds[..n] {
            self.paper.fold(fold);
        }
        self.folds = self.folds[n..].to_vec();
    }
    fn fold(&mut self) {
        self.fold_n(self.folds.len());
    }
    fn count_dots(&self) -> i32 {
        self.paper.count_dots()
    }
}
impl FromStr for PaperFolds {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.lines();
        let mut dots: Vec<(usize, usize)> = Vec::new();

        for line in &mut s {
            if line.is_empty() {
                break;
            }
            let mut line = line.split(',');
            let x = line.next().unwrap().parse().unwrap();
            let y = line.next().unwrap().parse().unwrap();
            dots.push((x, y));
        }

        let folds: Vec<Fold> = s.map(str::parse).collect::<Result<Vec<Fold>, _>>()?;
        let paper = Paper::from_dots(&dots);
        Ok(Self { paper, folds })
    }
}

#[cfg(test)]
mod tests {
    use crate::{first, PaperFolds};

    fn load_test_data() -> PaperFolds {
        include_str!("test.txt").parse().unwrap()
    }

    #[test]
    fn day13_first() {
        let input = load_test_data();
        assert_eq!(first(input), 17);
    }
}
