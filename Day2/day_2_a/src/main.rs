use std::{io::{self, BufRead}, collections::HashMap};

fn main() {
    let mut lines = io::stdin().lock().lines();
    let mut score = 0;
    let win = 6;
    let tie = 3;
    let mut points = HashMap::new();
    points.insert("Rock", 1);
    points.insert("Paper", 2);
    points.insert("Scissors", 3);
    let mut mapping = HashMap::new();
    mapping.insert("A", "Rock");
    mapping.insert("B", "Paper");
    mapping.insert("C", "Scissors");
    mapping.insert("X", "Rock");
    mapping.insert("Y", "Paper");
    mapping.insert("Z", "Scissors");
    while let Some(line) = lines.next() {
        let input_line = match line {
            Ok(val) => val,
            Err(e) => {println!("Error: {}", e); continue;},
        };

        let moves: Vec<&str> = input_line.split(" ").collect();
        let opp = mapping[moves[0]];
        let me = mapping[moves[1]];

        score += points[me];
        if opp == me {
            score += tie;
        } else if opp == "Scissors" {
            if me == "Rock" {
                score += win;
            }                 
        } else if opp == "Rock" {
            if me == "Paper" {
                score += win;
            }                 
        } else {
            if me == "Scissors" {
                score += win;
            }
        }



    }
    println!("{}", score);
}
