use std::{io::{self, BufRead, StdinLock}, collections::{VecDeque}};

#[derive(Clone, Debug)]
struct node {
    dir_children: Vec<node>,
    file_children: Vec<node>,
    name: String,
    is_dir: bool,
    size: usize,
    //parent: Option<Box<node>>,
}


impl node {
    fn add_file(&mut self, name: String, file_size: usize) {
        self.file_children.push(node {dir_children: Vec::new(), file_children: Vec::new(), name: name.clone(), is_dir: false, /*parent: Some(parent), */size: file_size})
    }

    fn add_dir(&mut self, name: String) {
        self.dir_children.push(node {dir_children: Vec::new(), file_children: Vec::new(), name: name.clone(), is_dir: true, /*parent: Some(Box::new(self)), */size: 0})
    }
}

fn find_dirs(fs: node, total: usize, mut possible_dirs: Vec<(String, usize)>) -> Vec<(String, usize)> {
    for child in fs.dir_children.clone() {
        possible_dirs = find_dirs(child, total, possible_dirs);
    }

    if (total - fs.size) <= 40000000{
        possible_dirs.push((fs.name.to_string(), fs.size));
    }

    possible_dirs
}

fn calculate_answer(fs: node) -> usize {
    let mut sum: usize = 0;
    for child in fs.dir_children.clone() {
            sum += calculate_answer(child);
    }

    if fs.size < 100000 {
        sum += fs.size;
    }

    sum
}

fn calculate_dir_size(mut fs: node) -> node {
    let mut dir_size: usize = 0;
    for file in fs.file_children.clone() {
        dir_size += file.size;
    }

    for i in 0..fs.dir_children.len() {
        fs.dir_children[i] = calculate_dir_size(fs.dir_children[i].clone());
        dir_size += fs.dir_children[i].size
    }

    fs.size = dir_size;

    fs
}

fn parse_input(mut lines: VecDeque<String>, mut fs: node) -> (VecDeque<String>, node) {
    while lines.len() > 0 {
    let input_line = match lines.pop_front() {
        Some(line) => line,
        None => return (lines, fs),
    };

    let line: Vec<&str> = input_line.split(" ").collect();
    if line[0] == "$" {
        if line[1] == "cd" {
            let dest = line[2];
            if dest == ".." {
                    return (lines, fs);
            } else {
                for i in 0..fs.dir_children.len() {
                    if fs.dir_children[i].name == dest {
                        (lines, fs.dir_children[i]) = parse_input(lines, fs.dir_children[i].clone());
                        break;

                    }
                }
                //return (lines, fs);
            }
        } else if line[1] == "ls" {
            continue;
        }
    } else if line[0] == "dir" {
        fs.add_dir(line[1].to_string());
    } else {
        fs.add_file(line[1].to_string(), line[0].parse::<usize>().unwrap());
    }
    }
    (lines, fs)
}

fn pt1 (mut lines: std::io::Lines<StdinLock>) -> usize{
    let root: node = node {dir_children: Vec::new(), file_children: Vec::new(), name: String::from("/"), is_dir: true, /*parent: None,*/ size: 0};
    let mut commands:VecDeque<String> = VecDeque::new();
    while let Some(line) = lines.next() {
        let input_line = match line {
            Ok(val) => val,
            Err(e) => {println!("Error: {}", e); continue;},
        };
        if input_line == "$ cd /" {
            continue;
        }

        commands.push_back(input_line);
    }

    let (_, fs) = parse_input(commands, root);
    let fs = calculate_dir_size(fs);
    dbg!(&fs);
    calculate_answer(fs)
}

fn pt2 (mut lines: std::io::Lines<StdinLock>) -> (String, usize) {
    let root: node = node {dir_children: Vec::new(), file_children: Vec::new(), name: String::from("/"), is_dir: true, /*parent: None,*/ size: 0};
    let mut commands:VecDeque<String> = VecDeque::new();
    while let Some(line) = lines.next() {
        let input_line = match line {
            Ok(val) => val,
            Err(e) => {println!("Error: {}", e); continue;},
        };
        if input_line == "$ cd /" {
            continue;
        }

        commands.push_back(input_line);
    }

    let (_, fs) = parse_input(commands, root);
    let fs = calculate_dir_size(fs);
    dbg!(&fs);
    let mut possible_dirs: Vec<(String, usize)> = Vec::new();
    let total = fs.size;
    possible_dirs = find_dirs(fs, total, possible_dirs);
    possible_dirs.sort_by_key(|k| k.1);

    (possible_dirs[0].0.clone(), possible_dirs[0].1)
}

fn main() {
    let lines = io::stdin().lock().lines();

    //let state = pt1(lines);
    let state = pt2(lines);

    println!("{}: {}", state.0, state.1);
}
