use regex::Regex;
use std::error::Error;
use std::fs::read_to_string;
use std::time::Instant;

fn solve_part_a(contents: &str) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),\s*(\d{1,3})\)").unwrap();
    let mut sum = 0;

    for captures in re.captures_iter(&contents) {
        // let full_match = captures.get(0).unwrap().as_str();
        // println!("{:?}", full_match);
        let x_str = captures.get(1).unwrap().as_str();
        let y_str = captures.get(2).unwrap().as_str();
        let x: i32 = x_str.parse().unwrap();
        let y: i32 = y_str.parse().unwrap();
        sum += x * y;
    }

    sum
}

fn solve_part_b(contents: &str) -> i32 {
    // Now also capture "do()" and "don't()".
    let re = regex::Regex::new(r"mul\((\d+),\s*(\d+)\)|do\(\)|don't\(\)").unwrap();
    let mut mul_enabled = true;
    let mut sum = 0;

    for captures in re.captures_iter(contents) {
        if let Some(matched) = captures.get(0) {
            let instruction = matched.as_str();

            if instruction == "do()" {
                mul_enabled = true;
            } else if instruction == "don't()" {
                mul_enabled = false;
            } else if mul_enabled {
                let result = solve_part_a(instruction);
                sum += result;
            }
        }
    }

    sum
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "input.txt";
    let contents = read_to_string(file_path)?;

    // Solve part a
    let start = Instant::now();
    let answer_a = solve_part_a(&contents);
    println!("Answer (A): {:?}", answer_a);
    let duration = start.elapsed();
    println!("Time elapsed: {:?}\n", duration);

    // Solve part b
    let start = Instant::now();
    let answer_b = solve_part_b(&contents);
    println!("Answer (B): {:?}", answer_b);
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);

    Ok(())
}
