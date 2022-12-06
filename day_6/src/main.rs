use std::{io::{self, BufRead, StdinLock}, collections::{VecDeque, HashSet}};




fn pt1(mut lines: std::io::Lines<StdinLock>) -> u32{
    while let Some(line) = lines.next() {
        let input_line = match line {
            Ok(val) => val,
            Err(e) => {println!("Error: {}", e); continue;},
        };

        let input_line:Vec<char> = input_line.chars().into_iter().collect();
        let mut window: VecDeque<char> = VecDeque::new();
        window.push_back(input_line[0]);
        window.push_back(input_line[1]);
        window.push_back(input_line[2]);
        window.push_back(input_line[3]);
        for i in 4..(input_line.len()) {
            let mut set: HashSet<char> = HashSet::new();
            for item in window.clone().into_iter() {
                set.insert(item.clone());
            }

            if set.len() == 4 {
                return i as u32;
            }

            window.pop_front();
            window.push_back(input_line[i]);
        }
    }

    0
}

fn pt2(mut lines: std::io::Lines<StdinLock>) -> u32{
    while let Some(line) = lines.next() {
        let input_line = match line {
            Ok(val) => val,
            Err(e) => {println!("Error: {}", e); continue;},
        };

        let input_line:Vec<char> = input_line.chars().into_iter().collect();
        let mut window: VecDeque<char> = VecDeque::new();
        for i in 0..14 {
            window.push_back(input_line[i]);
        }

        for i in 14..(input_line.len()) {
            let mut set: HashSet<char> = HashSet::new();
            for item in window.clone().into_iter() {
                set.insert(item.clone());
            }

            if set.len() == 14 {
                return i as u32;
            }

            window.pop_front();
            window.push_back(input_line[i]);
        }
    }

    0
}


fn main() {
    let lines = io::stdin().lock().lines();

    //let state = pt1(lines);
    let state = pt2(lines);

    println!("{}", state);
}
