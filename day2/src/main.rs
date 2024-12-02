use std::fs::read_to_string;
use std::time::Instant;

fn read_levels(file_path: &str) -> Result<Vec<Vec<i32>>, Box<dyn std::error::Error>> {
    let input = read_to_string(file_path)?;
    let mut result = Vec::new();

    for line in input.lines() {
        let row: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse::<i32>())
            .collect::<Result<Vec<_>, _>>()?;
        result.push(row);
    }

    Ok(result)
}

fn level_is_increasing(level: &[i32]) -> bool {
    for i in 0..level.len() - 1 {
        if level[i] > level[i + 1] {
            return false;
        }
    }

    true
}

fn level_is_decreasing(level: &[i32]) -> bool {
    for i in 0..level.len() - 1 {
        if level[i] < level[i + 1] {
            return false;
        }
    }

    true
}

fn level_is_safe(level: &[i32]) -> bool {
    if level_is_increasing(&level) || level_is_decreasing(&level) {
        for i in 0..level.len() - 1 {
            let diff = (level[i] - level[i + 1]).abs();
            if diff == 0 || diff > 3 {
                return false;
            }
        }
    } else {
        return false;
    }

    true
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    let levels = read_levels("input.txt")?;
    let mut number_of_safe_levels = 0;

    for level in &levels {
        if level_is_safe(&level) {
            number_of_safe_levels += 1;
        } else {
            // Start removing an item, and check back if level is safe.
            for i in 0..level.len() {
                let temp_level = [&level[..i], &level[i + 1..]].concat();
                if level_is_safe(&temp_level) {
                    number_of_safe_levels += 1;
                    break;
                }
            }
        }
    }

    println!("Safe levels: {:?}", number_of_safe_levels);
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
    Ok(())
}

