use std::{fs, cmp::{min, max}};

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let mut pos: Vec<usize> = Vec::new();

    for pd in data.split(",") {
        let p: usize = pd.parse().unwrap();
        pos.push(p);
    }

    let size = pos.iter().max().unwrap();
    let min_fuel: usize = (0..*size).map(|final_pos| {
        pos.iter().map(|&start_pos| max(final_pos, start_pos) - min(final_pos, start_pos)).sum()
    }).min().unwrap();

    println!("Min fuel : {}", min_fuel);
}
