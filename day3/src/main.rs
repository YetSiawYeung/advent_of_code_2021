fn main() {
    let input = include_str!("input.txt");
    let input: Vec<&str> = input.lines().filter(|line| !line.is_empty()).collect();

    println!("{}", first(&input));
    println!("{}", second(&input));
}

fn first(input: &[&str]) -> i32 {
    let most_common_bits = (0..input[0].len())
        .map(|i| {
            let one_count = input.iter().filter(|num| num.as_bytes()[i] == b'1').count();
            one_count * 2 > input.len()
        })
        .collect::<Vec<_>>();

    let gamma = calc_gamma(&most_common_bits);
    let epsilon = calc_epsilon(&most_common_bits);

    gamma * epsilon
}

fn calc_gamma(nums: &[bool]) -> i32 {
    nums.iter()
        .fold(0, |acc, byte| (acc << 1) + if *byte { 1 } else { 0 })
}

fn calc_epsilon(nums: &[bool]) -> i32 {
    nums.iter()
        .fold(0, |acc, byte| (acc << 1) + if !*byte { 1 } else { 0 })
}

fn second(input: &[&str]) -> i32 {
    let oxygen_generator = calc_oxygen(input);
    let co2_scrubber = calc_co2(input);

    oxygen_generator * co2_scrubber
}

fn binary_str_to_int(num: &str) -> i32 {
    num.bytes().fold(0, |acc, byte| {
        (acc << 1)
            + match byte {
                b'0' => 0,
                b'1' => 1,
                _ => unreachable!(),
            }
    })
}

fn calc_oxygen(nums: &[&str]) -> i32 {
    let mut nums = nums.to_vec();
    let mut i = 0;

    while nums.len() > 1 {
        let ones_count = nums.iter().filter(|num| num.as_bytes()[i] == b'1').count();
        if ones_count * 2 >= nums.len() {
            nums.retain(|num| num.as_bytes()[i] == b'1');
        } else {
            nums.retain(|num| num.as_bytes()[i] == b'0');
        }

        i += 1;
    }

    binary_str_to_int(nums[0])
}

fn calc_co2(nums: &[&str]) -> i32 {
    let mut nums = nums.to_vec();
    let mut i = 0;

    while nums.len() > 1 {
        let ones_count = nums.iter().filter(|num| num.as_bytes()[i] == b'1').count();
        if ones_count * 2 >= nums.len() {
            nums.retain(|num| num.as_bytes()[i] == b'0');
        } else {
            nums.retain(|num| num.as_bytes()[i] == b'1');
        }

        i += 1;
    }

    binary_str_to_int(nums[0])
}

#[test]
fn day3_first() {
    let input = vec![
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ];
    assert_eq!(first(&input), 198);
}

#[test]
fn day3_second() {
    let input = vec![
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ];
    assert_eq!(second(&input), 230);
}
