use std::fs::File;
use std::io::{BufRead, BufReader};

fn c_value(c : char) -> u32 {
    let i = (c as u8 - 65) + 27;
    if i <= 52 {
        return i as u32;
    }
    (i - 58) as u32
}

#[allow(dead_code)]
pub fn solve() -> Result<(), Box<dyn std::error::Error>> {

    let file = File::open("src/inputs/day3.txt")?;
    let reader = BufReader::new(file);

    let mut total = 0;

    for line in reader.lines() {
        let ss = format!("{}", line?);
        let s = ss.as_bytes();
        let half_index = s.len() / 2;
        'outer: for i in 0..half_index {
            for j in half_index..s.len() {
                if s[i] == s[j] {
                    let c = s[i] as char;
                    let v = c_value(c);
                    total += v;
                    break 'outer
                }
            }
        }
    }
    println!("score : {}", total);
    Ok(())
}

fn score_buffer(str_buffer: &Vec<Vec<u8>>) -> u32 {
    let vec_1 = &str_buffer[0];
    for i in 0..vec_1.len() {
        let vec_2 = &str_buffer[1];
        for j in 0..vec_2.len() {
            if vec_1[i] == vec_2[j] {
                let vec_3 = &str_buffer[2];
                for k in 0..vec_3.len() {
                    if vec_2[j] == vec_3[k] {
                        let c = vec_3[k] as char;
                        let v = c_value(c);
                        return v;
                    }

                }
            }
        }
    }
    return 0
}

pub fn solve_2() -> Result<(), Box<dyn std::error::Error>> {

    let file = File::open("src/inputs/day3.txt")?;
    let reader = BufReader::new(file);

    let mut total = 0;
    let mut line_count = 0;
    let mut str_buffer: Vec<Vec<u8>> = vec![vec![]; 3];

    for line in reader.lines() {
        let ss = format!("{}", line?);
        let s_vec = ss.as_bytes().to_vec();
        if line_count < 3 {
            str_buffer[line_count] = s_vec;
            line_count += 1;
        } else {
            total += score_buffer(&str_buffer);
            str_buffer[0] = s_vec;
            line_count = 1;
        }
    }
    total += score_buffer(&str_buffer);
    println!("score : {}", total);
    Ok(())
}
