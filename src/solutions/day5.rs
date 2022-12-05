use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::Stack;

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

struct SupplyStacks {
    crates : Vec<Stack<T>>
}

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
        crates = vec![
            vec!["W", "B", "D", "N", "C", "F", "J"].into_iter().collect::<Stack<&str>>(),
            vec!["P", "Z", "V", "Q", "L", "S", "T"].into_iter().collect::<Stack>

        ].into_iter().collect::<Stack<&str>>()
    );


    let file = File::open("src/inputs/day4.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let ss = format!("{}", line?);
    }
    println!("{total} | {total_2}");
    Ok(())
}
