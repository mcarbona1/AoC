use std::{io::{self, BufRead}, collections::HashSet};
use priority_queue::DoublePriorityQueue;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Path {
    curr: Point,
    weight: usize,
}

fn dijikstra(map: Vec<Vec<char>>, mut steps: DoublePriorityQueue<Point, usize>, mut paths: Vec<Path>, mut unvisited: HashSet<Point>, end: Point) -> usize {
    let row_len = map[0].len();
    while steps.len() > 0 {
        let (next, weight) = steps.pop_min().unwrap();
        if !unvisited.contains(&next) {
            continue;
        } else {
            unvisited.remove(&next);
        }

        if next.x == end.x && next.y == end.y {
            return weight;
        }

        for i in 0..3 {
            for j  in 0..3 {
                if i == j || i as i32 == j as i32 - 2 || j as i32 == i as i32 - 2 || ((next.x == 0 && j == 0) || (next.y == 0 && i == 0) || (next.y == map.len()-1 && i == 2) ||  (next.x == map[0].len()-1 && j == 2)){
                    continue;
                }

                if (map[next.y + i-1][next.x + j-1]) as usize <= (map[next.y][next.x] as usize + 1) {
                    if unvisited.contains(&Point {x: next.x + j-1, y: next.y + i-1}) && paths[(next.y + i-1) * row_len + next.x + j-1].weight > weight + 1 && steps.get_priority(&Point {x: next.x + j-1, y: next.y + i-1}).unwrap_or(&usize::MAX) > &(weight+1) {
                        steps.push(Point {x: next.x + j-1, y: next.y + i-1}, weight + 1);

                    }
                }
            }
        }

        paths[next.y*map[0].len() + next.x].weight = weight;
    }
    0

}

fn pt2(mut map: Vec<Vec<char>>) {
    let mut paths: Vec<Path> = Vec::new();
    let mut unvisited: HashSet<Point> = HashSet::new();
    let mut end: Point = Point{x: 0, y: 0};
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            unvisited.insert(Point {x: j, y: i});
            if map[i][j] == 'E' {
                map[i][j] = 'z';
                end = Point {x: j, y: i};
            }
            if map[i][j] == 'S'{
                map[i][j] = 'a';
            }
            paths.push(Path {curr: Point {x:j, y:i}, weight: usize::MAX});
        }
    }

    let mut weight = usize::MAX;

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 'a' {
                let mut steps: DoublePriorityQueue<Point, usize> = DoublePriorityQueue::new();
                let path_clone = paths.clone();
                paths[i*map[0].len() + j].weight = 0;
                steps.push(Point {x: j, y: i}, 0);
                let path_len = dijikstra(map.clone(), steps, path_clone, unvisited.clone(), end.clone());
                if path_len < weight && path_len > 0 {
                    weight = path_len;
                }
            }

        }
    }

    println!("{}", weight);

}

fn pt1(mut map: Vec<Vec<char>>) {
    let mut steps: DoublePriorityQueue<Point, usize> = DoublePriorityQueue::new();
    let mut paths: Vec<Path> = Vec::new();
    let mut unvisited: HashSet<Point> = HashSet::new();
    let mut end: Point = Point{x: 0, y: 0};
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            unvisited.insert(Point {x: j, y: i});
            if map[i][j] == 'E' {
                map[i][j] = 'z';
                end = Point {x: j, y: i};
            }
            if map[i][j] == 'S'{
                map[i][j] = 'a';
                paths.push(Path {curr: Point {x:j, y:i}, weight: 0});
                steps.push(Point {x: j, y: i}, 0);
            } else {
                paths.push(Path {curr: Point {x:j, y:i}, weight: usize::MAX});

            }
        }
    }

    let weight = dijikstra(map, steps, paths, unvisited, end);

    println!("{}", weight);

}

fn main() {
    let mut lines = io::stdin().lock().lines();
    let mut map: Vec<Vec<char>> = Vec::new();
    while let Some(line) = lines.next() {
        let input_line = match line {
            Ok(val) => val,
            Err(e) => {println!("Error: {}", e); continue;},
        };

        map.push(input_line.chars().collect::<Vec<char>>());
    }

    pt1(map.clone());
    pt2(map.clone());


}
