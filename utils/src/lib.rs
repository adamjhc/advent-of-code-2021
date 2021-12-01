use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader, Lines};

pub fn read_lines(filename: &str) -> Lines<BufReader<File>> {
    let file = File::open(filename).unwrap();
    BufReader::new(file).lines()
}

pub fn read_string(filename: &str) -> String {
    read_to_string(filename).unwrap()
}
