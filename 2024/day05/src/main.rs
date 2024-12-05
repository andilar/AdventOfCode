use std::collections::{ HashMap, VecDeque };
use std::fs::File;
use std::io::{ self, BufRead };
use std::path::Path;

fn main() -> io::Result<()> {
    // Read the input from the file
    let input = read_input("input.txt")?;
    let mut lines = input.lines();
    // Read the rules
    let mut rules = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        if let Some((x, y)) = line.split_once('|') {
            rules.push((x, y));
        } else {
            eprintln!("Error: Rule '{}' is not in the expected format.", line);
            return Ok(());
        }
    }
    // Print the rules for debugging
    for (x, y) in &rules {
        println!("Rule: {}|{}", x, y);
    }
    // Read the updates
    let updates: Vec<Vec<&str>> = lines.map(|line| line.split(',').collect()).collect();
    // Print the updates for debugging
    for update in &updates {
        println!("Update: {:?}", update);
    }
    let mut graph = HashMap::new();
    let mut in_degree = HashMap::new();
    for (x, y) in rules {
        graph.entry(x).or_insert_with(Vec::new).push(y);
        *in_degree.entry(y).or_insert(0) += 1;
        in_degree.entry(x).or_insert(0);
    }
    let mut sum_of_middle_pages = 0;
    for update in updates {
        if is_correct_order(&update, &graph, &in_degree) {
            let middle_page = find_middle_page(&update);
            println!("Middle page of update {:?}: {}", update, middle_page); // Debugging
            sum_of_middle_pages += middle_page.parse::<i32>().unwrap();
        } else {
            println!("Update {:?} is not in the correct order", update); // Debugging
        }
    }
    println!("Sum of middle pages: {}", sum_of_middle_pages);
    Ok(())
}

fn read_input(filename: &str) -> io::Result<String> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let mut content = String::new();
    for line in io::BufReader::new(file).lines() {
        content.push_str(&line?);
        content.push('\n');
    }
    Ok(content)
}

fn is_correct_order(
    update: &[&str],
    graph: &HashMap<&str, Vec<&str>>,
    in_degree: &HashMap<&str, i32>
) -> bool {
    let mut in_degree = in_degree.clone();
    let mut queue = VecDeque::new();
    for &page in update {
        if *in_degree.get(page).unwrap_or(&0) == 0 {
            queue.push_back(page);
        }
    }
    let mut sorted = Vec::new();
    while let Some(page) = queue.pop_front() {
        sorted.push(page);
        if let Some(neighbors) = graph.get(page) {
            for &neighbor in neighbors {
                if let Some(degree) = in_degree.get_mut(neighbor) {
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }
    sorted.len() == update.len()
}

fn find_middle_page<'a>(update: &'a [&str]) -> &'a str {
    let mid = update.len() / 2;
    update[mid]
}
