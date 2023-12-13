use std::io::{self, BufRead};
fn main() {
    let mut lines = io::stdin().lock().lines();
    let mut v: Vec<i32> = Vec::new();
    let mut elves: Vec<i32> = Vec::new();
    while let Some(line) = lines.next() {
        let input_line = match line {
            Ok(val) => val,
            Err(e) => {println!("Error: {}", e); continue;},
        };

        if input_line == "" {
            let new_max = v.iter().sum();
            elves.push(new_max);
            v.clear()
        } else {
            v.push(input_line.parse::<i32>().unwrap());
        }
    }
    elves.sort();
    println!("{}", (elves[elves.len()-1] + elves[elves.len()-2] + elves[elves.len()-3]))
}
