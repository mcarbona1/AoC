use std::io::{self, BufRead};
fn main() {
    let mut lines = io::stdin().lock().lines();
    let mut overlap = 0;
    while let Some(line) = lines.next() {
        let input_line = match line {
            Ok(val) => val,
            Err(e) => {println!("Error: {}", e); continue;},
        };

        let tasks: Vec<&str> = input_line.split(",").collect();
        let elf1 = tasks[0];
        let elf2 = tasks[1];
        let elf1: Vec<i32> = elf1.split("-").into_iter().map(|x| x.parse::<i32>().unwrap()).collect();
        let elf2: Vec<i32> = elf2.split("-").into_iter().map(|x| x.parse::<i32>().unwrap()).collect();


        if (elf1[0] <= elf2[0] && elf1[1] >= elf2[0]) || (elf1[0] >= elf2[0] && elf1[0] <= elf2[1]){
            overlap += 1;
        }

    }

    println!("{}", overlap);
}
