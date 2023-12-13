use std::{io::{self, BufRead}, collections::HashSet};

fn main() {
    let mut lines = io::stdin().lock().lines();
    let mut score = 0;
    while let Some(line) = lines.next() {
        let mut sack1: HashSet<char> = HashSet::new();
        let mut sack2: HashSet<char> = HashSet::new();
        let input_line = match line {
            Ok(val) => val,
            Err(e) => {println!("Error: {}", e); continue;},
        };

        let middle = input_line.len()/2;
        for item in input_line[..middle].chars() {
            sack1.insert(item);
        }

        for item in input_line[middle..].chars() {
            sack2.insert(item);
        }

        let common: HashSet<char> = sack1.intersection(&sack2).into_iter().cloned().collect();

        for item in common.into_iter() {
            if item >= 'a' && item <= 'z' {
                score += item as u32 - 'a' as u32 + 1;
            } else {
                score += item as u32 - 'A' as u32 + 1 + 26;
            }
        }

    }

    println!("{}", score);
}
