use std::collections::HashMap;
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
    
    // Key: right number, Value: how many times we found it in the right list
    let mut instances_count: HashMap<i32, i32> = HashMap::new();

    for &num in &right_numbers {
        *instances_count.entry(num).or_insert(0) += 1;
    }

    let total_sum: i32 = left_numbers.iter()
        // We multiply each number on the left by how many times we found it in the right, & sum those values
        .map(|&n| n * instances_count.get(&n).unwrap_or(&0))
        .sum();

    Ok(total_sum)
}

fn main() {
    let file_path = "src/input.txt";

    match calculate_total_sum(file_path) {
        Ok(total_sum) => println!("Total sum: {}", total_sum),
        Err(e) => eprintln!("Error: {}", e),
    }
}
