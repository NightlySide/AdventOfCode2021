use std::fs;

fn main() {
    let input_file = "input.txt";
    let data = fs::read_to_string(input_file).unwrap();

    let mut bits: Vec<i32> = vec![0; 12];
    let mut nb_lines: i32 = 0;

    for line in data.split("\n") {
        let line = line.trim();

        if !line.eq("") {
            nb_lines += 1;

            bits = bits
                .iter()
                .enumerate()
                .map(|(idx, &el)| {
                    if line.chars().nth(idx).unwrap().eq(&'1') { return el + 1; }
                    el
                }).collect();
        }
    }

    let mut gamma: Vec<usize> = vec![0; 12];
    let mut epsilon: Vec<usize> = vec![0; 12];

    bits.iter().enumerate()
        .for_each(|(idx, &el)| {
            if (el as f32) >= (nb_lines as f32) / 2.0 {
                gamma[idx] = 1;
                epsilon[idx] = 0;
            } else {
                gamma[idx] = 0;
                epsilon[idx] = 1;
            }
        });

    let gamma: String = gamma.iter().map(|&val| val.to_string()).collect();
    let epsilon: String = epsilon.iter().map(|&val| val.to_string()).collect();
    println!("Binaire : g: {} e: {}", gamma, epsilon);

    let gamma: usize = usize::from_str_radix(&gamma, 2).unwrap();
    let epsilon: usize = usize::from_str_radix(&epsilon, 2).unwrap();
    println!("Decimal : g: {} e: {}", gamma, epsilon);
    println!("Multiplication : {}", gamma * epsilon);
}
