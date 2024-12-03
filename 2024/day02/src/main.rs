use std::fs::File;
use std::io::{ self, BufRead, BufReader };
use std::path::Path;

fn main() -> io::Result<()> {
    let input = "input.txt";
    let reports = read_reports_from_file(input)?;
    let safe_reports_count = count_safe_reports(&reports);

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
        .filter(|report| is_safe_or_can_be_made_safe(report))
        .count()
}

fn is_safe_or_can_be_made_safe(report: &Vec<i32>) -> bool {
    if is_safe_report(report) {
        return true;
    }

    for i in 0..report.len() {
        let mut modified_report = report.clone();
        modified_report.remove(i);
        if is_safe_report(&modified_report) {
            return true;
        }
    }

    false
}

fn is_safe_report(report: &Vec<i32>) -> bool {
    if report.len() < 2 {
        return true;
    }

    let mut increasing = true;
    let mut decreasing = true;

    for window in report.windows(2) {
        let diff = window[1] - window[0];
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }
        
        if diff != 0 {
            if diff < 0 {
                increasing = false;
            } else {
                decreasing = false;
            }
        }
    }

    increasing || decreasing
}
