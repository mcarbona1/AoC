use std::{io::{self, BufRead}, collections::{HashSet, VecDeque}};
use regex::Regex;
use std::cmp;

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Offset {
    low: i32,
    high: i32,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Pair {
    sensor: Point,
    beacon: Point,
}

fn pt2(pairs: Vec<Pair>) -> u64 {
    let max = 4000000;
    for y in 0..=max {
        let mut intervals: VecDeque<Offset> = VecDeque::new();

        for pair in pairs.clone() {
            let distance = (pair.sensor.x - pair.beacon.x).abs() + (pair.sensor.y - pair.beacon.y).abs();
            let offset = distance - (pair.sensor.y - y).abs();

            if offset < 0 {
                continue;
            }

            intervals.push_back(Offset {low: pair.sensor.x - offset, high: pair.sensor.x + offset});
        }

        intervals.make_contiguous().sort();

        let mut q: Vec<Offset> = Vec::new();
        q.push(intervals.pop_front().unwrap());

        for range in intervals {
            let curr = q[q.len()-1].clone();

            if range.low > curr.high + 1 {
                q.push(range.clone());
                continue;
            }

            let last = q.len() - 1;
            q[last].high = cmp::max(curr.high, range.high);
        }


        let mut x = 0;
        for range in q {
            if x < range.low {
                return (x as u64 * 4000000 + y as u64) as u64;
            }

            x = cmp::max(x, range.high + 1);
            if x > max {
                break;
            }
        }
    }

    0
}

fn pt1(pairs: Vec<Pair>, row: i32) -> usize {
    let mut row_set: HashSet<Point> = HashSet::new();
    let mut beacon_set: HashSet<Point> = HashSet::new();
    for pair in pairs.clone() {
        beacon_set.insert(pair.beacon);
    }
    for pair in pairs.clone() {
        let distance = (pair.sensor.x - pair.beacon.x).abs() + (pair.sensor.y - pair.beacon.y).abs();
        if (distance + pair.sensor.y >= row && pair.sensor.y <= row) || (pair.sensor.y - distance <= row && pair.sensor.y >= row){
            for i in 0..=(distance - (row - pair.sensor.y).abs()).abs() {
                if !beacon_set.contains(&Point {x: pair.sensor.x + i, y: row}) {
                    row_set.insert(Point {x: pair.sensor.x + i, y: row});
                }
                if !beacon_set.contains(&Point {x: pair.sensor.x - i, y: row}) {
                    row_set.insert(Point {x: pair.sensor.x - i, y: row});
                }
            }
        }
    }

    row_set.len()

}

fn parse_input(directions: VecDeque<String>) -> Vec<Pair> {
    let mut pairs: Vec<Pair> = Vec::new();
    for line in directions {
        let re = Regex::new(r"Sensor at x=(-*\d+), y=(-*\d+): closest beacon is at x=(-*\d+), y=(-*\d+)").unwrap();
        let pair = re.captures(&line).unwrap();
        pairs.push(Pair {sensor: Point {x: pair.get(1).unwrap().as_str().parse::<i32>().unwrap(), y: pair.get(2).unwrap().as_str().parse::<i32>().unwrap()}, beacon: Point {x: pair.get(3).unwrap().as_str().parse::<i32>().unwrap(), y: pair.get(4).unwrap().as_str().parse::<i32>().unwrap()}});
    }
    pairs
}

fn main() {
    let mut lines = io::stdin().lock().lines();
    let mut input_lines: VecDeque<String> = VecDeque::new();
    while let Some(line) = lines.next() {
        let input_line = match line {
            Ok(val) => val,
            Err(e) => {println!("Error: {}", e); continue;},
        };
        input_lines.push_back(input_line.clone());
    }

    let pairs = parse_input(input_lines);

    let taken_spaces = pt1(pairs.clone(), 2000000);
    let freq = pt2(pairs.clone());

    println!("Taken Spaces: {}", taken_spaces);
    println!("Freq: {}", freq);
}
