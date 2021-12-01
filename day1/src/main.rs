fn main() {
    let input = include_str!("input.txt");
    let input = input
        .lines()
        .enumerate()
        .filter(|(_linenum, line)| !line.is_empty())
        .map(|(linenum, line)| match line.parse::<i32>() {
            Ok(num) => num,
            Err(err) => panic!("Got {:?} error at line {} of input.txt", err, linenum),
        })
        .collect::<Vec<i32>>();

    println!("{}", first(&input));
    println!("{}", second(&input));
}

fn first(input: &[i32]) -> usize {
    input
        .windows(2)
        .filter(|measurements| measurements[1] > measurements[0])
        .count()
}

fn second(input: &[i32]) -> usize {
    input
        .windows(4)
        .filter(|measurements| measurements[3] > measurements[0]) // (B+C+D)-(A+B+C) = D-A
        .count()
}

#[test]
fn day1_first() {
    let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    assert_eq!(first(&input), 7);
}

#[test]
fn day1_second() {
    let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    assert_eq!(second(&input), 5);
}
