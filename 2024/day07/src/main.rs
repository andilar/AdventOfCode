use std::fs::File;
use std::io::{ self, BufRead };
use std::path::Path;
use std::time::Instant;

fn parse_input(filename: &str) -> io::Result<Vec<(i64, Vec<i64>)>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let mut equations = Vec::new();

    for line in io::BufReader::new(file).lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(": ").collect();
        let test_value: i64 = parts[0].parse().unwrap();
        let numbers: Vec<i64> = parts[1]
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        equations.push((test_value, numbers));
    }

    Ok(equations)
}

fn generate_expressions(
    numbers: &[i64],
    operators: &[&str],
    index: usize,
    current_expr: &mut Vec<String>,
    expressions: &mut Vec<String>
) {
    if index == numbers.len() - 1 {
        expressions.push(current_expr.join(" "));
        return;
    }

    for &op in operators {
        current_expr.push(op.to_string());
        current_expr.push(numbers[index + 1].to_string());
        generate_expressions(numbers, operators, index + 1, current_expr, expressions);
        current_expr.pop();
        current_expr.pop();
    }
}

fn evaluate_expression(expression: &str) -> i64 {
    let tokens: Vec<&str> = expression.split_whitespace().collect();
    let mut result = tokens[0].parse::<i64>().unwrap();

    let mut i = 1;
    while i < tokens.len() {
        let op = tokens[i];
        let num = tokens[i + 1].parse::<i64>().unwrap();
        if op == "+" {
            result += num;
        } else if op == "*" {
            result *= num;
        } else if op == "||" {
            result = result * (10_i64).pow(num.to_string().len() as u32) + num;
        }
        i += 2;
    }

    result
}

fn main() -> io::Result<()> {
    let start_time = Instant::now();

    let equations = parse_input("input.txt")?;
    let operators = vec!["+", "*", "||"];
    let mut total_calibration_result = 0;

    for (test_value, numbers) in equations {
        let mut expressions = Vec::new();
        let mut current_expr = vec![numbers[0].to_string()];
        generate_expressions(&numbers, &operators, 0, &mut current_expr, &mut expressions);

        for expr in expressions {
            if evaluate_expression(&expr) == test_value {
                total_calibration_result += test_value;
                break;
            }
        }
    }

    let duration = start_time.elapsed();
    println!("Total calibration result: {}", total_calibration_result);
    println!("Time taken: {:?}", duration);

    Ok(())
}
