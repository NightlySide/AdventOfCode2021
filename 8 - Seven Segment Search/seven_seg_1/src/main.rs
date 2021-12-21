use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();

    let mut count: usize = 0;
    for line in data.split("\n") {
        if line.eq("") { continue; }
        
        let line = line.trim();
        let args = line.split(" | ").collect::<Vec<&str>>();
        let nbs = args[1].split(" ").collect::<Vec<&str>>();

        for nb in nbs {
            if nb.len() == 2 || nb.len() == 3 || nb.len() == 4 || nb.len() == 7 {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
