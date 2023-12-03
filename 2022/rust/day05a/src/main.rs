use std::fs::File;
use std::io;
use std::io::BufRead;
use std::iter::Iterator;
use std::ops::Add;
use regex::Regex;

fn main() {
    let input_file = "challenge1.txt";

    let re_stack_id = Regex::new(r" *([0-9]+) *").unwrap();

    // chunks might be out to split boxes easily
    let mut line_iter = get_lines(input_file);

    let mut crate_lines: Vec<String> = line_iter.by_ref()
        .take_while(|line| !line.is_empty())
        .collect();

    let last_line = crate_lines.pop().unwrap();
    println!("crates {crate_lines:?}");
    let stack_count = re_stack_id.captures_iter(&last_line).count();
    let max_boxes = stack_count * crate_lines.len();

    let mut stacks: Vec<Vec<char>> = (0 .. stack_count)
        .map(|_| Vec::with_capacity(max_boxes))
        .collect();

    // Fill the stacks
    for line in crate_lines.iter().rev() {
        for (stack_idx, c) in line.chars().skip(1).step_by(4).enumerate() {
            if c != ' ' {
                stacks.get_mut(stack_idx).unwrap().push(c);
            }
            println!("stack {stack_idx} {c}");
        }
    }
    println!("filled stacks: {stacks:?}");

    // Process the instructions
    let re_instruction = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for line in line_iter {
        match re_instruction.captures_iter(&line).next() {
            None => panic!("Did not expect bad match for line {}", &line),
            Some(m) => {
                let quantity = &m[1].parse::<usize>().unwrap();
                let from = &m[2].parse::<usize>().unwrap() - 1;
                let to = &m[3].parse::<usize>().unwrap() - 1;
                for _ in 0 .. *quantity {
                    if let Some(from_char) = stacks.get_mut(from)
                        .and_then(|s| s.pop()) {

                        if let Some(dest) = stacks.get_mut(to) {
                            dest.push(from_char);
                        } else {
                            panic!("Failed to get dest stack: {}", &line);
                        }
                    } else {
                        panic!("Failed to get source char: {}", &line);
                    }
                }
            }
        }
    }

    println!("final stacks: {stacks:?}");

    let tops = stacks.iter().map(|s| s.last().unwrap())
        .fold(String::with_capacity(stacks.len()), |acc, item| acc.add(&item.to_string()));
    println!("result: {tops}");
}


fn get_lines(filename: &str) -> impl Iterator<Item=String> {
    let file = File::open(filename).unwrap_or_else(|e| panic!("Cannot open {filename}: {e}"));
    let reader = io::BufReader::new(file);
    reader.lines().enumerate().map(|(i, line)| {
        line.unwrap_or_else(|e| panic!("failed to read line {i}: {e}"))
    })
}
