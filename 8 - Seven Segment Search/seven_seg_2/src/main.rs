use std::{collections::HashMap, fs};

fn count_freq(data: &str) -> HashMap<char, usize> {
    let mut res: HashMap<char, usize> = HashMap::new();

    for word in data.split(" ") {
        for c in word.chars() {
            let init_val = *res.entry(c).or_insert(0);
            res.insert(c, init_val + 1);
        }
    }

    res
}

fn build_trans_vec(observation: &str) -> HashMap<char, char> {
     // translation vector
     let mut translate: HashMap<char, char> = HashMap::new();

    // par fréquence
    let freqs = count_freq(observation);
    let mut g_or_d: Vec<char> = Vec::new();
    let mut a_or_c: Vec<char> = Vec::new();

    for (ch, count) in freqs {
        if count == 6 {
            translate.insert(ch, 'b');
        } else if count == 4 {
            translate.insert(ch, 'e');
        } else if count == 9 {
            translate.insert(ch, 'f');
        } else if count == 7 {
            g_or_d.push(ch);
        } else if count == 8 {
            a_or_c.push(ch);
        }
    }

    let rule4 = observation.split(" ").filter(|i| i.len() == 4).collect::<Vec<&str>>()[0];

    let c = a_or_c.clone().into_iter().filter(|&ch| rule4.contains(ch)).collect::<Vec<char>>()[0];
    translate.insert(c, 'c');
    let a = a_or_c.into_iter().filter(|&ch| ch != c).collect::<Vec<char>>()[0];
    translate.insert(a, 'a');

    let d = g_or_d.clone().into_iter().filter(|&ch| rule4.contains(ch)).collect::<Vec<char>>()[0];
    translate.insert(d, 'd');
    let g = g_or_d.into_iter().filter(|&ch| ch != d).collect::<Vec<char>>()[0];
    translate.insert(g, 'g');

    translate
}

fn main() {
    let mut num_seg: HashMap<usize, Vec<char>> = HashMap::from([
        (0, vec!['a', 'b', 'c', 'e', 'f', 'g']),
        (1, vec!['c', 'f']),
        (2, vec!['a', 'c', 'd', 'e', 'g']),
        (3, vec!['a', 'c', 'd', 'f', 'g']),
        (4, vec!['b', 'c', 'd', 'f']),
        (5, vec!['a', 'b', 'd', 'f', 'g']),
        (6, vec!['a', 'b', 'd', 'e', 'f', 'g']),
        (7, vec!['a', 'c', 'f']),
        (8, vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']),
        (9, vec!['a', 'b', 'c', 'd', 'f', 'g']),
    ]);

    let data = fs::read_to_string("input.txt").unwrap();

    let mut somme: usize = 0;

    for line in data.split("\n") {
        let line = line.trim();
        if line.eq("") { continue; }

        let args = line.split(" | ").collect::<Vec<&str>>();
        let observation = args[0];
        let to_decode = args[1].split(" ").collect::<Vec<&str>>();

        let trans_vec = build_trans_vec(observation);

        let mut number: Vec<usize> = Vec::new();
        for word in to_decode {
            let mut trans_w: Vec<char> = Vec::new();
            for ch in word.chars() {
                trans_w.push(trans_vec.get(&ch).unwrap().clone());
            }
            for (nb, val) in num_seg.iter_mut() {
                let mut val = val.clone();
                val.sort();
                trans_w.sort();

                if val.eq(&trans_w) {
                    number.push(nb.clone());
                    break;
                }
            }
        }
        println!("{:?}", number);
        let number = number.iter().map(|el| el.to_string()).collect::<Vec<String>>().join("").parse::<usize>().unwrap();
        somme += number;
        println!("{}", number);
    }
    println!("Somme: {}", somme);
}
