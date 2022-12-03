use std::{
    collections::BinaryHeap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut calories = 0;

    let input_file = "input.txt";
    let mut elves = BinaryHeap::new();
    let file = File::open(input_file)?;

    for line in BufReader::new(file).lines() {
        let line = line?;
        let v = line.trim();

        if !v.is_empty() {
            let n: u32 = v.parse()?;
            calories += n;
        } else {
            // println!("calories: {calories}");
            elves.push(calories);
            calories = 0;
        }
    }

    let mut max = 0;
    for i in (1..=3) {
        let v = elves.pop().unwrap();
        println!("{i} = {v}");
        max += v
    }
    println!("max: {max}");

    Ok(())
}
