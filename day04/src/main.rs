use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    str::FromStr,
};

#[derive(Copy, Clone)]
struct SectionRange {
    first: u32,
    last: u32,
}

impl SectionRange {
    fn contains(&self, section: &SectionRange) -> bool {
        self.first <= section.first && self.last >= section.last
    }

    fn contains_section(&self, section: u32) -> bool {
        self.first <= section && self.last >= section
    }
}

impl FromStr for SectionRange {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match &input
            .split('-')
            .map(|i| i.parse::<u32>().unwrap())
            .collect::<Vec<_>>()[..]
        {
            &[first, last] => Ok(SectionRange { first, last }),
            _ => Err(()),
        }
    }
}

const INPUT_FILE: &str = "input.txt";

fn main() -> io::Result<()> {
    let mut counter = 0;

    for line in input_lines() {
        let line = line?;
        let (elf1, elf2) = match &(line
            .split(',')
            .map(|s| SectionRange::from_str(s).unwrap())
            .collect::<Vec<_>>())[..]
        {
            &[first, second] => (first, second),
            _ => unreachable!(),
        };

        if elf1.contains(&elf2) || elf2.contains(&elf1) {
            counter += 1;
        }
    }

    println!("counter: {counter}");
    Ok(())
}

fn input_lines() -> io::Lines<BufReader<File>> {
    let file = File::open(INPUT_FILE).unwrap();
    BufReader::new(file).lines()
}
