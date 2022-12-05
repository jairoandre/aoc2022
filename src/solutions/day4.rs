use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_int(v : &str) -> i32 {
    v.parse::<i32>().unwrap()
}

fn check_in(r1 : &Vec<i32>, r2 : &Vec<i32>) -> bool {
    r1[0] <= r2[0] && r1[1] >= r2[1]
}

fn check_at_all(r1 : &Vec<i32>, r2 : &Vec<i32>) -> bool {
    (r1[0] <= r2[0] && r2[0] <= r1[1]) || (r2[1] >= r1[0] && r2[1] <= r1[1])
}
#[allow(dead_code)]
pub fn solve() -> Result<(), Box<dyn std::error::Error>> {

    let file = File::open("src/inputs/day4.txt")?;
    let reader = BufReader::new(file);
    let mut total = 0;
    let mut total_2 = 0;

    for line in reader.lines() {
        let ss = format!("{}", line?);
        let ranges : Vec<Vec<i32>> = ss.split(",")
            .map(|s| s.split("-")
                .map(|r| parse_int(r))
                .collect())
            .collect();
        let r1 = &ranges[0];
        let r2 = &ranges[1];
        if check_in(r1, r2) || check_in(r2, r1) {
            total += 1;
        }
        if check_at_all(r1, r2) || check_at_all(r2, r1) {
            total_2 += 1;
        }
    }
    println!("{total} | {total_2}");
    Ok(())
}
