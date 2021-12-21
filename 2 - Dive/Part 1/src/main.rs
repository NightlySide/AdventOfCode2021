use std::fs;

fn main() {
    let input_file = "input.txt";
    let data = fs::read_to_string(input_file).unwrap();

    let mut h: i32 = 0;
    let mut v: i32 = 0;

    for line in data.split("\n") {
        let line = line.trim();
        let args: Vec<&str> = line.split(" ").collect();

        if !line.eq("") {
            let amount = args[1].parse::<i32>().unwrap();
            if args[0].eq("forward") {
                h += amount;
            } else if args[0].eq("up")  {
                v -= amount;
            } else {
                v += amount;
            }
        }
    }

    println!("Position finale : h: {} v: {}", h, v);
    println!("Multiplication : {}", h*v);
}
