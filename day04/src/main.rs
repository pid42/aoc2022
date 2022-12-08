use std::{io, str::FromStr};

use util::input_lines;

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

    fn overlaps(&self, section: &SectionRange) -> bool {
        section.first <= self.last
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

    for line in input_lines(INPUT_FILE) {
        let line = line?;
        let mut parsed_line = line
            .split(',')
            .map(|s| SectionRange::from_str(s).unwrap())
            .collect::<Vec<_>>();
        parsed_line.sort_unstable_by(|a, b| a.first.cmp(&b.first));

        let (elf1, elf2) = match &(parsed_line)[..] {
            &[first, second] => (first, second),
            _ => unreachable!(),
        };

        if elf1.overlaps(&elf2) {
            counter += 1;
        }
    }

    println!("counter: {counter}");
    Ok(())
}
