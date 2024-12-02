use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    File::open(filename).map(|file| io::BufReader::new(file).lines())
}