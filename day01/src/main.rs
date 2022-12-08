use std::collections::BinaryHeap;

use util::input_lines;

const INPUT_FILE: &str = "input.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut calories = 0;

    let mut elves = BinaryHeap::new();

    for line in input_lines(INPUT_FILE) {
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
