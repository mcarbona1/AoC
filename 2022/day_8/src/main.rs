use std::{io::{self, BufRead}, collections::VecDeque, cmp};

fn is_visible(trees: &Vec<Vec<i32>>, tree_height: i32, tree_x: usize, tree_y: usize) -> bool {
    let mut taller = true;
    for i in 0..tree_x {
        if trees[tree_y][i] >= tree_height {
            taller = false;
            break;
        }
    }

    if taller {
        return true
    }

    taller = true;

    for i in (tree_x+1)..trees.len() {
        if trees[tree_y][i] >= tree_height {
            taller = false;
            break;
        }
    }

    if taller {
        return true
    }

    taller = true;

    for i in 0..tree_y {
        if trees[i][tree_x] >= tree_height {
            taller = false;
            break;
        }
    }

    if taller {
        return true
    }

    taller = true;

    for i in (tree_y+1)..trees.len() {
        if trees[i][tree_x] >= tree_height {
            taller = false;
            break;
        }
    }

    if taller {
        return true
    }

    false
}

fn find_tree_range(trees: &Vec<Vec<i32>>, tree_height: i32, tree_x: usize, tree_y: usize) -> i32 {
    let mut tree_range = 1;
    let mut sight_range = 0;
    for i in (0..tree_x).rev() {
        sight_range += 1;
        if trees[tree_y][i] >= tree_height {
            break;
        }
    }

    tree_range *= sight_range;
    sight_range = 0;

    for i in (tree_x+1)..trees.len() {
        sight_range += 1;
        if trees[tree_y][i] >= tree_height {
            break;
        }
    }

    tree_range *= sight_range;
    sight_range = 0;

    for i in (0..tree_y).rev() {
        sight_range += 1;
        if trees[i][tree_x] >= tree_height {
            break;
        }
    }

    tree_range *= sight_range;
    sight_range = 0;

    for i in (tree_y+1)..trees.len() {
        sight_range += 1;
        if trees[i][tree_x] >= tree_height {
            break;
        }
    }

    tree_range *= sight_range;

    tree_range
}

fn pt1(trees: &Vec<Vec<i32>>) -> i32 {
    let mut visible: i32 = 0;
    for i in 1..(trees.len()-1) {
        for j in 1..(trees[0].len()-1) {
            if is_visible(&trees, trees[i][j], j, i) {
                visible += 1;
            }

        }
    }

    visible += 2*(2*trees.len() as i32 - 2);

    visible
}

fn pt2(trees: &Vec<Vec<i32>>) -> i32 {
    let mut tree_range: i32 = 0;
    for i in 1..(trees.len()-1) {
        for j in 1..(trees[0].len()-1) {
            tree_range = cmp::max(find_tree_range(&trees, trees[i][j], j, i), tree_range);
        }
    }
    tree_range
}

fn main() {
    let mut lines = io::stdin().lock().lines();
    let mut trees: Vec<Vec<i32>> = Vec::new();
    while let Some(line) = lines.next() {
        let input_line = match line {
            Ok(val) => val,
            Err(e) => {println!("Error: {}", e); continue;},
        };
        let mut tree_line: VecDeque<&str> = input_line.split("").collect();
        tree_line.pop_back();
        tree_line.pop_front();
        let tree_line: Vec<i32> = tree_line.into_iter().map(|x| x.parse::<i32>().unwrap()).collect();
        trees.push(tree_line.clone());
    }


    let hidden = pt1(&trees);
    let range = pt2(&trees);
    println!("Hidden: {}", hidden);
    println!("Range: {}", range);

}
