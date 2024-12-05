use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;

fn read_file(
    file_path: &str,
) -> Result<(Vec<(i32, i32)>, Vec<Vec<i32>>), Box<dyn std::error::Error>> {
    let input = read_to_string(file_path)?;
    let mut sections = input.split("\n\n");

    let rules_section = sections
        .next()
        .ok_or("Missing rules section")?
        .lines()
        .map(|line| {
            let mut parts = line.split('|');
            let first = parts.next().ok_or("Invalid rule format")?.parse()?;
            let second = parts.next().ok_or("Invalid rule format")?.parse()?;
            Ok((first, second))
        })
        .collect::<Result<Vec<(i32, i32)>, Box<dyn std::error::Error>>>()?;

    let updates_section = sections
        .next()
        .ok_or("Missing updates section")?
        .lines()
        .map(|line| {
            line.split(',')
                .map(|num| num.parse())
                .collect::<Result<Vec<i32>, _>>()
        })
        .collect::<Result<Vec<Vec<i32>>, _>>()?;

    Ok((rules_section, updates_section))
}

fn convert_rules_to_hashmaps(
    rules: Vec<(i32, i32)>,
) -> (HashMap<i32, Vec<i32>>, HashMap<i32, Vec<i32>>) {
    let mut strong_rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut weak_rules: HashMap<i32, Vec<i32>> = HashMap::new();

    for (key, value) in rules {
        strong_rules.entry(key).or_insert_with(Vec::new).push(value);
        weak_rules.entry(value).or_insert_with(Vec::new).push(key);
    }

    (strong_rules, weak_rules)
}

fn is_update_right(
    update: &Vec<i32>,
    strong_map: &HashMap<i32, Vec<i32>>,
    weak_map: &HashMap<i32, Vec<i32>>,
) -> bool {
    let positions: HashMap<i32, usize> = update
        .iter()
        .enumerate()
        .map(|(i, &page)| (page, i))
        .collect();

    for (page, values) in strong_map {
        if let Some(&pos_page) = positions.get(page) {
            for &value in values {
                if let Some(&pos_value) = positions.get(&value) {
                    if pos_page > pos_value {
                        return false;
                    }
                }
            }
        }
    }

    for (page, values) in weak_map {
        if let Some(&pos_page) = positions.get(page) {
            for &value in values {
                if let Some(&pos_value) = positions.get(&value) {
                    if pos_page < pos_value {
                        return false;
                    }
                }
            }
        }
    }

    true
}

fn solve_part_a(updates: &Vec<Vec<i32>>, rules: &Vec<(i32, i32)>) {
    let mut right_updates: Vec<Vec<i32>> = Vec::new();
    let (strong_map, weak_map) = convert_rules_to_hashmaps(rules.to_vec());

    for update in updates {
        if is_update_right(&update, &strong_map, &weak_map) {
            right_updates.push(update.to_vec());
        }
    }

    let mut sum = 0;
    for update in right_updates {
        let mid: usize = update.len() / 2;
        sum += update[mid];
    }

    println!("Answer: {:?}", sum);
}

fn solve_part_b(updates: &Vec<Vec<i32>>, rules: &Vec<(i32, i32)>) {
    let mut wrong_updates: Vec<Vec<i32>> = Vec::new();
    let (strong_map, weak_map) = convert_rules_to_hashmaps(rules.to_vec());

    for update in updates {
        if !is_update_right(&update, &strong_map, &weak_map) {
            wrong_updates.push(update.to_vec());
        }
    }

    for update in &mut wrong_updates {
        while !is_update_right(&update, &strong_map, &weak_map) {
            let positions: HashMap<i32, usize> = update
                .iter()
                .enumerate()
                .map(|(i, &page)| (page, i))
                .collect();

            let mut swapped = false;

            for (page, values) in &strong_map {
                if let Some(&pos_page) = positions.get(page) {
                    for &value in values {
                        if let Some(&pos_value) = positions.get(&value) {
                            if pos_page > pos_value {
                                update.swap(pos_page, pos_value);
                                swapped = true;
                                break;
                            }
                        }
                    }
                }
                if swapped {
                    break;
                }
            }

            if swapped {
                continue;
            }

            for (page, values) in &weak_map {
                if let Some(&pos_page) = positions.get(page) {
                    for &value in values {
                        if let Some(&pos_value) = positions.get(&value) {
                            if pos_page < pos_value {
                                update.swap(pos_page, pos_value);
                                swapped = true;
                                break;
                            }
                        }
                    }
                }
                if swapped {
                    break;
                }
            }

            if !swapped {
                break;
            }
        }
    }

    let mut sum = 0;
    for update in wrong_updates {
        let mid: usize = update.len() / 2;
        sum += update[mid];
    }

    println!("Answer: {:?}", sum);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "input.txt";
    let (rules, updates) = read_file(file_path)?;

    let start = Instant::now();
    solve_part_a(&updates, &rules);
    let duration = start.elapsed();
    println!("Time elapsed: {:?}\n", duration);

    let start = Instant::now();
    solve_part_b(&updates, &rules);
    let duration = start.elapsed();
    println!("Time elapsed: {:?}\n", duration);

    Ok(())
}
