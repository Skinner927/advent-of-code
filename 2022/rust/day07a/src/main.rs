use std::collections::{HashMap};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::iter::Iterator;
use regex::Regex;


struct Node {
    parent_idx: Option<usize>,
    children_idx: Vec<usize>,

    name: String,
    my_idx: usize,
    my_size: usize,
    calc_size: Option<usize>,
}

struct Arena {
    nodes: Vec<Node>,
}

impl Arena {
    fn get_child(&mut self, parent_idx: usize, name: &str) -> usize {
        let parent = self.nodes.get(parent_idx);
        if parent.is_none() {
            panic!("Invalid parent {parent_idx} for {name}")
        }
        let parent = parent.unwrap();

        for child_idx in &parent.children_idx {
            if let Some(child) = self.nodes.get(*child_idx) {
                if child.name == name {
                    return *child_idx;
                }
            }
        }

        let new_idx = self.nodes.len();
        self.nodes.push(Node {
            parent_idx: Some(parent_idx),
            children_idx: Vec::new(),
            name: String::from(name),
            my_idx: new_idx,
            my_size: 0,
            calc_size: None,
        });

        let parent = self.nodes.get_mut(parent_idx).unwrap();
        parent.children_idx.push(new_idx);

        new_idx
    }

    fn get_parent(&mut self, current_idx: usize) -> Option<usize> {
        if let Some(current) = self.nodes.get(current_idx) {
            return current.parent_idx;
        }
        None
    }

    fn set_size(&mut self, target_idx: usize, new_size: usize) {
        if let Some(current) = self.nodes.get_mut(target_idx) {
            if current.my_size != 0 {
                panic!("Node {target_idx} {} already set", current.name);
            }
            current.my_size = new_size;
        }
    }

    // fn add_size(&mut self, target_idx: usize,  new_size: usize) {
    //     if let Some(current) = self.nodes.get_mut(target_idx) {
    //         current.my_size += new_size;
    //     }
    // }

    // fn get_node(&mut self, target_idx: usize) -> &Node {
    //     self.nodes.get(target_idx).unwrap_or_else(|| panic!("Unknown Node {target_idx}"))
    // }
}

fn main() {
    let input_file = "challenge.txt";

    let mut arena = Arena {
        nodes: vec![Node {
            parent_idx: None,
            children_idx: Vec::new(),
            name: String::from("/"),
            my_idx: 0,
            my_size: 0,
            calc_size: None,
        }],
    };
    let mut current: usize = 0;

    let re_size = Regex::new(r"^(\d+) +([^ ]+) *$").unwrap();
    for line in get_lines(input_file).skip(1).filter(|s| !s.is_empty()).collect::<Vec<String>>() {
        println!("{line}");
        if line.starts_with("$ cd ..") {
            println!("> .. parent dir");
            current = arena.get_parent(current).unwrap_or(0);
            continue;
        } else if line.starts_with("$ cd ") {
            let dir_name = line.strip_prefix("$ cd ").unwrap();
            println!("> cd into {dir_name}");
            current = arena.get_child(current, dir_name);
            continue;
        } else if line.starts_with("$ ls") || line.starts_with("dir ") {
            // ignore
            continue;
        }

        // else it's gotta be a number
        if let Some(m) = re_size.captures(&line) {
            let size = m.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let name = m.get(2).unwrap().as_str();
            println!("> {size}  {name}");
            let child = arena.get_child(current, name);
            arena.set_size(child, size);
        } else {
            panic!("Unexpected line format {line}");
        }
    }

    println!("---------------");

    let mut calc_size: HashMap<usize, usize> = HashMap::with_capacity(arena.nodes.len());
    loop {
        let mut did_mute = false;
        for node in &arena.nodes {
            if calc_size.get(&node.my_idx).is_none() {
                if node.children_idx.is_empty() {
                    calc_size.insert(node.my_idx, node.my_size);
                    did_mute = true;
                    continue;
                }
                if node.children_idx.iter().all(|i| calc_size.get(i).is_some()) {
                    let total: usize = node.children_idx.iter().map(|i| calc_size.get(i).unwrap()).sum();
                    calc_size.insert(node.my_idx, total);
                    did_mute = true;
                }
            }
        }
        if !did_mute {
            break;
        }
    }

    let mut total = 0;
    for node in &arena.nodes {
        if node.children_idx.is_empty() {
            // dir only
            continue;
        }
        let my_size = *calc_size.get(&node.my_idx).unwrap();
        if my_size <= 100000 {
            total += my_size;
        }
    }

    // Part 1
    println!("win {total}");

    let root_size = calc_size.get(&0).unwrap();

    let space = 70000000 - root_size;
    let need = 30000000;
    println!("root is {root_size} which leaves {space}");

    let mut dir_sizes: Vec<usize> = arena.nodes.iter()
        .filter(|n| !n.children_idx.is_empty())
        .map(|n| *calc_size.get(&n.my_idx).unwrap())
        .collect();
    dir_sizes.sort();
    for d in &dir_sizes {
        if (space + d) > need {
            // Part2
            println!("d {d}");
            break;
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
