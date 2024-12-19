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

fn blink(stones: Vec<u64>) -> Vec<u64> {
    let mut new_stones = Vec::new();
    for stone in stones {
        if stone == 0 {
            new_stones.push(1);
        } else if stone.to_string().len() % 2 == 0 {
            let (left, right) = split_number(stone);
            new_stones.push(left);
            new_stones.push(right);
        } else {
            new_stones.push(stone * 2024);
        }
    }
    new_stones
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
    let final_stones = simulate_blinks(initial_stones, blinks);
    println!("Number of stones after {} blinks: {}", blinks, final_stones.len());
    Ok(())
}
