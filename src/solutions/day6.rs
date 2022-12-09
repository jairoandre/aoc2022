use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

fn all_numbers_are_different(arr: [i32; BUFFER_SIZE]) -> bool {
    let mut set = HashSet::new();

    // Insert the numbers in the array into the set
    for &num in arr.iter() {
        set.insert(num);
    }

    // Check if the size of the set is equal to the size of the array
    set.len() == arr.len()
}

const BUFFER_SIZE: usize = 14;

#[allow(dead_code)]
pub fn solve() -> Result<(), Box<dyn std::error::Error>> {
    let mut result = 0;
    let file = File::open("src/inputs/day6.txt")?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let str_arr = line?.as_bytes().to_vec();
        let mut buffer: [i32;BUFFER_SIZE] = [0;BUFFER_SIZE];
        let mut idx = 0usize;
        let mut count = 0;
        for c in str_arr.iter() {
            buffer[idx] = *c as i32;
            idx = (idx + 1) % BUFFER_SIZE;
            count += 1;
            if count > 4 {
                if all_numbers_are_different(buffer) {
                    result = count;
                    break;
                }
            }
        }
    }
    println!("{result}");
    Ok(())
}
