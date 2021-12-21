use std::fs;

struct LanternFish {
    days_left: usize,
}

impl LanternFish {
    fn new(value: usize,) -> Self {
        Self {
            days_left: value,
        }
    }

    fn breed(&mut self) {
        // add the new fish to the heap
        self.days_left = 6;
    }
}

fn main() {
    const NB_DAYS: usize = 80;
    let data = fs::read_to_string("input.txt").expect("Cannot open 'input.txt'");

    let mut fishes: Vec<LanternFish> = Vec::new();
    for fd in data.split(",") {
        let days: usize = fd.parse().unwrap();
        fishes.push(LanternFish::new(days));
    }

    let mut fish_to_add: Vec<LanternFish> = Vec::new();
    for _ in 0..NB_DAYS {
        for fish in fishes.iter_mut() {
            // breeding time
            if fish.days_left == 0 {
                fish.breed();
                fish_to_add.push(LanternFish::new(8));
            } else {
                fish.days_left -= 1;
            }
        }
        fishes.append(&mut fish_to_add);
        fish_to_add.clear();
    }

    println!("There is {} fishes", fishes.len());
}
