use std::{io::{self, BufRead}, collections::VecDeque};

fn draw_pixel(line: &mut Vec<char>, pos: i32, pix_pos: &mut i32) {
    if (*pix_pos - pos).abs() <= 1 {
        line.push('#');
    } else {
        line.push('.');
    }

    *pix_pos += 1;

}

fn pt1(directions: VecDeque<String>) -> i32 {
    let mut signal_strength: i32 = 0;
    let mut clock: i32 = 1;
    let mut critical_val: i32 = 20;
    let mut x: i32 = 1;
    for direction in directions {
        clock += 1;
        if direction.starts_with("addx") {
            let add = direction.split(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
            if clock == critical_val {
                signal_strength += x * clock;
                critical_val += 40;
            }
            clock += 1;
            x += add;
        }
        if clock == critical_val {
            signal_strength += x * clock;
            critical_val += 40;
        }
    }

    signal_strength
}

fn pt2(directions: VecDeque<String>) -> Vec<Vec<char>>{
    let mut screen: Vec<Vec<char>> = Vec::new();
    for _i in 0..6 {
        screen.push(Vec::new());
    }
    //let mut clock: i32 = 1;
    let mut x: i32 = 1;
    let mut screen_x: i32 = 0;
    let mut screen_y: usize = 0;
    for direction in directions {
        //clock += 1;
        draw_pixel(&mut screen[screen_y], x, &mut screen_x);
        if screen_x >= 40 {
            screen_x = 0;
            screen_y += 1;
        }
        if direction.starts_with("addx") {
            let add = direction.split(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
            //clock += 1;
            draw_pixel(&mut screen[screen_y], x, &mut screen_x);
            if screen_x >= 40 {
                screen_x = 0;
                screen_y += 1;
            }
            x += add;
        }


    }

    screen
}

fn main() {
    let mut lines = io::stdin().lock().lines();
    let mut directions: VecDeque<String> = VecDeque::new();
    while let Some(line) = lines.next() {
        let input_line = match line {
            Ok(val) => val,
            Err(e) => {println!("Error: {}", e); continue;},
        };
        directions.push_back(input_line.clone());
    }

    let sig = pt1(directions.clone());
    let screen = pt2(directions.clone());

    println!("{}", sig);

    for line in screen {
        println!("{}", line.into_iter().collect::<String>());
    }

}
