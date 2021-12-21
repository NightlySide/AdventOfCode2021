use std::fs;

fn main() {
    let input_file = "input.txt";
    let data = fs::read_to_string(input_file).unwrap();

    let mut count: i32 = 0;
    let mut previous: i32 = -1;

    for line in data.split("\n") {
        let line = line.trim();
        if !line.eq("") {
            let current = line.parse::<i32>().unwrap();

            if previous != -1 {
                if current > previous {
                    count += 1;
                }
            }
            previous = current;
        }
    }

    println!("Nb de mesures croissantes : {}", count);
}
