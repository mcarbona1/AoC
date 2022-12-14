use std::{io::{self, BufRead}, collections::{HashSet, VecDeque}};
use std::cmp;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

fn sim_sand_pt1(mut map: HashSet<Point>, abyss: usize) -> usize{
    let mut done = false;
    let mut sand = Point {x: 500, y: 0};
    let mut sand_count = 0;
    while !done {
        if sand.y > abyss {
            done = true;
        } else if !map.contains(&Point {x: sand.x, y: sand.y + 1}) {
            sand.y += 1;
        } else if !map.contains(&Point {x: sand.x - 1, y: sand.y + 1}) {
            sand.y += 1;
            sand.x -= 1;
        } else if !map.contains(&Point {x: sand.x + 1, y: sand.y + 1}) {
            sand.y += 1;
            sand.x += 1;
        } else {
            map.insert(sand.clone());
            sand.x = 500;
            sand.y = 0;
            sand_count += 1;
        }
    }

    sand_count
}

fn pt1(obstacles: VecDeque<String>) {
    let mut map: HashSet<Point> = HashSet::new();
    for obstacle in obstacles {
        let lines = obstacle.split(" -> ").collect::<VecDeque<&str>>();
        for i in 0..(lines.len()-1) {
            let mut start = lines[i].split(",");
            let mut end = lines[i+1].split(",");
            let x1 =  start.next().unwrap().parse::<usize>().unwrap();
            let y1 =  start.next().unwrap().parse::<usize>().unwrap();
            let x2 =  end.next().unwrap().parse::<usize>().unwrap();
            let y2 =  end.next().unwrap().parse::<usize>().unwrap();

            for j in cmp::min(x1,x2)..=cmp::max(x1,x2) {
                for k in cmp::min(y1, y2)..=cmp::max(y1, y2) {
                    map.insert(Point {x: j, y: k});
                }
            }

        }
    }

    let mut abyss = 0;
    for point in map.clone() {
        if point.y > abyss {
            abyss = point.y;
        }
    }

    let placed = sim_sand_pt1(map.clone(), abyss);
    println!("Sand Placed: {}", placed);

}

fn sim_sand_pt2(mut map: HashSet<Point>, abyss: usize) -> usize{
    let mut done = false;
    let mut sand = Point {x: 500, y: 0};
    let mut sand_count = 0;
    while !done {
        if sand.y + 1 == abyss + 2{
            map.insert(sand.clone());
            sand_count += 1;
            sand.x = 500;
            sand.y = 0;
        } else if !map.contains(&Point {x: sand.x, y: sand.y + 1}) {
            sand.y += 1;
        } else if !map.contains(&Point {x: sand.x - 1, y: sand.y + 1}) {
            sand.y += 1;
            sand.x -= 1;
        } else if !map.contains(&Point {x: sand.x + 1, y: sand.y + 1}) {
            sand.y += 1;
            sand.x += 1;
        } else {
            map.insert(sand.clone());
            sand_count += 1;
            if sand.x == 500 && sand.y == 0 {
                done = true;
                continue;
            }
            sand.x = 500;
            sand.y = 0;
        }
    }

    sand_count
}

fn pt2(obstacles: VecDeque<String>) {
    let mut map: HashSet<Point> = HashSet::new();
    for obstacle in obstacles {
        let lines = obstacle.split(" -> ").collect::<VecDeque<&str>>();
        for i in 0..(lines.len()-1) {
            let mut start = lines[i].split(",");
            let mut end = lines[i+1].split(",");
            let x1 =  start.next().unwrap().parse::<usize>().unwrap();
            let y1 =  start.next().unwrap().parse::<usize>().unwrap();
            let x2 =  end.next().unwrap().parse::<usize>().unwrap();
            let y2 =  end.next().unwrap().parse::<usize>().unwrap();

            for j in cmp::min(x1,x2)..=cmp::max(x1,x2) {
                for k in cmp::min(y1, y2)..=cmp::max(y1, y2) {
                    map.insert(Point {x: j, y: k});
                }
            }

        }
    }

    let mut abyss = 0;
    for point in map.clone() {
        if point.y > abyss {
            abyss = point.y;
        }
    }

    let placed = sim_sand_pt2(map.clone(), abyss);
    println!("Sand Placed: {}", placed);

}

fn main() {
    let mut lines = io::stdin().lock().lines();
    let mut obstacles: VecDeque<String> = VecDeque::new();
    while let Some(line) = lines.next() {
        let input_line = match line {
            Ok(val) => val,
            Err(e) => {println!("Error: {}", e); continue;},
        };
        if input_line == "" {
            continue;
        }
        obstacles.push_back(input_line.clone());
    }
    pt1(obstacles.clone());
    pt2(obstacles.clone());

}
