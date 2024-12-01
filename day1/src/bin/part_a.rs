use std::error::Error;
use std::time::Instant;
include!("../common.rs");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    let (mut list1, mut list2) = read_file("input.txt")?;
    quicksort(&mut list1);
    quicksort(&mut list2);

    let mut sum_of_distances = 0;
    for (a, b) in list1.iter().zip(list2.iter()) {
        let distance = a - b;
        sum_of_distances = sum_of_distances + distance.abs();
    }

    println!("Sum of distances: {:?}", sum_of_distances);
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
    Ok(())
}
