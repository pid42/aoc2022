use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    str::FromStr,
};

const INPUT_FILE: &str = "input.txt";

// const WIN: u32 = 6;
// const DRAW: u32 = 3;
// const LOSE: u32 = 0;

#[derive(Debug)]
enum MatchResult {
    Loose = 0,
    Draw = 3,
    Win = 6,
}

impl MatchResult {
    fn points(&self) -> u32 {
        match self {
            MatchResult::Win => 6,
            MatchResult::Draw => 3,
            MatchResult::Loose => 0,
        }
    }
}

#[derive(Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Shape {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "A" | "X" => Ok(Shape::Rock),
            "B" | "Y" => Ok(Shape::Paper),
            "C" | "Z" => Ok(Shape::Scissors),
            _ => Err(()),
        }
    }
}

impl Shape {
    fn points(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn play(&self, other: &Shape) -> MatchResult {
        match self {
            Shape::Rock => match other {
                Shape::Rock => MatchResult::Draw,
                Shape::Paper => MatchResult::Loose,
                Shape::Scissors => MatchResult::Win,
            },
            Shape::Paper => match other {
                Shape::Rock => MatchResult::Win,
                Shape::Paper => MatchResult::Draw,
                Shape::Scissors => MatchResult::Loose,
            },
            Shape::Scissors => match other {
                Shape::Rock => MatchResult::Loose,
                Shape::Paper => MatchResult::Win,
                Shape::Scissors => MatchResult::Draw,
            },
        }
    }
}

fn main() -> io::Result<()> {
    let mut points = 0;

    for line in input_lines() {
        let line = line?;
        let line: Vec<&str> = line.split(' ').collect();
        let opponent = Shape::from_str(line[0]).unwrap();
        let me = Shape::from_str(line[1]).unwrap();
        let result = me.play(&opponent);

        println!(
            "line: {:?}({}) vs {:?}({}) => {:?}({})",
            &opponent,
            &opponent.points(),
            &me,
            &me.points(),
            &result,
            &result.points()
        );

        points += me.points() + result.points();
    }
    println!("points: {points}");

    Ok(())
}

fn input_lines() -> io::Lines<BufReader<File>> {
    let file = File::open(INPUT_FILE).unwrap();
    BufReader::new(file).lines()
}
