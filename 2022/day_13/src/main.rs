use std::{io::{self, BufRead}, collections::VecDeque};
use json::{self, JsonValue, array};

fn parse_packets(first: JsonValue, second: JsonValue) -> Option<bool> {
    let mut first = first.members();
    let mut second = second.members();

    while let Some(first_item) = first.next() {
        let mut second_item = match second.next() {
            Some(item) => item.clone(),
            None => {return Some(false);},
        };
        let mut first_item = first_item.clone();

        if first_item.is_number() && second_item.is_number() {
            let first_num = first_item.as_i32().unwrap();
            let second_num = second_item.as_i32().unwrap();

            if first_num < second_num {
                return Some(true);
            } else if first_num == second_num {
                continue;
            } else {
                return Some(false);
            }
        } else if first_item.is_array() && second_item.is_array() {
            match parse_packets(first_item.clone(), second_item.clone()) {
                Some(val) => {return Some(val)},
                None => {continue;},
            }
        } else {
            if first_item.is_number() {
                first_item = array![first_item.clone()];
            }

            if second_item.is_number() {
                second_item = array![second_item.clone()];
            }
            match parse_packets(first_item.clone(), second_item.clone()) {
                Some(val) => {return Some(val)},
                None => {continue;},
            }
        }

    }
    match second.next() {
        Some(_val) => Some(true),
        None => None,
    }
}

fn pt1(mut packets: VecDeque<String>) -> i32 {
    let mut counter = 1;
    let mut sum = 0;
    while packets.len() > 0 {
        let first = json::parse(&packets.pop_front().unwrap()).unwrap();
        let second = json::parse(&packets.pop_front().unwrap()).unwrap();

        if let Some(result) = parse_packets(first, second) {
            if result {
                sum += counter;
            }
        };
        counter += 1;
    }

    sum
}

fn pt2(mut packets: VecDeque<String>) -> usize {
    let mut cont = true;
    let mut key = 1;
    packets.push_front("[[2]]".to_string());
    packets.push_front("[[6]]".to_string());
    while cont {
        cont = false;
        for i in 0..(packets.len()-1) {
            let first = json::parse(&packets[i]).unwrap();
            let second = json::parse(&packets[i+1]).unwrap();
            if let Some(result) = parse_packets(first, second) {
                if !result {
                    let temp = packets[i].clone();
                    packets[i] = packets[i+1].clone();
                    packets[i+1] = temp;
                    cont = true;
                }
            };
        }
    }

    for i in 0..packets.len() {
        if packets[i] == "[[2]]" {
            key *= i + 1;
        } else if packets[i] == "[[6]]" {
            key *= i + 1;
        }
    }

    key
}

fn main() {
    let mut lines = io::stdin().lock().lines();
    let mut packets: VecDeque<String> = VecDeque::new();
    while let Some(line) = lines.next() {
        let input_line = match line {
            Ok(val) => val,
            Err(e) => {println!("Error: {}", e); continue;},
        };
        if input_line == "" {
            continue;
        }
        packets.push_back(input_line.clone());
    }

    let sum = pt1(packets.clone());
    let key = pt2(packets.clone());
    println!("{}", sum);
    println!("{}", key);



}
