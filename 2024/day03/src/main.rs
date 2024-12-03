use regex::Regex;
use std::fs;

fn extract_and_sum_mul_instructions(data: &str) -> i32 {
    // Regular expression to match valid mul(X,Y) instructions
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    
    // Initialize the sum of results
    let mut total_sum = 0;
    
    // Iterate over all matches and calculate the sum of the products
    for cap in re.captures_iter(data) {
        let x: i32 = cap[1].parse().unwrap();
        let y: i32 = cap[2].parse().unwrap();
        total_sum += x * y;
    }
    
    total_sum
}

fn main() {
    // Read the input from the file
    let data = fs::read_to_string("input.txt")
        .expect("Failed to read input file");

    // Calculate the sum of valid mul instructions
    let result = extract_and_sum_mul_instructions(&data);
    println!("{}", result);
}