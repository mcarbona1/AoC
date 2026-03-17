use std::{collections::HashSet, fs};

use clap::Parser;

// Parser struct definition.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = '1', long)]
    part1: bool,

    #[arg(short = '2', long)]
    part2: bool,

    #[arg(short, long)]
    test: bool,

    #[arg(short, long)]
    input: bool,
}

// [328, 222327] -> [1010, 222222]
fn clamp(str_num: &str, len: u32, start: bool) -> u64 {
    let num = str_num.parse::<u64>().unwrap();

    for i in if start {
        1..num
    } else {
        (num / 10_u64.pow(len - 1))..num
    } {
        let mut candidate = 0;
        for _ in 0..len {
            candidate = candidate * 10_u64.pow(len - 1) + i
        }

        if start && candidate > num {
            return candidate;
        } else if !start && num < candidate {
            return candidate;
        }
    }

    0
}

fn find_invalid(start: &str, stop: &str, pt1: bool) -> u64 {
    let mut count = 0;
    let mut seen = HashSet::new();

    for i in clamp(start, 2, false)..clamp(stop, 2, true) {}

    count
}

// fn find_invalid(start: &str, stop: &str, pt1: bool) -> u64 {
//     let mut count = 0;
//     let mut seen = HashSet::new();
//     let (str, end) = (start.parse::<u64>().unwrap(), stop.parse::<u64>().unwrap());

//     let mut stack = vec![(0 as u64, 1 as u64, 1 as u64)];

//     while !stack.is_empty() {
//         let Some((curr, mag, mult)) = stack.pop() else {
//             panic!("Failed to pop off stack!");
//         };

//         let mut invalid = 0;

//         for _ in 0..=mult {
//             invalid = invalid * mag + curr
//         }

//         dbg!(&invalid);
//         if invalid == 0 {
//             dbg!(&stack);
//         }

//         if seen.contains(&invalid) {
//             continue;
//         }

//         if str <= invalid && invalid <= end {
//             count += invalid;
//             seen.insert(invalid);
//         }

//         if invalid > end {
//             continue;
//         }

//         for i in 0..=9 {
//             if i == 0 && curr == 0 {
//                 continue;
//             }
//             stack.push((curr * 10 + i, mag * 10, mult))
//         }

//         if !pt1 {
//             stack.push((curr, mag, mult))
//         }
//     }

//     count
// }

fn part1(input: &str) {
    let mut count = 0;
    for line in input.split("\n") {
        for range in line.split(",") {
            let Some((start, stop)) = range.split_once("-") else {
                panic!("Failed to unwrap range");
            };

            count += find_invalid(start, stop, true);
        }
    }

    println!("ID sums: {count}")
}

fn part2(input: &str) {
    let mut count = 0;
    for line in input.split("\n") {
        for range in line.split(",") {
            let Some((start, stop)) = range.split_once("-") else {
                panic!("Failed to unwrap range");
            };

            count += find_invalid(start, stop, false);
        }
    }

    println!("ID sums: {count}")
}

fn handle_cases(args: Args) {
    let mut files = Vec::new();

    if args.test {
        files.push("test.txt");
    }

    if args.input {
        files.push("input.txt");
    }

    for file in files {
        let input = fs::read_to_string(file).unwrap_or_else(|_| panic!("Failed to read {file}"));
        println!("****** {file} ******");

        if args.part1 {
            print!("Part 1 ");
            part1(&input);
        }

        if args.part2 {
            print!("Part 2 ");
            part2(&input);
        }

        println!("**** {file} ends ****\n");
    }
}

fn main() {
    let args = Args::parse();

    handle_cases(args);
}
