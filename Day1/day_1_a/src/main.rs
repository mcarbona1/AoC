use std::io::{self, BufRead};
fn main() {
    let mut lines = io::stdin().lock().lines();
    let mut v: Vec<i32> = Vec::new();
    let mut max = 0;
    while let Some(line) = lines.next() {
        let input_line = match line {
            Ok(val) => val,
            Err(e) => {println!("Error: {}", e); continue;},
        };

        if input_line == "" {
            let new_max = v.iter().sum();
            if new_max > max {
                max = new_max;
            }
            v.clear()
        } else {
            v.push(input_line.parse::<i32>().unwrap());
        }
    }

    println!("{}", max);
}
