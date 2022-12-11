use std::{collections::VecDeque, rc::Rc};

#[derive(Clone)]
struct Monkey {
    items: VecDeque<u64>,
    op: Rc<dyn Fn(u64) -> u64>,
    test: u64,
    test_true: usize,
    test_false: usize,
    nispects: u64,
}

impl Monkey {
    fn from_string(data: String) -> Monkey {
        let data = data
            .split("\n")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let items = data
            .iter()
            .nth(1)
            .expect("Invalid Input")
            .split(": ")
            .nth(1)
            .expect("Invalid Input")
            .split(",")
            .map(|num| num.trim_start().parse::<u64>().expect("Invalid Input"))
            .collect::<VecDeque<u64>>();
        let mut expr = data
            .iter()
            .nth(2)
            .expect("Invalid Input")
            .split_once("=")
            .expect("Invalid Input")
            .1
            .trim()
            .split(" ");

        let test = data
            .iter()
            .nth(3)
            .expect("Invalid Input")
            .split(" ")
            .last()
            .expect("Invalid Input")
            .parse::<u64>()
            .expect("Invalid Input");
        let test_true = data
            .iter()
            .rev()
            .nth(1)
            .expect("Invalid Input")
            .split(" ")
            .last()
            .expect("Invalid Input")
            .parse::<usize>()
            .expect("Invalid Input");
        let test_false = data
            .iter()
            .rev()
            .nth(0)
            .expect("Invalid Input")
            .split(" ")
            .last()
            .expect("Invalid Input")
            .parse::<usize>()
            .expect("Invalid Input");

        let first = expr.next().expect("Invalid Input").trim().to_string();
        let op = expr.next().expect("Invalid Input").trim().to_string();
        let second = expr.next().expect("Invalid Input").trim().to_string();

        let op = Rc::new(move |x| -> u64 {
            let bfirst = first.parse::<u64>().unwrap_or(x);
            let bsecond = second.parse::<u64>().unwrap_or(x);
            match op.as_str() {
                "+" => bfirst + bsecond,
                "-" => bfirst - bsecond,
                "*" => bfirst * bsecond,
                "/" => bfirst / bsecond,
                _ => panic!("Invalid Input"),
            }
        });

        Monkey {
            items,
            op,
            test,
            test_true,
            test_false,
            nispects: 0,
        }
    }
}

fn round(monkeys: &mut Vec<Monkey>, mcm: u64, do_div: bool) {
    for monkey_n in 0..monkeys.len() {
        for _ in 0..monkeys[monkey_n].items.len() {
            let item = monkeys[monkey_n].items.remove(0).unwrap();
            let t = monkeys[monkey_n].test;
            let worry_level = ((monkeys[monkey_n].op)(item) / if do_div { 3 } else { 1 }) % mcm;
            let tt = monkeys[monkey_n].test_true;
            let tf = monkeys[monkey_n].test_false;
            if worry_level % t == 0 {
                monkeys[tt].items.push_back(worry_level);
            } else {
                monkeys[tf].items.push_back(worry_level);
            }
            monkeys[monkey_n].nispects += 1;
        }
    }
}

fn main() {
    let mut monkeys = include_str!("input.txt")
        .split("\n\n")
        .map(|s| s.to_string())
        .map(|s| Monkey::from_string(s))
        .collect::<Vec<Monkey>>();

    let prod = monkeys.iter().map(|m| m.test).product::<u64>();
    let mut m1 = monkeys.clone();
    for _ in 0..20 {
        round(&mut m1, prod, true);
    }

    let mut nispects = m1.iter().map(|m| m.nispects).collect::<Vec<u64>>();
    nispects.sort();
    let r1 = nispects[nispects.len() - 1] * nispects[nispects.len() - 2];

    println!("Result I: {}", r1);

    for _ in 0..10_000 {
        round(&mut monkeys, prod, false);
    }
    let mut nispects = monkeys.iter().map(|m| m.nispects).collect::<Vec<u64>>();
    nispects.sort();
    let r2 = nispects[nispects.len() - 1] * nispects[nispects.len() - 2];

    println!("Result II: {}", r2)
}
