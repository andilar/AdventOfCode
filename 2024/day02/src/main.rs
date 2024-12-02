use std::fs::File;
use std::io::{ self, BufRead, BufReader };
use std::path::Path;

fn main() -> io::Result<()> {
    // Pfad zur Eingabedatei
    let input = "input.txt";

    // Lese die Berichte aus der Datei
    let reports = read_reports_from_file(input)?;

    // Berechne die Anzahl der sicheren Berichte
    let safe_reports_count = count_safe_reports(&reports);

    // Ausgabe der Anzahl der sicheren Berichte
    println!("Die Anzahl der sicheren Berichte beträgt: {}", safe_reports_count);

    Ok(())
}

fn read_reports_from_file<P>(filename: P) -> io::Result<Vec<Vec<i32>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut reports = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let report: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        reports.push(report);
    }

    Ok(reports)
}

fn count_safe_reports(reports: &Vec<Vec<i32>>) -> usize {
    reports
        .iter()
        .filter(|report| is_safe_report(report))
        .count()
}

fn is_safe_report(report: &Vec<i32>) -> bool {
    if report.len() < 2 {
        return true;
    }

    let mut increasing = true;
    let mut decreasing = true;

    for window in report.windows(2) {
        let diff = window[1] - window[0];
        if diff < 1 || diff > 3 {
            return false;
        }
        if diff < 0 {
            increasing = false;
        }
        if diff > 0 {
            decreasing = false;
        }
    }

    increasing || decreasing
}
