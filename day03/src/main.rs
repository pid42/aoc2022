use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader},
};

const INPUT_FILE: &str = "input.txt";

fn main() {
    let mut points = 0;

    for line in input_lines() {
        let line = line.unwrap();
        let (compartment1, compartment2) = line.split_at(line.len() / 2);
        // println!("compartment1: {compartment1} ==== compartment2: {compartment2}");

        let mut set = HashSet::new();
        let mut dups = HashSet::new();

        for char in compartment1.chars() {
            set.insert(char);
        }
        for char in compartment2.chars() {
            if set.contains(&char) {
                dups.insert(char);
            }
        }

        for char in dups {
            points += char_points(char);
        }
    }

    println!("points: {points}");
}

fn char_points(c: char) -> u32 {
    if c.is_lowercase() {
        (c as u32) - 96
    } else {
        (c as u32) - 38
    }
}

fn input_lines() -> io::Lines<BufReader<File>> {
    let file = File::open(INPUT_FILE).unwrap();
    BufReader::new(file).lines()
}
