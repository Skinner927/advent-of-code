use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::iter::Iterator;

const WINDOW: usize = 4;
const MESSAGE: usize = 14;

fn main() {
    let input_file = "challenge.txt";

    // abcd efgh ijkl mnop

    for line in get_lines(input_file).filter(|s|!s.is_empty()).collect::<Vec<String>>() {
        let mut parts = line.split_whitespace();
        let first = parts.next().unwrap();
        let rest = parts.collect::<Vec<&str>>().join(" ");
        println!("{first}");

        // Part 1
        let mut set: HashSet<char> = HashSet::with_capacity(WINDOW);
        let search: Vec<char> = first.chars().collect();
        for x in 0 .. (search.len() - WINDOW) {
            set.clear();
            for v in x .. x+WINDOW {
                set.insert(search[v]);
            }
            if set.len() == WINDOW {
                let winner = WINDOW + x;
                println!("Packcet: {winner} should be {rest} for {first}");
                break;
            }
        }

        // Part 2
        let mut msg_set: HashSet<char> = HashSet::with_capacity(MESSAGE);
        for x in 0 .. (search.len() - MESSAGE) {
            msg_set.clear();
            for v in x .. x+MESSAGE {
                msg_set.insert(search[v]);
            }
            if msg_set.len() == MESSAGE {
                let winner = MESSAGE + x;
                println!("Message: {winner} should be {rest} for {first}");
                break;
            }
        }
    }
}


fn get_lines(filename: &str) -> impl Iterator<Item=String> {
    let file = File::open(filename).unwrap_or_else(|e| panic!("Cannot open {filename}: {e}"));
    let reader = io::BufReader::new(file);
    reader.lines().enumerate().map(|(i, line)| {
        line.unwrap_or_else(|e| panic!("failed to read line {i}: {e}"))
    })
}
