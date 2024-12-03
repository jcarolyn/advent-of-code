use std::fs;
use std::io::{self, BufRead};

const FILE_PATH: &str = "input.txt";
const MIN_LEVEL_DIFFERENCE: i32 = 1;
const MAX_LEVEL_DIFFERENCE: i32 = 3;

fn analyze_reports(file_path: &str) -> Result<i32, io::Error> {
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut total_safe_reports = 0;

    for line in reader.lines() {
        let line = line?;

        let levels: Vec<i32> = line.split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        let mut increasing: bool = false;
        let mut decreasing: bool = false;
        let mut is_safe: bool = true;

        // Windows allows you to create an iterator over a specific sized slice of vector
        // in this case, we create a slice of size 2 so we can compare adjacent numbers
        for window in levels.windows(2) {
            let diff = window[0] - window[1];

            if diff.abs() < MIN_LEVEL_DIFFERENCE || diff.abs() > MAX_LEVEL_DIFFERENCE {
                is_safe = false;
                break;
            }

            if diff > 0 {
                increasing = true;
            } else if diff < 0 {
                decreasing = true;
            }
        }

        // Check if we are strictly increasing or strictly decreasing
        if is_safe && !(increasing && decreasing) {
            total_safe_reports += 1;
        }
    }

    Ok(total_safe_reports)
}

fn main() {
    match analyze_reports(FILE_PATH) {
        Ok(total_safe_reports) => println!("Total safe reports: {}", total_safe_reports),
        Err(e) => eprintln!("Error: {}", e),
    }
}
