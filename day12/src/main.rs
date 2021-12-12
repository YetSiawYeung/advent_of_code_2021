use std::{
    collections::{HashMap, HashSet, VecDeque},
    str::FromStr,
};

fn main() {
    let input: CaveSystem = include_str!("input.txt").parse().unwrap();

    println!("{}", first(&input));
    println!("{}", second(&input));
}

fn first(caves: &CaveSystem) -> i32 {
    let mut paths = 0;
    let mut possible_paths = VecDeque::new();
    possible_paths.push_back((0, vec![false; caves.caves.len()]));
    let end = caves.caves.len() - 1;

    while let Some((current, visited)) = possible_paths.pop_front() {
        if current == end {
            paths += 1;
            continue;
        }
        for i in
            caves.connections[current]
                .iter()
                .enumerate()
                .filter_map(|(index, is_connected)| {
                    if *is_connected
                        && match caves.caves[index] {
                            Node::Start => false,
                            Node::Small(_) => !visited[index],
                            Node::Big(_) | Node::End => true,
                        }
                    {
                        Some(index)
                    } else {
                        None
                    }
                })
        {
            let mut visited = visited.clone();
            visited[i] = true;
            possible_paths.push_back((i, visited));
        }
    }

    paths
}

fn second(caves: &CaveSystem) -> i32 {
    let mut paths = 0;
    let mut possible_paths = VecDeque::new();
    possible_paths.push_back((0, vec![0; caves.caves.len()]));
    let end = caves.caves.len() - 1;

    while let Some((current, visited)) = possible_paths.pop_front() {
        if current == end {
            paths += 1;
            continue;
        }
        let can_visit_small_caves_twice =
            visited
                .iter()
                .zip(caves.caves.iter())
                .all(|(times_visited, node)| {
                    if let Node::Small(_) = node {
                        *times_visited < 2
                    } else {
                        true
                    }
                });

        for i in
            caves.connections[current]
                .iter()
                .enumerate()
                .filter_map(|(index, is_connected)| {
                    if *is_connected
                        && match caves.caves[index] {
                            Node::Start => false,
                            Node::Small(_) => {
                                visited[index] == 0
                                    || (visited[index] == 1 && can_visit_small_caves_twice)
                            }
                            Node::Big(_) | Node::End => true,
                        }
                    {
                        Some(index)
                    } else {
                        None
                    }
                })
        {
            let mut visited = visited.clone();
            visited[i] += 1;
            possible_paths.push_back((i, visited));
        }
    }

    paths
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Node {
    Start,
    Small(String),
    Big(String),
    End,
}
impl FromStr for Node {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "start" => Ok(Self::Start),
            "end" => Ok(Self::End),
            s if s.chars().all(char::is_uppercase) => Ok(Self::Big(s.to_string())),
            s => Ok(Self::Small(s.to_string())),
        }
    }
}

#[derive(Debug, Clone)]
struct CaveSystem {
    caves: Vec<Node>,
    connections: Vec<Vec<bool>>,
}
impl FromStr for CaveSystem {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nodes: HashSet<Node> = HashSet::new();
        for line in s.lines().filter(|line| !line.is_empty()) {
            let mut line = line.split('-');
            let first = line.next().unwrap().parse().unwrap();
            let second = line.next().unwrap().parse().unwrap();
            nodes.insert(first);
            nodes.insert(second);
        }

        let mut nodes: Vec<Node> = nodes.into_iter().collect();
        let start = nodes
            .iter()
            .position(|node| matches!(node, Node::Start))
            .ok_or_else(|| String::from("No start node"))?;
        nodes.swap(start, 0);
        let end = nodes
            .iter()
            .position(|node| matches!(node, Node::End))
            .ok_or_else(|| String::from("No end node"))?;
        let length = nodes.len();
        nodes.swap(end, length - 1);

        let mappings: HashMap<&Node, usize> = nodes.iter().zip(0..).collect();

        let mut connections = vec![vec![false; nodes.len()]; nodes.len()];
        for line in s.lines().filter(|line| !line.is_empty()) {
            let mut line = line.split('-');
            let first: Node = line.next().unwrap().parse().unwrap();
            let second: Node = line.next().unwrap().parse().unwrap();
            let first = mappings[&first];
            let second = mappings[&second];

            connections[first][second] = true;
            connections[second][first] = true;
        }

        Ok(Self {
            caves: nodes,
            connections,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{first, second, CaveSystem};

    fn load_test_data_1() -> CaveSystem {
        include_str!("test1.txt").parse().unwrap()
    }
    fn load_test_data_2() -> CaveSystem {
        include_str!("test2.txt").parse().unwrap()
    }
    fn load_test_data_3() -> CaveSystem {
        include_str!("test3.txt").parse().unwrap()
    }

    #[test]
    fn day12_first() {
        let input = load_test_data_1();
        assert_eq!(first(&input), 10);
        let input = load_test_data_2();
        assert_eq!(first(&input), 19);
        let input = load_test_data_3();
        assert_eq!(first(&input), 226);
    }

    #[test]
    fn day12_second() {
        let input = load_test_data_1();
        assert_eq!(second(&input), 36);
        let input = load_test_data_2();
        assert_eq!(second(&input), 103);
        let input = load_test_data_3();
        assert_eq!(second(&input), 3509);
    }
}
