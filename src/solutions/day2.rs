use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn solve() -> Result<(), Box<dyn std::error::Error>> {

    let file = File::open("src/inputs/day2.txt")?;
    let reader = BufReader::new(file);

    let mut total = 0;
    let mut total_2 = 0;

    for line in reader.lines() {
        let s = format!("{}", line?);
        total += score(&s);
        total_2 += score_2(&s)
    }
    println!("score : {}", total);
    println!("score_2 : {}", total_2);
    Ok(())
}
// ROCK = A, X
// PAPERS = B, Y
// SCISSORS = C, Z
fn score(game : &str) -> i32 {
    match game {
        "A X" => 4,
        "B Y" => 5,
        "C Z" => 6,
        "A Z" => 3,
        "B X" => 1,
        "C Y" => 2,
        "A Y" => 8,
        "B Z" => 9,
        "C X" => 7,
        _ => 0
    }
}

fn score_2(game : &str) -> i32 {
    match game {
        "A X" => 3,
        "A Y" => 4,
        "A Z" => 8,
        "B X" => 1,
        "B Y" => 5,
        "B Z" => 9,
        "C X" => 2,
        "C Y" => 6,
        "C Z" => 7,
        _ => 0
    }
}
