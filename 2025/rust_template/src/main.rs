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
    for line in input.split("\n") {
        println!("{line}");
    }
}

fn part2(input: &str) {
    for line in input.split("\n") {
        println!("{line}");
    }
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
