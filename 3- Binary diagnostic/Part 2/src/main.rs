use std::fs;

fn most_byte(data: &Vec<&str>, index: usize) -> usize {
    let mut occ = 0;

    data.iter().for_each(|line| {
        if line.chars().nth(index).unwrap().eq(&'1') {
            occ += 1;
        }
    });
            
    if 2 * occ >= data.len() { return 1; }
    0
}

fn less_byte(data: &Vec<&str>, index: usize) -> usize {
    let mut occ = 0;

    data.iter().for_each(|line| {
        if line.chars().nth(index).unwrap().eq(&'1') {
            occ += 1;
        }
    });
            
    if 2 * occ < data.len() { return 1; }
    0
}

fn main() {
    let input_file = "input.txt";
    let data = fs::read_to_string(input_file).unwrap();

    let mut init_potential: Vec<&str> = vec![];
    data.split("\n").for_each(|line| {
        let line = line.trim();
        if !line.eq("") {
            init_potential.push(line);
        }
    });
    

    // computing oxygen & co2
    let mut potential = init_potential.clone();
    let mut index: usize = 0;
    while potential.len() > 1 {
        let bit: u32 = most_byte(&potential, index) as u32;
        potential = potential.iter()
            .filter(|&&line| {
                line.chars().nth(index).eq(&char::from_digit(bit, 10))
            })
            .map(|line| *line)
            .collect();
        println!("OXY: Found {} potentials", potential.len());
        
        index += 1;
    }
    let oxygen = *potential.get(0).unwrap();
    println!("Found oxygen: {}", oxygen);

    let mut potential = init_potential.clone();
    let mut index: usize = 0;
    while potential.len() > 1 {
        let bit: u32 = less_byte(&potential, index) as u32;
        potential = potential.iter()
            .filter(|&&line| {
                line.chars().nth(index).eq(&char::from_digit(bit, 10))
            })
            .map(|line| *line)
            .collect();
        println!("CO2: Found {} potentials", potential.len());
        
        index += 1;
    }
    let co2 = *potential.get(0).unwrap();

    // showing results    

    println!("Binaire : g: {} e: {}", oxygen, co2);
    let oxygen: usize = usize::from_str_radix(&oxygen, 2).unwrap();
    let co2: usize = usize::from_str_radix(&co2, 2).unwrap();
    println!("Decimal : g: {} e: {}", oxygen, co2);
    println!("Multiplication : {}", oxygen * co2);
}
