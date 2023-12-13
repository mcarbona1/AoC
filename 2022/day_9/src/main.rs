use std::{io::{self, BufRead}, collections::{VecDeque, HashSet}};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

fn move_head(direction: &str, distance: i32, snake: &mut Vec<Point>, n: usize, visited: &mut HashSet<Point>) {
    let mut x_move = 0;
    let mut y_move = 0;
    match direction {
        "U" => {y_move = 1;},
        "D" => {y_move = -1;},
        "R" => {x_move = 1;},
        "L" => {x_move = -1;},
        &_ => return,
    }
    for _i in 0..distance {
        snake[0].x += x_move;
        snake[0].y += y_move;

        for j in 1..n {
            let mut head = snake[j-1].clone();
            let mut tail = snake[j].clone();
            move_tail(&mut head, &mut tail);
            snake[j-1] = head;
            snake[j] = tail;
        }
            visited.insert(snake[n-1].clone());
    }

}

fn move_tail(head: &mut Point, tail: &mut Point) {
    if next_to_head(head, tail) {
        return;
    }

    if head.x > tail.x {
        tail.x += 1;
    } else if head.x < tail.x {
        tail.x -= 1;
    }

    if head.y > tail.y {
        tail.y += 1;
    } else if head.y < tail.y {
        tail.y -= 1;
    }

}

fn next_to_head(head: &mut Point, tail: &mut Point) -> bool {
    if (tail.x - head.x).abs() <= 1 && (tail.y - head.y).abs() <= 1 {
        return true;
    }

    false
}

fn pt1(mut directions: VecDeque<(String, i32)>) -> i32 {
    let mut snake:Vec<Point> = Vec::new();
    for _i in 0..2 {
        snake.push(Point {x:0, y:0}.clone());
    }
    let mut visited: HashSet<Point> = HashSet::new();
    while directions.len() > 0 {
        let direction = directions.pop_front().unwrap();
        move_head(&direction.0, direction.1, &mut snake, 2, &mut visited);
        //dbg!(&visited);

    }

    visited.len() as i32
}

fn pt2(mut directions: VecDeque<(String, i32)>) -> i32 {
    let mut snake:Vec<Point> = Vec::new();
    for _i in 0..10 {
        snake.push(Point {x:0, y:0}.clone());
    }
    let mut visited: HashSet<Point> = HashSet::new();
    while directions.len() > 0 {
        let direction = directions.pop_front().unwrap();
        move_head(&direction.0, direction.1, &mut snake, 10, &mut visited);

    }

    visited.len() as i32
}

fn main() {
    let mut lines = io::stdin().lock().lines();
    let mut directions: VecDeque<(String, i32)> = VecDeque::new();
    while let Some(line) = lines.next() {
        let input_line = match line {
            Ok(val) => val,
            Err(e) => {println!("Error: {}", e); continue;},
        };
        let direction: Vec<&str> = input_line.split(" ").collect();
        directions.push_back((direction[0].to_string(), direction[1].parse::<i32>().unwrap()));
    }
    let visited = pt1(directions.clone());
    let visited10 = pt2(directions.clone());
    println!("{}", visited);
    println!("{}", visited10);

}
