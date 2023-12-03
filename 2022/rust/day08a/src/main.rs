use std::fs::File;
use std::io;
use std::io::BufRead;
use std::iter::Iterator;

fn main() {
    let input_file = "input.txt";

    let mut grid: Vec<u8> = Vec::new();

    let mut max_x = 0;
    let mut max_y = 0;
    for (i, line) in get_lines(input_file).enumerate() {
        if i == 0 {
            max_x = line.len();
        }
        max_y += 1;
        grid.extend(line.chars().map(|c| (c as u8) - b'0'));
    }
    println!("Tree grid is {max_x} x {max_y}");
    //println!("Grid {grid:?}");

    let mut visible: Vec<u32> = (0..(max_x * max_y)).map(|_| 0).collect();

    // This goes north to south and south to north
    for x in 0..max_x {
        let mut tallest_ns: i32 = -1;
        let mut tallest_sn: i32 = -1;
        for y in 0..max_y {
            let idx_ns = x + (y * max_x);
            let idx_sn = x + ((max_y - 1 - y) * max_x);
            let val_ns = grid[idx_ns] as i32;
            let val_sn = grid[idx_sn] as i32;
            //println!("idx_ns {idx_ns} val_ns {val_ns}");
            //println!("  idx_sn {idx_sn} val_sn {val_sn}");

            if val_ns > tallest_ns {
                //println!("NS: {x} {y} = {val_ns}");
                tallest_ns = val_ns;
                visible[idx_ns] = 1;
            }
            if val_sn > tallest_sn {
                //println!("SN: {x} {y} = {val_sn}");
                tallest_sn = val_sn;
                visible[idx_sn] = 1;
            }
        }
    }

    // this goes east to west and west to east
    for y in 0..max_y {
        let mut tallest_ew: i32 = -1;
        let mut tallest_we: i32 = -1;
        for x in 0..max_x {
            let idx_ew = x + (y * max_x);
            let idx_we = (max_x - x - 1) + (y * max_x);
            let val_ew = grid[idx_ew] as i32;
            let val_we = grid[idx_we] as i32;
            //println!("idx_ew {idx_ew} val_ew {val_ew}");
            //println!("  idx_we {idx_we} val_we {val_we}");

            if val_ew > tallest_ew {
                //println!("EW: {x} {y} = {val_ew}");
                tallest_ew = val_ew;
                visible[idx_ew] = 1;
            }
            if val_we > tallest_we {
                //println!("WE: {x} {y} = {val_we}");
                tallest_we = val_we;
                visible[idx_we] = 1;
            }
        }
    }

    let total: u32 = visible.iter().sum();
    println!("visible: {total}");
}

fn get_lines(filename: &str) -> impl Iterator<Item=String> {
    let file = File::open(filename).unwrap_or_else(|e| panic!("Cannot open {filename}: {e}"));
    let reader = io::BufReader::new(file);
    reader.lines().enumerate().map(|(i, line)| {
        line.unwrap_or_else(|e| panic!("failed to read line {i}: {e}"))
    })
}
