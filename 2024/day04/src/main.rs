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
    // Count occurrences of "X-MAS"
    let total_count = count_xmas_occurrences(&word_search);
    println!("Total occurrences of X-MAS: {}", total_count);
}

fn read_lines<P>(filename: P) -> io::Result<Vec<String>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    let buf = io::BufReader::new(file);
    buf.lines().collect()
}

fn count_xmas_occurrences(word_search: &Vec<Vec<char>>) -> usize {
    let n = word_search.len();
    let mut count = 0;
    // Check for X-MAS pattern
    for i in 0..n - 2 {
        for j in 0..n - 2 {
            // Check the X-MAS pattern in all 8 possible orientations
            if check_xmas_pattern(word_search, i, j) {
                count += 1;
            }
        }
    }
    count
}

fn check_xmas_pattern(word_search: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let patterns = vec![
        vec![(0, 0), (1, 1), (2, 2), (0, 2), (2, 0)], // Diagonal top-left to bottom-right
        vec![(2, 0), (1, 1), (0, 2), (2, 2), (0, 0)], // Diagonal bottom-left to top-right
        vec![(0, 2), (1, 1), (2, 0), (0, 0), (2, 2)], // Diagonal top-right to bottom-left
        vec![(2, 2), (1, 1), (0, 0), (2, 0), (0, 2)] // Diagonal bottom-right to top-left
    ];

    for pattern in patterns {
        if
            pattern
                .iter()
                .all(
                    |&(x, y)|
                        word_search[i + x][j + y] == 'M' ||
                        word_search[i + x][j + y] == 'A' ||
                        word_search[i + x][j + y] == 'S'
                )
        {
            let mas1 = vec![
                word_search[i + pattern[0].0][j + pattern[0].1],
                word_search[i + pattern[1].0][j + pattern[1].1],
                word_search[i + pattern[2].0][j + pattern[2].1]
            ];
            let mas2 = vec![
                word_search[i + pattern[3].0][j + pattern[3].1],
                word_search[i + pattern[1].0][j + pattern[1].1],
                word_search[i + pattern[4].0][j + pattern[4].1]
            ];
            if
                (mas1 == vec!['M', 'A', 'S'] || mas1 == vec!['S', 'A', 'M']) &&
                (mas2 == vec!['M', 'A', 'S'] || mas2 == vec!['S', 'A', 'M'])
            {
                return true;
            }
        }
    }
    false
}
