use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader},
};

const INPUT_FILE: &str = "input.txt";

fn main() {
    let mut points = 0;
    let mut elf_group: Vec<HashSet<char>> = Vec::with_capacity(3);

    for line in input_lines() {
        let line = line.unwrap();
        elf_group.push(line.chars().collect());

        if elf_group.len() == 3 {
            let tmp: HashSet<&char> = elf_group[0].intersection(&elf_group[1]).collect();
            let mut result = elf_group[2].clone();
            result.retain(|i| tmp.contains(i));
            // println!("result: {:?}", result);
            assert!(result.len() == 1);
            points += char_points(*result.iter().next().unwrap());

            elf_group.clear();
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
