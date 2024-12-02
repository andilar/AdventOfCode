use std::fs::File;
use std::io::{ self, BufRead, BufReader };
use std::path::Path;

fn main() -> io::Result<()> {
    // Pfad zur Eingabedatei
    let input = "input.txt";

    // Lese die Listen aus der Datei
    let (mut left_list, mut right_list) = read_lists_from_file(input)?;

    // Sortiere beide Listen
    left_list.sort();
    right_list.sort();

    // Berechne die Gesamtdistanz
    let total_distance = calculate_total_distance(&left_list, &right_list);

    // Ausgabe der Gesamtdistanz
    println!("Die Gesamtdistanz zwischen den Listen beträgt: {}", total_distance);

    Ok(())
}

fn read_lists_from_file<P>(filename: P) -> io::Result<(Vec<i32>, Vec<i32>)> where P: AsRef<Path> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        if numbers.len() == 2 {
            left_list.push(numbers[0]);
            right_list.push(numbers[1]);
        }
    }

    Ok((left_list, right_list))
}

fn calculate_total_distance(left_list: &Vec<i32>, right_list: &Vec<i32>) -> i32 {
    let mut total_distance = 0;

    // Paare die Zahlen und berechne die Abstände
    for (left, right) in left_list.iter().zip(right_list.iter()) {
        total_distance += (left - right).abs();
    }

    total_distance
}
