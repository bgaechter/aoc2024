use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    File::open(filename).map(|file| io::BufReader::new(file).lines())
}

fn parse_line(line: String) -> Option<(i64, i64)> {
    let numbers: Vec<i64> = line
        .split_whitespace()
        .filter_map(|num| num.parse::<i64>().ok())
        .collect();
    
    match numbers.as_slice() {
        [a, b, ..] => Some((*a, *b)),
        _ => None
    }
}

fn calculate_differences(pairs: &[(i64, i64)]) -> i64 {
    pairs.iter()
        .enumerate()
        .map(|(i, (a, b))| {
            let difference = (a - b).abs();
            println!("Position {}: {} - {} = {}", i, b, a, difference);
            difference
        })
        .sum()
}

fn calculate_similarity_score(list1: &[i64], frequency_map: &HashMap<i64, i64>) -> i64 {
    list1.iter()
        .map(|&num| {
            let frequency = frequency_map.get(&num).unwrap_or(&0);
            let score = num * frequency;
            println!("Number {} appears {} times, score: {}", num, frequency, score);
            score
        })
        .sum()
}

fn create_frequency_map(numbers: &[i64]) -> HashMap<i64, i64> {
    numbers.iter()
        .fold(HashMap::new(), |mut map, &num| {
            *map.entry(num).or_insert(0) += 1;
            map
        })
}

fn main() {
    let number_pairs: Vec<(i64, i64)> = read_lines("input_01.txt")
        .into_iter()
        .flatten()
        .filter_map(|line| line.ok())
        .filter_map(parse_line)
        .collect();

    let (list1, list2): (Vec<_>, Vec<_>) = number_pairs.into_iter().unzip();

    // Part One: Create pairs vector first, then pass reference to it
    let pairs: Vec<(i64, i64)> = list1.iter()
        .zip(list2.iter())
        .map(|(&a, &b)| (a, b))
        .collect();
    let result = calculate_differences(&pairs);
    println!("The difference is: {}", result);

    // Part Two
    let frequency_map = create_frequency_map(&list2);
    let total_score = calculate_similarity_score(&list1, &frequency_map);
    println!("Total similarity score: {}", total_score);
}
