use std::collections::{ HashSet, VecDeque };
use std::fs::File;
use std::io::{ self, BufRead };
use std::path::Path;

/// Reads the input file and returns the topographic map as a 2D vector of u8 values.
fn parse_input(filename: &str) -> io::Result<Vec<Vec<u8>>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let mut map = Vec::new();

    for line in io::BufReader::new(file).lines() {
        let line = line?;
        let row: Vec<u8> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        map.push(row);
    }

    Ok(map)
}

/// Identifies all trailheads (positions with height 0) on the topographic map.
fn find_trailheads(map: &[Vec<u8>]) -> Vec<(usize, usize)> {
    let mut trailheads = Vec::new();
    for (i, row) in map.iter().enumerate() {
        for (j, &height) in row.iter().enumerate() {
            if height == 0 {
                trailheads.push((i, j));
            }
        }
    }
    trailheads
}

/// Performs a breadth-first search (BFS) to find all distinct hiking trails from a given trailhead.
fn bfs(map: &[Vec<u8>], start: (usize, usize)) -> u64 {
    let directions = [
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
    ];
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut trail_count = 0;

    queue.push_back((start, 0));
    visited.insert(start);

    while let Some(((x, y), height)) = queue.pop_front() {
        if height == 9 {
            trail_count += 1;
            continue;
        }

        for &(dx, dy) in &directions {
            let nx = (x as isize) + dx;
            let ny = (y as isize) + dy;
            if nx >= 0 && ny >= 0 {
                let nx = nx as usize;
                let ny = ny as usize;
                if nx < map.len() && ny < map[0].len() {
                    let next_height = map[nx][ny];
                    if next_height == height + 1 && !visited.contains(&(nx, ny)) {
                        queue.push_back(((nx, ny), next_height));
                        visited.insert((nx, ny));
                    }
                }
            }
        }
    }

    trail_count
}

/// Calculates the rating for each trailhead by counting the number of distinct hiking trails that begin at that trailhead.
fn calculate_ratings(map: &[Vec<u8>], trailheads: &[(usize, usize)]) -> u64 {
    let mut total_rating = 0;

    for &trailhead in trailheads {
        let trail_count = bfs(map, trailhead);
        total_rating += trail_count;
    }

    total_rating
}

fn main() -> io::Result<()> {
    // Parse the input file to get the topographic map.
    let map = parse_input("input.txt")?;

    // Find all trailheads on the map.
    let trailheads = find_trailheads(&map);

    // Calculate the total rating of all trailheads.
    let total_rating = calculate_ratings(&map, &trailheads);

    // Print the resulting sum of the ratings.
    println!("Sum of the ratings of all trailheads: {}", total_rating);

    Ok(())
}
