use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn parse_map(map_str: &str) -> (Vec<Vec<char>>, (usize, usize), char) {
    let map_lines: Vec<Vec<char>> = map_str.lines().map(|line| line.chars().collect()).collect();
    let mut guard_pos = (0, 0);
    let mut guard_dir = '^';
    let directions = vec!['^', '>', 'v', '<'];

    for (y, line) in map_lines.iter().enumerate() {
        for (x, &char) in line.iter().enumerate() {
            if directions.contains(&char) {
                guard_pos = (x, y);
                guard_dir = char;
                break;
            }
        }
        if guard_pos != (0, 0) {
            break;
        }
    }

    (map_lines, guard_pos, guard_dir)
}

fn turn_right(direction: char) -> char {
    match direction {
        '^' => '>',
        '>' => 'v',
        'v' => '<',
        '<' => '^',
        _ => direction,
    }
}

fn move_forward(position: (usize, usize), direction: char) -> (usize, usize) {
    match direction {
        '^' => (position.0, position.1.wrapping_sub(1)),
        '>' => (position.0 + 1, position.1),
        'v' => (position.0, position.1 + 1),
        '<' => (position.0.wrapping_sub(1), position.1),
        _ => position,
    }
}

fn is_within_bounds(position: (usize, usize), map_lines: &Vec<Vec<char>>) -> bool {
    let (x, y) = position;
    y < map_lines.len() && x < map_lines[0].len()
}

fn is_obstacle(position: (usize, usize), map_lines: &Vec<Vec<char>>) -> bool {
    let (x, y) = position;
    map_lines[y][x] == '#'
}

fn simulate_guard(map_str: &str) -> HashSet<(usize, usize)> {
    let (map_lines, mut guard_pos, mut guard_dir) = parse_map(map_str);
    let mut visited_positions: HashSet<(usize, usize)> = HashSet::new();
    visited_positions.insert(guard_pos);

    loop {
        let next_pos = move_forward(guard_pos, guard_dir);

        if !is_within_bounds(next_pos, &map_lines) {
            break;
        }

        if is_obstacle(next_pos, &map_lines) {
            guard_dir = turn_right(guard_dir);
        } else {
            guard_pos = next_pos;
            visited_positions.insert(guard_pos);
        }
    }

    visited_positions
}

fn simulate_with_obstruction(map_lines: &Vec<Vec<char>>, start_pos: (usize, usize), start_dir: char, obstruction: (usize, usize)) -> bool {
    let mut guard_pos = start_pos;
    let mut guard_dir = start_dir;
    let mut visited_positions: HashSet<(usize, usize)> = HashSet::new();
    visited_positions.insert(guard_pos);

    loop {
        let next_pos = move_forward(guard_pos, guard_dir);

        if !is_within_bounds(next_pos, &map_lines) {
            return false;
        }

        if next_pos == obstruction || is_obstacle(next_pos, &map_lines) {
            guard_dir = turn_right(guard_dir);
        } else {
            guard_pos = next_pos;
            if visited_positions.contains(&guard_pos) {
                return true;
            }
            visited_positions.insert(guard_pos);
        }
    }
}

fn find_loop_positions(map_str: &str) -> usize {
    let (map_lines, guard_pos, guard_dir) = parse_map(map_str);
    let visited_positions = simulate_guard(map_str);
    let mut loop_positions = 0;

    for &pos in &visited_positions {
        if pos != guard_pos && simulate_with_obstruction(&map_lines, guard_pos, guard_dir, pos) {
            loop_positions += 1;
        }
    }

    loop_positions
}

fn read_input_file(filename: &str) -> io::Result<String> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let mut content = String::new();
    for line in io::BufReader::new(file).lines() {
        content.push_str(&line?);
        content.push('\n');
    }
    Ok(content)
}

fn main() -> io::Result<()> {
    let map_str = read_input_file("input.txt")?;
    let result = find_loop_positions(&map_str);
    println!("Possible positions for obstruction: {}", result);
    Ok(())
}
