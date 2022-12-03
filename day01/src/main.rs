use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut calories = 0;
    let mut max_calories = 0;

    let input_file = "input.txt";
    let mut elves = Vec::new();
    let mut file = File::open(input_file)?;

    for line in BufReader::new(file).lines() {
        let line = line?;
        let v = line.trim();

        if !v.is_empty() {
            let n: u32 = v.parse()?;
            calories += n;
        } else {
            println!("calories: {calories}");
            elves.push(calories);
            if calories > max_calories {
                max_calories = calories;
            }
            calories = 0;
        }
    }

    println!("max_calories: {max_calories}");
    Ok(())
}
