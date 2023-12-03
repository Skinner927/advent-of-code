use std::fs::File;
use std::io;
use std::collections::{HashSet};
use std::io::BufRead;
use std::iter::Iterator;
use std::str::FromStr;

fn main() {
    let input_file = "challenge1.txt";

    let file = File::open(input_file).unwrap_or_else(|_| panic!("Could not open '{input_file}'"));

    let mut total = 0;
    for line in (io::BufReader::new(file)).lines().flatten() {
        if line.is_empty() {
            continue;
        }
        let sets : Vec<HashSet<i32>> = line.split(&",").map(|side: &str| {
            let range: Vec<i32> = side.split(&"-").map(|x| i32::from_str(x).unwrap()).collect();
            match range.len() {
                2 => {
                    HashSet::from_iter(range[0] ..= range[1])
                },
                n => panic!("Expected len of 2, got {n}"),
            }
        }).collect();
        let sets_len = sets.len();
        if sets_len != 2 {
            panic!("Should have 2 sets, have {sets_len}")
        }

        let left = &sets[0];
        let right = &sets[1];

        let joined: HashSet<&i32> = left.intersection(right).collect();
        if joined.len() == left.len() || joined.len() == right.len() {
            println!("Overlap {line}");
            total += 1;
        }
    }

    println!("\ntotal\n\n{total}");
}
