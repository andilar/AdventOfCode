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
            let file_id_char = std::char::from_digit(file_id % 10, 10).unwrap();

            for _ in 0..file_len {
                blocks.push(file_id_char);
            }
            file_id += 1;

            if let Some(free_len_char) = chars.next() {
                if let Some(free_len) = free_len_char.to_digit(10) {
                    for _ in 0..free_len {
                        blocks.push('.');
                    }
                } else {
                    eprintln!("Unexpected character for free space length: {}", free_len_char);
                }
            } else {
                eprintln!("Missing free space length after file length: {}", file_len_char);
            }
        } else {
            eprintln!("Unexpected character for file length: {}", file_len_char);
        }
    }

    // Compact the disk by moving file blocks to the leftmost free space
    let mut compacted_blocks: Vec<char> = Vec::new();
    for block in blocks.iter() {
        if *block != '.' {
            compacted_blocks.push(*block);
        }
    }

    // Fill the remaining space with '.'
    while compacted_blocks.len() < blocks.len() {
        compacted_blocks.push('.');
    }

    compacted_blocks
}

fn calculate_checksum(blocks: &[char]) -> u64 {
    let mut checksum = 0;
    for (pos, &block) in blocks.iter().enumerate() {
        if block != '.' {
            if let Some(file_id) = block.to_digit(10) {
                checksum += (pos as u64) * (file_id as u64);
            } else {
                eprintln!("Unexpected character in block: {}", block);
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
