use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

/*
            [C]         [N] [R]
[J] [T]     [H]         [P] [L]
[F] [S] [T] [B]         [M] [D]
[C] [L] [J] [Z] [S]     [L] [B]
[N] [Q] [G] [J] [J]     [F] [F] [R]
[D] [V] [B] [L] [B] [Q] [D] [M] [T]
[B] [Z] [Z] [T] [V] [S] [V] [S] [D]
[W] [P] [P] [D] [G] [P] [B] [P] [V]
 1   2   3   4   5   6   7   8   9
 */


pub fn create_stack(args: Vec<&str>) -> Vec<&str> {
    let mut stack = Vec::new();
    for arg in args {
        stack.push(arg);
    }
    stack
}

#[allow(dead_code)]
pub fn solve() -> Result<(), Box<dyn std::error::Error>> {

    let mut supply_stacks =
        vec![
            vec!["W", "B", "D", "N", "C", "F", "J"],
            vec!["P", "Z", "V", "Q", "L", "S", "T"],
            vec!["P", "Z", "B", "G", "J", "T"],
            vec!["D", "T", "L", "J", "Z", "B", "H", "C"],
            vec!["G", "V", "B", "J", "S"],
            vec!["P", "S", "Q"],
            vec!["B", "V", "D", "F", "L", "M", "P", "N"],
            vec!["P", "S", "M", "F", "B", "D", "L", "R"],
            vec!["V", "D", "T", "R"]
        ];

    let file = File::open("src/inputs/day5.txt")?;
    let reader = BufReader::new(file);

    let re = Regex::new(r"\d+").unwrap();

    for line in reader.lines() {
        let mut numbers = Vec::new();
        let ss = format!("{}", line?);
        for caps in re.captures_iter(&ss) {
            numbers.push(caps[0].parse::<usize>().unwrap())
        }
        let qtd = numbers[0];
        let from = numbers[1] - 1;
        let to = numbers[2] - 1;
        let mut temp_stack = Vec::new();
        for _ in 0..qtd {
            let v = supply_stacks[from].pop().unwrap();
            temp_stack.push(v);
        }
        for _ in 0..qtd {
            let v = temp_stack.pop().unwrap();
            supply_stacks[to].push(v);
        }
    }
    let mut result = String::new();
    for stack in supply_stacks.iter() {
        result.push_str(format!("{}", stack[stack.len()-1]).as_str())
    }
    println!("{}", result);
    Ok(())
}
