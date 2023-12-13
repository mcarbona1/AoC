use std::{io::{self, BufRead}, collections::VecDeque};
use regex::Regex;

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<usize>,
    operation: String,
    opp_amount: String,
    test: usize,
    true_action: i32,
    false_action: i32,
    inspects: usize,
}

impl Monkey {
    fn update(&self, item: usize) -> usize {
        let update_amount;
        if self.opp_amount == "old" {
            update_amount = item;
        } else {
            update_amount = self.opp_amount.parse::<usize>().unwrap();
        }

        match self.operation.as_str() {
            "+" => {return item + update_amount;},
            "*" => {return item * update_amount;},
            "-" => {return item - update_amount;},
            "/" => {return item / update_amount;},
            &_ => {return item;},
        }
    }

    fn test(&self, item: usize) -> bool{
        (item % self.test) == 0
    }

    fn process_items(&mut self) -> VecDeque<(usize, i32)> {
        let mut outgoing: VecDeque<(usize, i32)> = VecDeque::new();
        for _i in 0..self.items.len() {
            let mut item = self.items.pop_front().unwrap();
            item = self.update(item);
            self.inspects += 1;
            item /= 3;
            if self.test(item) {
                outgoing.push_back((item, self.true_action))
            } else {
                outgoing.push_back((item, self.false_action))
            }
        }
        outgoing
    }

    fn process_items_stress(&mut self, Lcm: usize) -> VecDeque<(usize, i32)> {
        let mut outgoing: VecDeque<(usize, i32)> = VecDeque::new();
        for _i in 0..self.items.len() {
            let mut item = self.items.pop_front().unwrap();
            item = self.update(item);
            self.inspects += 1;
            item = item % Lcm;
            if self.test(item) {
                outgoing.push_back((item, self.true_action))
            } else {
                outgoing.push_back((item, self.false_action))
            }
        }
        outgoing
    }

    fn add_item(&mut self, item: usize) {
        self.items.push_back(item);
    }

}

fn parse_input(directions: VecDeque<String>) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut i = 0;
    monkeys.push(Monkey { items: VecDeque::new(), operation: "".to_string(), opp_amount: "".to_string(),test: 0, true_action: 0, false_action:  0, inspects: 0});
    for line in directions {
        if line == "" {
            monkeys.push(Monkey { items: VecDeque::new(), operation: "".to_string(), opp_amount: "".to_string(),test: 0, true_action: 0, false_action:  0, inspects: 0});
            i += 1;
        } else if line.starts_with("  Starting items:") {
            let offset = line.find(": ").unwrap_or(line.len());
            let array = &line[(offset+2)..];
            let array: VecDeque<usize> = array.split(", ").collect::<Vec<&str>>().into_iter().map(|x| x.parse::<usize>().unwrap()).collect();
            monkeys[i].items = array;
        } else if line.starts_with("  Operation:") {
            let re = Regex::new(r": new = old ([\+\-\*/]) (old|\d+)").unwrap();
            let operation = re.captures(&line).unwrap();
            monkeys[i].operation = operation.get(1).unwrap().as_str().to_string();
            monkeys[i].opp_amount = operation.get(2).unwrap().as_str().to_string();
        } else if line.starts_with("  Test:") {
            let offset = line.split(" ").collect::<Vec<&str>>();
            let offset = offset[offset.len()-1];
            let test = offset.parse::<usize>().unwrap();
            monkeys[i].test = test.clone();
        } else if line.starts_with("    If true:") {
            let offset = line.split(" ").collect::<Vec<&str>>();
            let offset = offset[offset.len()-1];
            let truth_action = offset.parse::<i32>().unwrap();
            monkeys[i].true_action = truth_action
        } else if line.starts_with("    If false:"){
            let offset = line.split(" ").collect::<Vec<&str>>();
            let offset = offset[offset.len()-1];
            let false_action = offset.parse::<i32>().unwrap();
            monkeys[i].false_action = false_action
        }
    }
    monkeys
}

fn lcm(numbers: Vec<usize>) -> usize {
  let mut lcm = 1;
  for number in numbers {
    lcm = lcm * number / gcd(lcm, number);
  }
  lcm as usize
}

fn gcd(a: usize, b: usize) -> usize {
  if b == 0 {
    a
  } else {
    gcd(b, a % b)
  }
}

fn pt1(mut monkeys: Vec<Monkey>) -> usize{
    for _j in 0..20 {
        for i in 0..monkeys.len() {
            let actions = monkeys[i].process_items();
            for item in actions {
                monkeys[item.1 as usize].add_item(item.0);
            }
        }
    }

    let mut passes: Vec<usize> = Vec::new();
    for monkey in monkeys {
        passes.push(monkey.inspects);
    }

    passes.sort();
    passes.reverse();

    passes[0] * passes[1]

}

fn pt2(mut monkeys: Vec<Monkey>) -> usize{
    let mut divisors: Vec<usize> = Vec::new();
    for monkey in monkeys.clone() {
        divisors.push(monkey.test);
    }

    let divisor = lcm(divisors);

    for _j in 0..10000 {
        for i in 0..monkeys.len() {
            let actions = monkeys[i].process_items_stress(divisor);
            for item in actions {
                monkeys[item.1 as usize].add_item(item.0);
            }
        }
    }

    let mut passes: Vec<usize> = Vec::new();
    for monkey in monkeys {
        passes.push(monkey.inspects);
    }

    passes.sort();
    passes.reverse();

    passes[0] * passes[1]

}

fn main() {
    let mut lines = io::stdin().lock().lines();
    let mut monkey_directions: VecDeque<String> = VecDeque::new();
    while let Some(line) = lines.next() {
        let input_line = match line {
            Ok(val) => val,
            Err(e) => {println!("Error: {}", e); continue;},
        };
        monkey_directions.push_back(input_line.clone());
    }

    let monkeys = parse_input(monkey_directions);
    let monkey_business = pt1(monkeys.clone());
    let stressed_mokey_business = pt2(monkeys);
    println!("{}", monkey_business);
    println!("{}", stressed_mokey_business);

}
