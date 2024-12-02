use std::fs;
use std::io::{self, BufRead};

fn calculate_total_sum(file_path: &str) -> Result<i32, io::Error> {
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut left_numbers: Vec<i32> = Vec::new();
    let mut right_numbers: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        // Split each line into two numbers via whitespace
        let numbers: Vec<i32> = line.split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        left_numbers.push(numbers[0]);
        right_numbers.push(numbers[1]);
    }

    // Sorting, since we need to add up the two smallest numbers
    left_numbers.sort();
    right_numbers.sort();

    // Zip iterates through both vectors at the same time
    let total_sum: i32 = left_numbers.iter().zip(right_numbers.iter())
        // We then find the absolute difference of the two numbers on the line
        .map(|(a, b)| (a - b).abs())
        // Then sum each of those line totals together
        .sum();

    Ok(total_sum)
}

fn main() {
    let file_path = "input.txt";

    match calculate_total_sum(file_path) {
        Ok(total_sum) => println!("Total sum: {}", total_sum),
        Err(e) => eprintln!("Error: {}", e),
    }
}
