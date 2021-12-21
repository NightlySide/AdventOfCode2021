use std::{fs, collections::{LinkedList, HashMap}};

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();

    let corresponding: HashMap<char, char> = HashMap::from([
        ('(', ')'),
        (')', '('),
        ('[', ']'),
        (']', '['),
        ('{', '}'),
        ('}', '{'),
        ('<', '>'),
        ('>', '<'),
    ]);
    let corrupted_score_map = HashMap::from([
        (')', 3),
        (']', 57),
        ('}', 1197),
        ('>', 25137),
    ]);
    let closing_score_map = HashMap::from([
        (')', 1),
        (']', 2),
        ('}', 3),
        ('>', 4),
    ]);
    

    let mut corrupted_total_score = 0;
    let mut incomplete_total_score: Vec<usize> = Vec::new();
    for line in data.split("\n") {
        let line = line.trim();
        if line.eq("") { continue; }

        let mut stack: LinkedList<char> = LinkedList::new();

        // corrupted lines
        let mut corrupted_score = 0;
        for ch in line.chars() {
            match ch {
                '(' | '[' | '<' | '{' => stack.push_back(ch),
                ')' | ']' | '>' | '}' => {
                    let last = stack.pop_back().unwrap();
                    let to_test = *corresponding.get(&last).unwrap();

                    if to_test != ch {
                        println!("Expected '{}' got '{}' instead", to_test, ch);
                        corrupted_score = corrupted_score_map[&ch];
                        break;
                    }
                }
                _ => println!("Pattern not found: '{}'", ch),
            }
        }
        corrupted_total_score += corrupted_score;

        // probably a incomplete line
        if corrupted_score == 0 && !stack.is_empty() {
            let mut incom_score = 0;
            while !stack.is_empty() {
                let next = stack.pop_back().unwrap();
                incom_score *= 5;
                incom_score += closing_score_map[&corresponding[&next]];
            }
            incomplete_total_score.push(incom_score);
        }
    }

    incomplete_total_score.sort();
    let incomplete_total_score = incomplete_total_score.get(incomplete_total_score.len() / 2).unwrap();

    println!("Corrupted score: {}", corrupted_total_score);
    println!("Incomplete score: {}", incomplete_total_score);
}
