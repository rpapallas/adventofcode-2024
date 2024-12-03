use std::error::Error;
use std::fs::read_to_string;
use std::time::Instant;

fn quicksort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot_index = partition(arr);
    quicksort(&mut arr[0..pivot_index]);
    quicksort(&mut arr[pivot_index + 1..]);
}

fn partition(arr: &mut [i32]) -> usize {
    let pivot_index = arr.len() - 1;
    let pivot = arr[pivot_index];
    let mut i = 0;

    for j in 0..pivot_index {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, pivot_index);
    i
}

fn read_file(file_path: &str) -> Result<(Vec<i32>, Vec<i32>), Box<dyn Error>> {
    let input = read_to_string(file_path)?;

    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            list1.push(parts[0].parse::<i32>()?);
            list2.push(parts[1].parse::<i32>()?);
        }
    }

    Ok((list1, list2))
}

fn solve_part_a(list1: &mut [i32], list2: &mut [i32]) {
    quicksort(list1);
    quicksort(list2);

    let mut sum_of_distances = 0;
    for (a, b) in list1.iter().zip(list2.iter()) {
        let distance = a - b;
        sum_of_distances = sum_of_distances + distance.abs();
    }

    println!("Sum of distances: {:?}", sum_of_distances);
}

fn solve_part_b(list1: &mut [i32], list2: &mut [i32]) {
    quicksort(list1);
    quicksort(list2);

    let mut similarity_score = 0;
    for a in list1.iter() {
        let mut counter = 0;
        for b in list2.iter() {
            if b > a {
                break;
            }

            if a == b {
                counter = counter + 1;
            }
        }
        similarity_score = similarity_score + counter * a;
    }

    println!("Similarity score: {:?}", similarity_score);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut list1, mut list2) = read_file("input.txt")?;

    let start = Instant::now();
    solve_part_a(&mut list1, &mut list2);
    let duration = start.elapsed();
    println!("Time elapsed: {:?}\n", duration);

    let start = Instant::now();
    solve_part_b(&mut list1, &mut list2);
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);

    Ok(())
}
