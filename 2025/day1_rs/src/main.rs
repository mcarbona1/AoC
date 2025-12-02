use std::fs;

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

fn part1(input: &str) {
    let mut count = 0;
    let mut location = 50;
    let size = 100;

    for line in input.split("\n") {
        let dir = &line[0..1];
        let amount = line[1..].parse::<i32>().unwrap();

        if dir == "L" {
            location = ((location - amount) % size + size) % size
        } else if dir == "R" {
            location = (location + amount) % size;
        }

        if location == 0 {
            count += 1;
        }
    }

    println!("Password: {count}")
}

fn part2(input: &str) {
    let mut count = 0;
    let mut location = 50;
    let size = 100;

    for line in input.split("\n") {
        let dir = &line[0..1];
        let amount = line[1..]
            .parse::<i32>()
            .unwrap_or_else(|_| panic!("Cannot convert {} to an integer", &line[1..]));

        if dir == "L" {
            if location - amount <= 0 {
                let clicks = (location - amount) / -size + (if location == 0 { 0 } else { 1 });

                count += clicks
            }

            location = ((location - amount) % size + size) % size
        } else if dir == "R" {
            let clicks = (location + amount) / size;
            location = (location + amount) % size;

            count += clicks;
        }
    }

    println!("Password: {count}")
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
