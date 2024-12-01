use std::fs::read_to_string;

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
