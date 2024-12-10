use std::fs::File;
use std::io::{ self, BufRead };
use std::path::Path;

fn parse_input(filename: &str) -> io::Result<String> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let mut disk_map = String::new();

    for line in io::BufReader::new(file).lines() {
        disk_map.push_str(&line?);
    }

    Ok(disk_map)
}

fn compact_disk(disk_map: &str) -> Vec<char> {
    let mut blocks: Vec<char> = Vec::new();
    let mut file_id = 0;

    let mut chars = disk_map.chars();
    while let Some(file_len_char) = chars.next() {
        if let Some(file_len) = file_len_char.to_digit(10) {
            for _ in 0..file_len {
                blocks.push(std::char::from_digit(file_id, 10).unwrap());
            }
            file_id += 1;

            if let Some(free_len_char) = chars.next() {
                if let Some(free_len) = free_len_char.to_digit(10) {
                    for _ in 0..free_len {
                        blocks.push('.');
                    }
                }
            }
        }
    }

    let mut compacted_blocks: Vec<char> = Vec::new();
    for block in blocks.iter() {
        if *block != '.' {
            compacted_blocks.push(*block);
        }
    }

    let mut result_blocks: Vec<char> = Vec::new();
    let mut compacted_iter = compacted_blocks.iter();
    for block in blocks.iter() {
        if *block == '.' {
            if let Some(&next_block) = compacted_iter.next() {
                result_blocks.push(next_block);
            } else {
                result_blocks.push('.');
            }
        } else {
            result_blocks.push(*block);
        }
    }

    result_blocks
}

fn calculate_checksum(blocks: &[char]) -> u64 {
    let mut checksum = 0;
    for (pos, &block) in blocks.iter().enumerate() {
        if block != '.' {
            if let Some(file_id) = block.to_digit(10) {
                checksum += (pos as u64) * (file_id as u64);
            }
        }
    }
    checksum
}

fn main() -> io::Result<()> {
    let disk_map = parse_input("input.txt")?;
    let compacted_blocks = compact_disk(&disk_map);
    let checksum = calculate_checksum(&compacted_blocks);
    println!("Filesystem checksum: {}", checksum);
    Ok(())
}
