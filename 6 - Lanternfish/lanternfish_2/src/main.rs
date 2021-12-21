use std::fs;

fn main() {
    const NB_DAYS: usize = 256;
    let data = fs::read_to_string("input.txt").expect("Cannot open 'input.txt'");

    let mut days: [usize; 9] = [0; 9];

    for fd in data.split(",") {
        let day: usize = fd.parse().unwrap();
        days[day] += 1;
    }

    for k in 0..NB_DAYS {
        println!("Jour {}({}/8) - Nb poisson {}", k, k%days.len(), days.iter().sum::<usize>());

        let today = k % days.len();
        // adding new babies
        days[(today + 7) % days.len()] += days[today];
    }

    println!("Nombre de poissons final : {}", days.iter().sum::<usize>());
}
