use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
use std::collections::HashMap;

struct Directories {
    list: Vec<String>
}

impl Display for Directories {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for s in self.list.iter() {
            write!(f, " {}>", s)?;
        }
        write!(f, "")
    }
}

#[allow(dead_code)]
pub fn solve() -> Result<(), Box<dyn std::error::Error>> {

    let file = File::open("src/inputs/day7.txt")?;
    let reader = BufReader::new(file);

    let cd = Regex::new(r"\$ cd (.+)").unwrap();
    let ls = Regex::new(r"\$ ls").unwrap();
    let dr = Regex::new(r"\dir (.+)").unwrap();
    let fr = Regex::new(r"(\d+)\s(.+)").unwrap();

    let mut directory: Directories = Directories {
        list: Vec::new(),
    };

    let mut map: HashMap<String, i32> = HashMap::new();
    let mut current_dir: String = String::from("");

    for line in reader.lines() {
        let ss = format!("{}", line?);
        match cd.captures(&ss) {
            Some(c) => {
                let name = c.get(1).map_or("", |m| m.as_str());
                if name == ".." {
                    let previous_dir = directory.list.pop().unwrap();
                    let total_size_previous = map.get(previous_dir.as_str()).unwrap();
                    current_dir = String::from(directory.list.get(directory.list.len()-1).unwrap());
                    let current_size = map.get(current_dir.as_str()).unwrap();
                    map.insert(current_dir.clone(), current_size + total_size_previous);

                } else {
                    directory.list.push(name.to_owned());
                    map.insert(name.to_owned(), 0);
                    current_dir = String::from(name);
                }
                continue;
            }
            None => {}
        }
        match ls.captures(&ss) {
            Some(_) => {
                continue;
            }
            _ => {}
        }
        match dr.captures(&ss) {
            Some(c) => {
                let _name = c.get(1).map_or("", |m| m.as_str());
                continue;
            }
            _ => {}
        }
        match fr.captures(&ss) {
            Some(c) => {
                let size = c.get(1).map_or(0, |m| m.as_str().parse::<i32>().unwrap());
                let _name = c.get(2).map_or("", |m| m.as_str());
                let current_size = map.get(current_dir.as_str()).unwrap();
                map.insert(current_dir.to_owned(), current_size + size);
                continue;
            }
            _ => {}
        }
    }

    let mut total = 0;
    for (k, v) in map.iter() {
        if *v < 100_000 && k != "/" {
            total += *v;
            println!("included {v}: {k}");
        }
    }
    println!("Total: {total}");

    Ok(())
}
