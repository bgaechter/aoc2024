use common::read_lines;

fn parse_line(line: String) -> Option<Vec<i64>> {
    let numbers: Vec<i64> = line
        .split_whitespace()
        .filter_map(|num| num.parse::<i64>().ok())
        .collect();

    Some(numbers)
}

fn main() {
    let numbers: Vec<Vec<i64>> = read_lines("/home/benny/git/aoc2024/day02/input02.txt")
        .into_iter()
        .flatten()
        .filter_map(|line| line.ok())
        .filter_map(parse_line)
        .collect();

    let result = numbers
        .iter()
        .filter(|seq| {
            seq.windows(2).all(|w| {
                let diff = w[0] - w[1];
                diff >= 1 && diff <= 3
            }) || seq.windows(2).all(|w| {
                let diff = w[1] - w[0];
                diff >= 1 && diff <= 3
            })
        })
        .count();
    println!("result 01: {:#?}", result)
}
