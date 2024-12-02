use std::fs;

pub fn day01() {
    let input = fs::read_to_string("day01-input.txt").unwrap();
    let lines: Vec<_> = input.lines().collect();
    let count = lines.len();

    let mut left: Vec<i32> = Vec::with_capacity(count);
    let mut right: Vec<i32> = Vec::with_capacity(count);
    for line in lines {
        let mut numbers = line.split_whitespace().filter_map(|s| s.parse().ok());
        left.push(numbers.next().unwrap());
        right.push(numbers.next().unwrap());
    }

    left.sort_unstable();
    right.sort_unstable();

    let mut total_distance = 0;
    for i in 0..count {
        total_distance += (left[i] - right[i]).abs();
    }

    println!("Total Distance: {}", total_distance);
}
