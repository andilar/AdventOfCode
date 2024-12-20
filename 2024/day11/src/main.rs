use rayon::prelude::*;
use std::fs::File;
use std::io::{ self, BufRead };
use std::path::Path;

fn split_number(n: u64) -> (u64, u64) {
    let mut num = n;
    let mut digits = 0;
    while num > 0 {
        num /= 10;
        digits += 1;
    }

    let mid = digits / 2;
    let divisor = 10u64.pow(mid as u32);
    let left = n / divisor;
    let right = n % divisor;
    (left, right)
}

fn process_stone(stone: i64) -> Vec<i64> {
    if stone == 0 {
        vec![1]
    } else if stone.to_string().len() % 2 == 0 {
        let (left, right) = split_number(stone);
        vec![left, right]
    } else {
        vec![stone * 2 * 2 * 506]
    }
}

fn blink(stones: Vec<u64>) -> Vec<u64> {
    stones
        .par_iter()
        .flat_map(|&stone| process_stone(stone))
        .collect()
}

fn simulate_blinks(initial_stones: Vec<u64>, blinks: usize) -> Vec<u64> {
    let mut stones = initial_stones;
    for _ in 0..blinks {
        stones = blink(stones);
    }
    stones
}

fn read_input_file(filename: &str) -> io::Result<Vec<u64>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    let mut stones = Vec::new();
    for line in reader.lines() {
        let line = line?;
        for num in line.split_whitespace() {
            stones.push(num.parse::<u64>().unwrap());
        }
    }
    Ok(stones)
}

fn main() -> io::Result<()> {
    let initial_stones = read_input_file("input.txt")?;
    let blinks = 75;
    let start = std::time::Instant::now();
    let final_stones = simulate_blinks(initial_stones, blinks);
    let duration = start.elapsed();
    println!("Number of stones after {} blinks: {}", blinks, final_stones.len());
    println!("Time taken: {:?}", duration);
    Ok(())
}