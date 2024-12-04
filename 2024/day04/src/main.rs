use std::fs::File;
use std::io::{ self, BufRead };
use std::path::Path;

fn main() {
    // Read the input file
    let path = "input.txt";
    let word_search = read_lines(path).expect("Could not read file");

    // Convert the word search into a 2D vector of characters
    let word_search: Vec<Vec<char>> = word_search
        .iter()
        .map(|line| line.chars().collect())
        .collect();

    // Count occurrences of "XMAS"
    let total_count = count_xmas_occurrences(&word_search, "XMAS");
    println!("Total occurrences of XMAS: {}", total_count);
}

fn read_lines<P>(filename: P) -> io::Result<Vec<String>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    let buf = io::BufReader::new(file);
    buf.lines().collect()
}

fn count_xmas_occurrences(word_search: &Vec<Vec<char>>, word: &str) -> usize {
    let n = word_search.len();
    let word_len = word.len();
    let word_chars: Vec<char> = word.chars().collect();
    let mut count = 0;

    // Horizontal and vertical search
    for i in 0..n {
        for j in 0..=n - word_len {
            // Horizontal left to right
            if word_search[i][j..j + word_len] == word_chars[..] {
                count += 1;
            }
            // Horizontal right to left
            if word_search[i][j..j + word_len].iter().rev().eq(word_chars.iter()) {
                count += 1;
            }
            // Vertical top to bottom
            if (0..word_len).all(|k| word_search[i + k][j] == word_chars[k]) {
                count += 1;
            }
            // Vertical bottom to top
            if (0..word_len).all(|k| word_search[i + k][j] == word_chars[word_len - 1 - k]) {
                count += 1;
            }
        }
    }

    // Diagonal search
    for i in 0..=n - word_len {
        for j in 0..=n - word_len {
            // Diagonal top-left to bottom-right
            if (0..word_len).all(|k| word_search[i + k][j + k] == word_chars[k]) {
                count += 1;
            }
            // Diagonal bottom-right to top-left
            if (0..word_len).all(|k| word_search[i + k][j + k] == word_chars[word_len - 1 - k]) {
                count += 1;
            }
            // Diagonal top-right to bottom-left
            if (0..word_len).all(|k| word_search[i + k][j + word_len - 1 - k] == word_chars[k]) {
                count += 1;
            }
            // Diagonal bottom-left to top-right
            if
                (0..word_len).all(
                    |k| word_search[i + k][j + word_len - 1 - k] == word_chars[word_len - 1 - k]
                )
            {
                count += 1;
            }
        }
    }

    count
}
