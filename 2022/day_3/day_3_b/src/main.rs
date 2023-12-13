use std::{io::{self, BufRead}, collections::HashSet};

fn main() {
    let mut lines = io::stdin().lock().lines();
    let mut score = 0;
    while let Some(line) = lines.next() {
        let mut sack1: HashSet<char> = HashSet::new();
        let mut sack2: HashSet<char> = HashSet::new();
        let mut sack3: HashSet<char> = HashSet::new();
        let input_line = match line {
            Ok(val) => val,
            Err(e) => {println!("Error: {}", e); continue;},
        };

        let input_line_2 = lines.next().unwrap().unwrap();
        let input_line_3 = lines.next().unwrap().unwrap();

        for item in input_line.chars() {
            sack1.insert(item);
        }

        for item in input_line_2.chars() {
            sack2.insert(item);
        }

        for item in input_line_3.chars() {
            sack3.insert(item);
        }
        let common: HashSet<char> = sack1.intersection(&sack2).cloned().collect();
        let common: HashSet<char> = sack3.intersection(&common).cloned().collect();

        for item in common {
            if sack3.contains(&item) {
                if item >= 'a' && item <= 'z' {
                    score += item as u32 - 'a' as u32 + 1;
                } else {
                    score += item as u32 - 'A' as u32 + 1 + 26;
                }
            }
        }

    }

    println!("{}", score);
}
