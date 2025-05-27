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
    let mut machine = Machine {
        a_x: 0,
        a_y: 0,
        b_x: 0,
        b_y: 0,
        prize_x: 0,
        prize_y: 0,
    };

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            // If we encounter an empty line, it means we have finished reading one machine's data
            machines.push(machine);
            machine = Machine {
                a_x: 0,
                a_y: 0,
                b_x: 0,
                b_y: 0,
                prize_x: 0,
                prize_y: 0,
            };
        } else if line.starts_with("Button A:") {
            let parts: Vec<&str> = line.split(&[',', ' '][..]).collect();
            machine.a_x = parts[2].trim_start_matches("X+").parse().unwrap_or(0);
            machine.a_y = parts[4].trim_start_matches("Y+").parse().unwrap_or(0);
        } else if line.starts_with("Button B:") {
            let parts: Vec<&str> = line.split(&[',', ' '][..]).collect();
            machine.b_x = parts[2].trim_start_matches("X+").parse().unwrap_or(0);
            machine.b_y = parts[4].trim_start_matches("Y+").parse().unwrap_or(0);
        } else if line.starts_with("Prize:") {
            let parts: Vec<&str> = line.split(&[',', ' '][..]).collect();
            machine.prize_x = parts[1].trim_start_matches("X=").parse().unwrap_or(0);
            machine.prize_y = parts[3].trim_start_matches("Y=").parse().unwrap_or(0);
        }
    }

    // Add the last machine if the file does not end with an empty line
    if
        machine.a_x != 0 ||
        machine.a_y != 0 ||
        machine.b_x != 0 ||
        machine.b_y != 0 ||
        machine.prize_x != 0 ||
        machine.prize_y != 0
    {
        machines.push(machine);
    }

    Ok(machines)
}

fn extended_gcd(a: i32, b: i32) -> (i32, i32, i32) {
    if b == 0 {
        (a.abs(), 1, 0)
    } else {
        let (g, x, y) = extended_gcd(b, a % b);
        let result = (g, y, x - (a / b) * y);
        println!("g: {}, y: {}, x - (a / b) * y: {}", result.0, result.1, result.2);
        result
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
        if let Some((u, v)) = solve_diophantine(machine.a_y, machine.b_y, machine.prize_y) {
            let mut min_tokens = None;
            for k in 0..=100 {
                let a_presses = x + k * (machine.b_x / extended_gcd(machine.a_x, machine.b_x).0);
                let b_presses = y - k * (machine.a_x / extended_gcd(machine.a_x, machine.b_x).0);
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
