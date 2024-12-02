use common::read_lines;

fn parse_line(line: String) -> Option<Vec<i64>> {
    let numbers: Vec<i64> = line
        .split_whitespace()
        .filter_map(|num| num.parse::<i64>().ok())
        .collect();

    Some(numbers)
}

fn is_valid_sequence(seq: &[i64]) -> bool {
    seq.windows(2).all(|w| {
        let diff = w[0] - w[1];
        diff >= 1 && diff <= 3
    }) || seq.windows(2).all(|w| {
        let diff = w[1] - w[0];
        diff >= 1 && diff <= 3
    })
}

fn can_be_made_valid(seq: &[i64]) -> bool {
    if is_valid_sequence(seq) {
        return true;
    }

    (0..seq.len()).any(|i| {
        let mut modified_seq: Vec<i64> = seq.to_vec();
        modified_seq.remove(i);
        is_valid_sequence(&modified_seq)
    })
}

fn main() {
    let numbers: Vec<Vec<i64>> = read_lines("/home/benny/git/aoc2024/day02/input02.txt")
        .into_iter()
        .flatten()
        .filter_map(|line| line.ok())
        .filter_map(parse_line)
        .collect();
    //Part One
    let result = numbers
        .iter()
        .filter(|seq| is_valid_sequence(seq))
        .count();
    println!("result 01: {:#?}", result);
    
    // Part Two
    let result2 = numbers.iter().filter(|seq| can_be_made_valid(seq)).count();
    println!("result 02: {:#?}", result2);
}
