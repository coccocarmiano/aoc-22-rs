use std::collections::VecDeque;

fn main() {
    let (start, commands) = include_str!("input.txt").split_once("\n\n").unwrap();

    let ncols = start
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let mut stacks_one = vec![VecDeque::new(); ncols];
    start
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .for_each(|chars| {
            chars
                .iter()
                .enumerate()
                .filter(|(_, c)| c.is_alphabetic())
                .for_each(|(i, c)| {
                    stacks_one[i / 4].push_front(*c);
                })
        });

    let mut stacks_two = stacks_one.clone();

    let commands = commands
        .lines()
        .map(|line| {
            line.split_whitespace()
                .into_iter()
                .map(|s| s.parse::<usize>())
                .filter_map(Result::ok)
        })
        .flat_map(|num| num)
        .collect::<Vec<usize>>()
        .chunks(3)
        .into_iter()
        .map(|chunk| (chunk[0], chunk[1] - 1, chunk[2] - 1))
        .collect::<Vec<(usize, usize, usize)>>();

    commands.clone().into_iter().for_each(|(n, from, to)| {
        for _ in 0..n {
            let target = stacks_one[from as usize].pop_back().unwrap();
            stacks_one[to as usize].push_back(target);
        }
    });

    commands.into_iter().for_each(|(n, from, to)| {
        let x = (0..n)
            .into_iter()
            .map(|_| stacks_two[from].pop_back().unwrap())
            .rev()
            .collect::<Vec<char>>();

        x.into_iter()
            .rev()
            .for_each(|c| stacks_two[to].push_back(c));
    });

    let r1 = stacks_one
        .iter()
        .map(|stack| stack.iter().last().cloned().unwrap())
        .collect::<String>();

    let r2 = stacks_two
        .iter()
        .map(|stack| stack.iter().last().cloned().unwrap())
        .collect::<String>();

    println!("Result I: {}", r1);
    println!("Result I: {}", r2);
}

// 1 5 9
