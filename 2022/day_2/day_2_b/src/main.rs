use std::{io::{self, BufRead}, collections::HashMap};

fn main() {
    let mut lines = io::stdin().lock().lines();
    let mut score = 0;
    let mut points = HashMap::new();
    points.insert("Win", 6);
    points.insert("Tie", 3);
    points.insert("Lose", 0);
    let mut mapping = HashMap::new();
    mapping.insert("A", "Rock");
    mapping.insert("B", "Paper");
    mapping.insert("C", "Scissors");
    mapping.insert("X", "Lose");
    mapping.insert("Y", "Tie");
    mapping.insert("Z", "Win");
    while let Some(line) = lines.next() {
        let input_line = match line {
            Ok(val) => val,
            Err(e) => {println!("Error: {}", e); continue;},
        };

        let moves: Vec<&str> = input_line.split(" ").collect();
        let opp = mapping[moves[0]];
        let me = mapping[moves[1]];

        score += points[me];
        if opp == "Scissors" {
            if me == "Win" {
                score += 1;
            } else if me == "Tie" {
                score += 3;
            } else {
                score += 2;
            }
        } else if opp == "Rock" {
            if me == "Win" {
                score += 2;
            } else if me == "Tie" {
                score += 1;
            } else {
                score += 3;
            }
        } else {
            if me == "Win" {
                score += 3;
            } else if me == "Tie" {
                score += 2;
            } else {
                score += 1;
            }
        }



    }
    println!("{}", score);
}
