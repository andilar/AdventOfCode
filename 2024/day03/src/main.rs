use regex::Regex;
use std::fs;

fn extract_and_sum_mul_instructions(data: &str) -> i32 {
    // Regular expressions to match valid mul(X,Y), do(), and don't() instructions
    let mul_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let do_re = Regex::new(r"do\(\)").unwrap();
    let dont_re = Regex::new(r"don't\(\)").unwrap();

    // Initialize the sum of results and the state of mul instructions
    let mut total_sum = 0;
    let mut mul_enabled = true;

    // Iterate over the data string
    let mut pos = 0;
    while pos < data.len() {
        if let Some(do_mat) = do_re.find(&data[pos..]) {
            if do_mat.start() == 0 {
                mul_enabled = true;
                println!("do() found, enabling mul"); // Debug print
                pos += do_mat.end();
                continue;
            }
        }

        if let Some(dont_mat) = dont_re.find(&data[pos..]) {
            if dont_mat.start() == 0 {
                mul_enabled = false;
                println!("don't() found, disabling mul"); // Debug print
                pos += dont_mat.end();
                continue;
            }
        }

        if let Some(mul_mat) = mul_re.find(&data[pos..]) {
            if mul_mat.start() == 0 {
                if mul_enabled {
                    let cap = mul_re.captures(&data[pos..pos + mul_mat.end()]).unwrap();
                    let x: i32 = cap[1].parse().unwrap();
                    let y: i32 = cap[2].parse().unwrap();
                    total_sum += x * y;
                    println!("mul({}, {}) = {}", x, y, x * y); // Debug print
                }
                pos += mul_mat.end();
                continue;
            }
        }

        pos += 1;
    }

    total_sum
}

fn main() {
    // Read the input from the file
    let data = fs::read_to_string("input.txt").expect("Failed to read input file");

    // Calculate the sum of valid mul instructions
    let result = extract_and_sum_mul_instructions(&data);
    println!("Total sum: {}", result);
}
