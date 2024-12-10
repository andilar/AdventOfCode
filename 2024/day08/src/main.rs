use std::collections::HashSet;
use std::fs::File;
use std::io::{ self, BufRead };
use std::path::Path;

fn parse_input(filename: &str) -> io::Result<Vec<(usize, char)>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let mut antennas = Vec::new();

    for line in io::BufReader::new(file).lines() {
        let line = line?;
        for (x, c) in line.chars().enumerate() {
            if c.is_alphanumeric() {
                antennas.push((x, c));
            }
        }
    }

    Ok(antennas)
}

fn find_antinodes(antennas: &[(usize, char)]) -> HashSet<usize> {
    let mut antinodes = HashSet::new();

    for i in 0..antennas.len() {
        for j in i + 1..antennas.len() {
            let (pos1, freq1) = antennas[i];
            let (pos2, freq2) = antennas[j];

            if freq1 == freq2 {
                let distance = ((pos2 as isize) - (pos1 as isize)).abs() as usize;
                if distance % 2 == 0 {
                    let mid = (pos1 + pos2) / 2;
                    antinodes.insert(mid);
                    let antinode1 = (pos1 as isize) - (distance as isize) / 2;
                    let antinode2 = (pos2 as isize) + (distance as isize) / 2;
                    if antinode1 >= 0 {
                        antinodes.insert(antinode1 as usize);
                    }
                    if antinode2 < (antennas.len() as isize) {
                        antinodes.insert(antinode2 as usize);
                    }
                }
            }
        }
    }

    antinodes
}

fn main() -> io::Result<()> {
    let antennas = parse_input("input.txt")?;
    let antinodes = find_antinodes(&antennas);
    println!("Number of unique antinode locations: {}", antinodes.len());
    Ok(())
}
