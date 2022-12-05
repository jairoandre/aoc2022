use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::Stack;
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

struct SupplyStacks(Vec<Stack<T>>);

pub fn create_stack(args: &[&str]) -> Stack<&str> {
    let mut stack = Stack::new();
    for arg in args {
        stack.push(arg);
    }
    stack
}

#[allow(dead_code)]
pub fn solve() -> Result<(), Box<dyn std::error::Error>> {

    let mut supply_stacks = SupplyStacks(
        vec![
            create_stack(&["W", "B", "D", "N", "C", "F", "J"]),
            create_stack(&["P", "Z", "V", "Q", "L", "S", "T"]),
            create_stack(&["P", "Z", "B", "G", "J", "T"]),
            create_stack(&["D", "T", "L", "J", "Z", "B", "H", "C"]),
            create_stack(&["G", "V", "B", "J", "S"]),
            create_stack(&["P", "S", "Q"]),
            create_stack(&["B", "V", "D", "F", "L", "M", "P", "N"]),
            create_stack(&["P", "S", "M", "F", "B", "D", "L", "R"]),
            create_stack(&["V", "D", "T", "R"]),
        ].into_iter().collect::<Stack<&str>>()
    );

    let file = File::open("src/inputs/day5.txt")?;
    let reader = BufReader::new(file);

    let re = Regex::new(r"\d+").unwrap();

    for line in reader.lines() {
        let mut numbers = Vec::new();
        let ss = format!("{}", line?);
        for caps in re.captures_iter(&ss) {
            numbers.push(caps[0].parse::<i32>().unwrap())
        }
        let qtd = numbers[0];
        let from = numbers[1] - 1;
        let to = numbers[2] - 1;
        for _ in 0..qtd {
            let v = supply_stacks.0[from].pop();
            supply_stacks.0[to].push(v);
        }
    }
    Ok(())
}
