use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() -> io::Result<()> {
    // Pfad zur Eingabedatei
    let input = "input.txt";

    // Lese die Listen aus der Datei
    let (left_list, right_list) = read_lists_from_file(input)?;

    // Berechne die Ähnlichkeitspunkte
    let similarity_score = calculate_similarity_score(&left_list, &right_list);

    // Ausgabe der Ähnlichkeitspunkte
    println!("Die Ähnlichkeitspunkte zwischen den Listen betragen: {}", similarity_score);

    Ok(())
}

fn read_lists_from_file<P>(filename: P) -> io::Result<(Vec<i32>, Vec<i32>)>
where
    P: AsRef<Path>,
{
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

fn calculate_similarity_score(left_list: &Vec<i32>, right_list: &Vec<i32>) -> i32 {
    let mut frequency_map = HashMap::new();

    // Zähle die Häufigkeit jedes Elements in der rechten Liste
    for &num in right_list {
        *frequency_map.entry(num).or_insert(0) += 1;
    }

    // Berechne die Ähnlichkeitspunkte
    let mut similarity_score = 0;
    for &num in left_list {
        if let Some(&count) = frequency_map.get(&num) {
            similarity_score += num * count;
        }
    }

    similarity_score
}