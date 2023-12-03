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
    if max_x != max_y {
        panic!("Expected grid to be a square!");
    }
    println!("Tree grid is {max_x} x {max_y}");
    //println!("Grid {grid:?}");

    let mut scenic_score = 0;

    // Iterate over each tree
    for x in 0..max_x {
        for y in 0..max_y {
            let score = calc_scenic_score(&grid, x as i32, y as i32);
            //let tree = &grid[(x + (y * max_x)) as usize];
            //println!("score {tree} {x}x{y} {score}");
            if score > scenic_score {
                scenic_score = score;
            }
        }
    }
    println!("scenic score: {scenic_score}");
}

fn calc_scenic_score(grid: &Vec<u8>, x: i32, y: i32) -> i32 {
    let max_x = (grid.len() as f64).sqrt() as i32;
    let n = crawl(grid, max_x, x, y, 0, -1);
    let e = crawl(grid, max_x, x, y, 1, 0);
    let s = crawl(grid, max_x, x, y, 0, 1);
    let w = crawl(grid, max_x, x, y, -1, 0);

    n * e * s * w
}

fn crawl(grid: &[u8], max_x: i32, x: i32, y: i32, x_mod: i32, y_mod: i32) -> i32 {
    let mut score = 0;
    let tallest = &grid[(x + (y * max_x)) as usize];
    for i in 1.. {
        let nx = x + (i * x_mod);
        let ny = y + (i * y_mod);
        if 0 > nx || nx >= max_x || 0 > ny || ny >= max_x {
            return score;
        }
        score += 1;
        let idx = nx + (ny * max_x);
        let val = &grid[idx as usize];
        if val >= tallest {
            return score;
        }
    }
    panic!("loop should never end");
}

fn get_lines(filename: &str) -> impl Iterator<Item=String> {
    let file = File::open(filename).unwrap_or_else(|e| panic!("Cannot open {filename}: {e}"));
    let reader = io::BufReader::new(file);
    reader.lines().enumerate().map(|(i, line)| {
        line.unwrap_or_else(|e| panic!("failed to read line {i}: {e}"))
    })
}
