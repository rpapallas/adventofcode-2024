use std::fs::read_to_string;
use std::time::Instant;

fn read_input(file_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let input = read_to_string(file_path)?;
    let result = input.lines().map(|line| line.to_string()).collect();

    Ok(result)
}

// -- Part A ---

fn invert_string(input: &str) -> String {
    input.chars().rev().collect()
}

fn count_xmas(input: &str) -> usize {
    let mut num_of_xmas_found = 0;
    let count = input.matches("XMAS").count();
    num_of_xmas_found += count;

    let count = invert_string(input).matches("XMAS").count();
    num_of_xmas_found += count;

    num_of_xmas_found
}

fn search_lines(input: &Vec<String>) -> usize {
    let mut num_of_xmas: usize = 0;

    for line in input.iter() {
        num_of_xmas += count_xmas(&line);
    }

    num_of_xmas
}

fn search_columns(input: &Vec<String>) -> usize {
    let mut num_of_xmas: usize = 0;
    let num_of_rows = input.len();
    let num_of_cols = input[0].len();

    for col_index in 0..num_of_cols {
        let mut col = String::new();
        for row_index in 0..num_of_rows {
            if let Some(ch) = input[row_index].chars().nth(col_index) {
                col.push(ch);
            }
        }
        num_of_xmas += count_xmas(&col);
    }

    num_of_xmas
}

fn search_diagonals(input: &Vec<String>) -> usize {
    let mut num_of_xmas: usize = 0;
    let num_of_rows = input.len();
    let num_of_cols = input[0].len();

    // Top-left to bottom-right
    for start_col in 0..num_of_cols {
        let mut col = String::new();
        let mut row = 0;
        let mut col_index = start_col;

        while row < num_of_rows && col_index < num_of_cols {
            if let Some(ch) = input[row].chars().nth(col_index) {
                col.push(ch);
            }
            row += 1;
            col_index += 1;
        }

        num_of_xmas += count_xmas(&col);
    }

    for start_row in 1..num_of_rows {
        let mut col = String::new();
        let mut row = start_row;
        let mut col_index = 0;

        while row < num_of_rows && col_index < num_of_cols {
            if let Some(ch) = input[row].chars().nth(col_index) {
                col.push(ch);
            }
            row += 1;
            col_index += 1;
        }

        num_of_xmas += count_xmas(&col);
    }

    // Top-right to bottom-left
    for start_col in (0..num_of_cols).rev() {
        let mut col = String::new();
        let mut row = 0;
        let mut col_index = start_col;

        while row < num_of_rows && col_index < num_of_cols {
            if let Some(ch) = input[row].chars().nth(col_index) {
                col.push(ch);
            }
            row += 1;
            if col_index == 0 {
                break;
            }
            col_index -= 1;
        }

        num_of_xmas += count_xmas(&col);
    }

    for start_row in 1..num_of_rows {
        let mut col = String::new();
        let mut row = start_row;
        let mut col_index = num_of_cols - 1;

        while row < num_of_rows && col_index < num_of_cols {
            if let Some(ch) = input[row].chars().nth(col_index) {
                col.push(ch);
            }
            row += 1;
            if col_index == 0 {
                break;
            }
            col_index -= 1;
        }

        num_of_xmas += count_xmas(&col);
    }

    num_of_xmas
}

fn solve_part_a(input: &Vec<String>) {
    let mut num_of_xmas = 0;

    num_of_xmas += search_lines(&input);
    num_of_xmas += search_columns(&input);
    num_of_xmas += search_diagonals(&input);

    println!("Num of xmas: {:?}", num_of_xmas);
}

// -- Part B ---

fn solve_part_b(input: &Vec<String>) {
    let mut num_of_x_mas = 0;
    let rows = input.len();
    let cols = input[0].len();

    for row in 1..rows - 1 {
        for col in 1..cols - 1 {
            if input[row].chars().nth(col) != Some('A') {
                continue;
            }

            let tl = input[row - 1].chars().nth(col - 1);
            let tr = input[row - 1].chars().nth(col + 1);

            let bl = input[row + 1].chars().nth(col - 1);
            let br = input[row + 1].chars().nth(col + 1);

            if let (Some(tl_ch), Some(tr_ch), Some(bl_ch), Some(br_ch)) = (tl, tr, bl, br) {
                let word1 = format!("{}A{}", tl_ch, br_ch);
                let word2 = format!("{}A{}", tr_ch, bl_ch);

                if (word1 == "MAS" || word1 == "SAM") && (word2 == "MAS" || word2 == "SAM") {
                    num_of_x_mas += 1;
                }
            }
        }
    }

    println!("Number of X-MASes: {}", num_of_x_mas);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_input("input.txt")?;

    let start = Instant::now();
    solve_part_a(&input);
    let duration = start.elapsed();
    println!("Time elapsed: {:?}\n", duration);

    let start = Instant::now();
    solve_part_b(&input);
    let duration = start.elapsed();
    println!("Time elapsed: {:?}\n", duration);

    Ok(())
}
