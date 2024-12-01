use std::error::Error;
use std::time::Instant;
include!("../common.rs");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    let (mut list1, mut list2) = read_file("input.txt")?;
    quicksort(&mut list1);
    quicksort(&mut list2);

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
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
    Ok(())
}
