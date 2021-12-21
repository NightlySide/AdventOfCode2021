use std::fs;

fn main() {
    let input_file = "input.txt";
    let data = fs::read_to_string(input_file).unwrap();

    let mut count: i32 = 0;
    let mut p_1: i32 = -1;
    let mut p_2: i32 = -1;
    let mut p_3: i32 = -1;

    for line in data.split("\n") {
        let line = line.trim();
        if !line.eq("") {
            let current = line.parse::<i32>().unwrap();

            if p_3 != -1 {
                if current + p_1 + p_2 > p_1 + p_2 + p_3 {
                    count += 1;
                }
            }
            p_3 = p_2;
            p_2 = p_1;
            p_1 = current;
        }
    }

    println!("Nb de mesures croissantes : {}", count);
}
