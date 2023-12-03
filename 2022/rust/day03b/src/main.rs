use std::fs::File;
use std::io;
use std::collections::{HashSet, HashMap};
use std::io::BufRead;
use std::iter::Iterator;

fn find_group_badge(groups: &[HashSet<char>]) -> Option<char> {
    let first = groups.first()?;
    let created = groups.iter().skip(1).fold(first.clone(), |acc: HashSet<char>, group: &HashSet<char>| {
        acc.intersection(group).cloned().collect()
    });

    created.into_iter().next()
}

fn main() {
    let input_file = "challenge1.txt";

    // Build lookup table
    let mut lookup_table: HashMap<char, u16> = HashMap::new();
    let offset = 1;
    for (i, c) in ('a'..='z').enumerate() {
        lookup_table.insert(c, (i as u16) + offset);
    }
    let offset = 27;
    for (i, c) in ('A'..='Z').enumerate() {
        lookup_table.insert(c, (i as u16) + offset);
    }
    let lookup_table = lookup_table;

    let file = File::open(input_file).unwrap_or_else(|_| panic!("Could not open '{}'", input_file));

    let mut total = 0;
    let mut sub_idx = 0;
    let mut groups: Vec<HashSet<char>> = vec![HashSet::new(), HashSet::new(), HashSet::new()];
    for (i, line) in (io::BufReader::new(file)).lines().flatten().enumerate() {
        if line.is_empty() {
            // Shouldn't need this
            sub_idx += 1;
            continue;
        }
        let group_id = (i - sub_idx) / 3;
        let idx = (i - sub_idx) % 3;
        println!("group_id={group_id} idx={idx} i={i} sub_idx={sub_idx}");

        // fill the current hashset
        groups[idx] = HashSet::from_iter(line.chars());

        if 2 == idx {
            println!("Dumping {groups:?}");
            if let Some(badge) = find_group_badge(&groups) {
                println!("got badge {badge}");
                total += *lookup_table.get(&badge).get_or_insert(&0);
            } else {
                println!("failed to hash group");
            }

            // Clear
            {
                for h in &mut groups {
                    h.clear()
                }
            }
        }


    }
    println!("\n{total}");
}
