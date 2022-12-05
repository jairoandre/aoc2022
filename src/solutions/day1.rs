use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn solve() -> Result<(), Box<dyn std::error::Error>> {

    let file = File::open("src/inputs/day1.txt")?;
    let reader = BufReader::new(file);

    let mut record = 0;
    let mut current = 0;

    for line in reader.lines() {
        match line?.parse::<i32>() {
            Ok(n) => current += n,
            Err(_) => {
                if current > record {
                    record = current;
                }
                current = 0;
            }
        }
    }
    println!("{}", record);
    Ok(())
}

#[allow(dead_code)]
pub fn solve2() -> Result<(), Box<dyn std::error::Error>> {

    let file = File::open("src/inputs/day1.txt")?;
    let reader = BufReader::new(file);

    let mut record = [0;3];
    let mut current = 0;

    for line in reader.lines() {
        match line?.parse::<i32>() {
            Ok(n) => current += n,
            Err(_) => {
                for i in 0..record.len() {
                    if current > record[i] {
                        record[i] = current;
                        break;
                    }
                }
                current = 0;
            }
        }
    }
    println!("{}", record[0] + record[1] + record[2]);
    Ok(())

}
