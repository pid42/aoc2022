use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

pub fn input_lines(input_file: &str) -> io::Lines<BufReader<File>> {
    let file = File::open(input_file).unwrap();
    BufReader::new(file).lines()
}
