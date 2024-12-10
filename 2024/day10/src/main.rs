use std::collections::{ HashSet, VecDeque };
use std::fs::File;
use std::io::{ self, BufRead };
use std::path::Path;

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

fn bfs(map: &[Vec<u8>], start: (usize, usize)) -> HashSet<(usize, usize)> {
    let directions = [
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
    ];
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut reachable_nines = HashSet::new();

    queue.push_back((start, 0));
    visited.insert(start);

    while let Some(((x, y), height)) = queue.pop_front() {
        if height == 9 {
            reachable_nines.insert((x, y));
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

    reachable_nines
}

fn calculate_scores(map: &[Vec<u8>], trailheads: &[(usize, usize)]) -> u64 {
    let mut total_score = 0;

    for &trailhead in trailheads {
        let reachable_nines = bfs(map, trailhead);
        total_score += reachable_nines.len() as u64;
    }

    total_score
}

fn main() -> io::Result<()> {
    let map = parse_input("input.txt")?;
    let trailheads = find_trailheads(&map);
    let total_score = calculate_scores(&map, &trailheads);
    println!("Sum of the scores of all trailheads: {}", total_score);
    Ok(())
}
