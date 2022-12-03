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

impl FromStr for MatchResult {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "X" => Ok(MatchResult::Loose),
            "Y" => Ok(MatchResult::Draw),
            "Z" => Ok(MatchResult::Win),
            _ => Err(()),
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
            "A" => Ok(Shape::Rock),
            "B" => Ok(Shape::Paper),
            "C" => Ok(Shape::Scissors),
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

    // fn play(&self, other: &Shape) -> MatchResult {
    //     match self {
    //         Shape::Rock => match other {
    //             Shape::Rock => MatchResult::Draw,
    //             Shape::Paper => MatchResult::Loose,
    //             Shape::Scissors => MatchResult::Win,
    //         },
    //         Shape::Paper => match other {
    //             Shape::Rock => MatchResult::Win,
    //             Shape::Paper => MatchResult::Draw,
    //             Shape::Scissors => MatchResult::Loose,
    //         },
    //         Shape::Scissors => match other {
    //             Shape::Rock => MatchResult::Loose,
    //             Shape::Paper => MatchResult::Win,
    //             Shape::Scissors => MatchResult::Draw,
    //         },
    //     }
    // }
}

fn my_move(opponent_shape: &Shape, wanted_result: &MatchResult) -> Shape {
    match opponent_shape {
        Shape::Rock => match wanted_result {
            MatchResult::Win => Shape::Paper,
            MatchResult::Draw => Shape::Rock,
            MatchResult::Loose => Shape::Scissors,
        },
        Shape::Paper => match wanted_result {
            MatchResult::Win => Shape::Scissors,
            MatchResult::Draw => Shape::Paper,
            MatchResult::Loose => Shape::Rock,
        },
        Shape::Scissors => match wanted_result {
            MatchResult::Win => Shape::Rock,
            MatchResult::Draw => Shape::Scissors,
            MatchResult::Loose => Shape::Paper,
        },
    }
}

fn main() -> io::Result<()> {
    let mut points = 0;

    for line in input_lines() {
        let line = line?;
        let line: Vec<&str> = line.split(' ').collect();
        let opponent = Shape::from_str(line[0]).unwrap();
        let wanted_result = MatchResult::from_str(line[1]).unwrap();
        let me = my_move(&opponent, &wanted_result);

        println!(
            "line: {:?}({}) vs {:?}({}) => {:?}({})",
            &opponent,
            &opponent.points(),
            &me,
            &me.points(),
            &wanted_result,
            &wanted_result.points()
        );

        points += me.points() + wanted_result.points();
    }
    println!("points: {points}");

    Ok(())
}

fn input_lines() -> io::Lines<BufReader<File>> {
    let file = File::open(INPUT_FILE).unwrap();
    BufReader::new(file).lines()
}
