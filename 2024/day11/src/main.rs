use rayon::prelude::*;
use std::fs::File;
use std::io::{ self, BufRead };
use std::path::Path;

fn split_number(n: u64) -> (u64, u64) {
    let s = n.to_string();
    let mid = s.len() / 2;
    let left = s[..mid].parse::<u64>().unwrap();
    let right = s[mid..].parse::<u64>().unwrap();
    (left, right)
}

fn process_stone(stone: u64) -> Vec<u64> {
    if stone == 0 {
        vec![1]
    } else if stone.to_string().len() % 2 == 0 {
        let (left, right) = split_number(stone);
        vec![left, right]
    } else {
        vec![stone * 2 * 2 * 506]
    }
}

fn blink(stones: Vec<u64>, batch_size: usize) -> Vec<u64> {
    stones
        .par_chunks(batch_size)
        .flat_map(|batch| {
            batch
                .iter()
                .flat_map(|&stone| process_stone(stone))
                .collect::<Vec<u64>>()
        })
        .collect()
}

fn simulate_blinks(initial_stones: Vec<u64>, blinks: usize, batch_size: usize) -> Vec<u64> {
    let mut stones = initial_stones;
    for _ in 0..blinks {
        stones = blink(stones, batch_size);
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
    let batch_size = 10; // Adjust the batch size 
    let final_stones = simulate_blinks(initial_stones, blinks, batch_size);
    println!("Number of stones after {} blinks: {}", blinks, final_stones.len());
    Ok(())
}
