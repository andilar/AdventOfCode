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

fn main() -> io::Result<()> {
    let machines = read_input_from_file("input.txt")?;
    for (index, machine) in machines.iter().enumerate() {
        println!("Machine {}: {:?}", index + 1, machine);
    }
    Ok(())
}
