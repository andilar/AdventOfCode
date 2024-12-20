use std::fs::File;
use std::io::{ self, BufRead };
use std::path::Path;

#[derive(Debug)]
struct Machine {
    a_x: i32,
    a_y: i32,
    b_x: i32,
    b_y: i32,
    prize_x: i32,
    prize_y: i32,
}

fn read_input_from_file(filename: &str) -> io::Result<Vec<Machine>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut machines = Vec::new();
    let mut lines = reader.lines();

    while let Some(Ok(line)) = lines.next() {
        if line.starts_with("Button A:") {
            let a_x = line
                .split_whitespace()
                .nth(2)
                .unwrap()
                .trim_start_matches("X+")
                .parse()
                .unwrap_or(0);
            let a_y = line
                .split_whitespace()
                .nth(3)
                .unwrap()
                .trim_start_matches("Y+")
                .parse()
                .unwrap_or(0);
            let b_line = lines.next().unwrap().unwrap();
            let b_x = b_line
                .split_whitespace()
                .nth(2)
                .unwrap()
                .trim_start_matches("X+")
                .parse()
                .unwrap_or(0);
            let b_y = b_line
                .split_whitespace()
                .nth(3)
                .unwrap()
                .trim_start_matches("Y+")
                .parse()
                .unwrap_or(0);
            let prize_line = lines.next().unwrap().unwrap();
            let prize_x = prize_line
                .split_whitespace()
                .nth(1)
                .unwrap()
                .trim_start_matches("X=")
                .parse()
                .unwrap_or(0);
            let prize_y = prize_line
                .split_whitespace()
                .nth(2)
                .unwrap()
                .trim_start_matches("Y=")
                .parse()
                .unwrap_or(0);

            machines.push(Machine { a_x, a_y, b_x, b_y, prize_x, prize_y });
        }
    }

    Ok(machines)
}

fn extended_gcd(a: i32, b: i32) -> (i32, i32, i32) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (g, x, y) = extended_gcd(b, a % b);
        (g, y, x - (a / b) * y)
    }
}

fn solve_diophantine(a: i32, b: i32, c: i32) -> Option<(i32, i32)> {
    let (g, x, y) = extended_gcd(a, b);
    if c % g != 0 {
        None
    } else {
        let k = c / g;
        Some((x * k, y * k))
    }
}

fn find_min_tokens(machine: &Machine) -> Option<i32> {
    if let Some((x, y)) = solve_diophantine(machine.a_x, machine.b_x, machine.prize_x) {
        if let Some((_, _)) = solve_diophantine(machine.a_y, machine.b_y, machine.prize_y) {
            let mut min_tokens = None;
            let gcd_x = extended_gcd(machine.a_x, machine.b_x).0;
            let gcd_y = extended_gcd(machine.a_y, machine.b_y).0;
            if gcd_x == 0 || gcd_y == 0 {
                return None;
            }
            for k in 0..=100 {
                let a_presses = x + k * (machine.b_x / gcd_x);
                let b_presses = y - k * (machine.a_x / gcd_x);
                if a_presses >= 0 && b_presses >= 0 && a_presses <= 100 && b_presses <= 100 {
                    let tokens = a_presses * 3 + b_presses;
                    if min_tokens.is_none() || tokens < min_tokens.unwrap() {
                        min_tokens = Some(tokens);
                    }
                }
            }
            return min_tokens;
        }
    }
    None
}

fn main() -> io::Result<()> {
    let machines = read_input_from_file("input.txt")?;
    let mut total_tokens = 0;
    let mut prizes_won = 0;

    for machine in machines {
        if let Some(tokens) = find_min_tokens(&machine) {
            total_tokens += tokens;
            prizes_won += 1;
        }
    }

    println!("Total prizes won: {}", prizes_won);
    println!("Total tokens spent: {}", total_tokens);

    Ok(())
}
