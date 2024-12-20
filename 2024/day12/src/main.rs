use std::collections::{ HashSet, VecDeque };
use std::fs::File;
use std::io::{ self, BufRead };
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

fn get_neighbors(point: Point, width: usize, height: usize) -> Vec<Point> {
    let mut neighbors = Vec::new();
    if point.x > 0 {
        neighbors.push(Point { x: point.x - 1, y: point.y });
    }
    if point.x < width - 1 {
        neighbors.push(Point { x: point.x + 1, y: point.y });
    }
    if point.y > 0 {
        neighbors.push(Point { x: point.x, y: point.y - 1 });
    }
    if point.y < height - 1 {
        neighbors.push(Point { x: point.x, y: point.y + 1 });
    }
    neighbors
}

fn bfs(map: &Vec<Vec<char>>, start: Point, visited: &mut HashSet<Point>) -> (usize, usize) {
    let mut queue = VecDeque::new();
    let mut area = 0;
    let mut perimeter = 0;
    let plant_type = map[start.y][start.x];
    let width = map[0].len();
    let height = map.len();

    queue.push_back(start);
    visited.insert(start);

    while let Some(point) = queue.pop_front() {
        area += 1;
        let mut local_perimeter = 4;

        for neighbor in get_neighbors(point, width, height) {
            if map[neighbor.y][neighbor.x] == plant_type {
                local_perimeter -= 1;
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back(neighbor);
                }
            }
        }
        perimeter += local_perimeter;
    }

    (area, perimeter)
}

fn calculate_total_price(map: &Vec<Vec<char>>) -> usize {
    let mut visited = HashSet::new();
    let mut total_price = 0;

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let point = Point { x, y };
            if !visited.contains(&point) {
                let (area, perimeter) = bfs(map, point, &mut visited);
                total_price += area * perimeter;
            }
        }
    }

    total_price
}

fn read_input_from_file(filename: &str) -> io::Result<Vec<Vec<char>>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut map = Vec::new();
    for line in reader.lines() {
        let line = line?;
        map.push(line.chars().collect());
    }

    Ok(map)
}

fn main() -> io::Result<()> {
    let map = read_input_from_file("input.txt")?;
    let total_price = calculate_total_price(&map);
    println!("Total price of fencing all regions: {}", total_price);
    Ok(())
}
