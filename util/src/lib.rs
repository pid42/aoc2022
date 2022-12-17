use std::{
    fmt::Display,
    fs::File,
    io::{self, BufRead, BufReader},
};

pub fn input_lines(input_file: &str) -> io::Lines<BufReader<File>> {
    let file = File::open(input_file).unwrap();
    BufReader::new(file).lines()
}

#[derive(Debug)]
pub struct MyError(pub String);

impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for MyError {}

impl std::convert::From<String> for MyError {
    fn from(msg: String) -> Self {
        Self(msg)
    }
}
