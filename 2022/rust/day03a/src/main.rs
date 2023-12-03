use std::fs::File;
use std::io;
use std::collections::{HashSet, HashMap};
use std::io::BufRead;
use std::iter::Iterator;


fn main() {
    let input_file = "challenge1.txt";

    // Build lookup table
    let mut lookup_table: HashMap<char, u16> = HashMap::new();
    let offset = 1;
    for (i, c) in ('a' ..= 'z').enumerate() {
        lookup_table.insert(c, (i as u16) + offset);
    }
    let offset = 27;
    for (i, c) in ('A' ..= 'Z').enumerate() {
        lookup_table.insert(c, (i as u16) + offset);
    }
    let lookup_table = lookup_table;

    let file = File::open(input_file).unwrap_or_else(|_| panic!("Could not open '{}'", input_file));

    let mut total = 0;
    for (idx, line) in (io::BufReader::new(file)).lines().flatten().enumerate() {
        if line.is_empty() {
            continue;
        }
        let half = line.len() / 2;
        let (left, right) = line.split_at(half);

        let left_set: HashSet<char> = HashSet::from_iter(left.chars());
        let right_set: HashSet<char> = HashSet::from_iter(right.chars());

        let diff: Vec<char> = left_set.intersection(&right_set).cloned().collect();

        match diff.len() {
            1 => {
                let first = diff.first().unwrap_or_else(|| panic!("Diff was empty"));
                println!("{idx} {first}");
                total += *lookup_table.get(first).get_or_insert(&0);
            },
            n => panic!("Expected diff to be len of 1 but was {}", n),
        }
    }
    println!("\n{total}");

}
