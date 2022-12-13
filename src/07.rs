use std::collections::VecDeque;

fn main() {
    let available_space = 70_000_000;
    let update_size = 30_000_000;

    let tree = include_str!("./input/07.txt")
        .lines()
        .filter(|s| !s.starts_with("dir"))
        .filter(|s| !s.starts_with("$ ls"))
        .map(|s| s.replace("$ cd ", ""))
        .map(|s| {
            let (w, n) = s.split_once(" ").unwrap_or(("0", &s));
            let w = w.parse::<usize>().expect("Invalid input");
            let n = n.to_string();
            (n, w)
        })
        .fold((0usize, VecDeque::new()), |(depth, mut acc), (n, w)| {
            if n == ".." {
                return (depth - 1, acc);
            } else if w == 0 {
                acc.push_back((depth, w));
                return (depth + 1, acc);
            } else {
                acc.push_back((depth, w));
                return (depth, acc);
            }
        })
        .1;

    let r1 = tree
        .iter()
        .enumerate()
        .filter(|(_, (_, w))| *w == 0)
        .map(|(idx, (d, _))| {
            tree.iter()
                .skip(idx + 1)
                .take_while(|(d2, _)| d2 > d)
                .map(|(_, w2)| w2)
                .sum::<usize>()
        })
        .filter(|sum| *sum <= 100_000)
        .sum::<usize>();

    let occupied = tree.iter().map(|(_, w)| w).sum::<usize>();
    let free = available_space - occupied;
    let obj = update_size - free;

    let r2 = tree
        .iter()
        .enumerate()
        .filter(|(_, (_, w))| *w == 0)
        .map(|(idx, (d, _))| {
            tree.iter()
                .skip(idx + 1)
                .take_while(|(d2, _)| d2 > d)
                .map(|(_, w2)| w2)
                .sum::<usize>()
        })
        .filter(|sum| *sum >= obj)
        .min()
        .expect("Invalid input");

    println!("Result I: {}", r1);
    println!("Result II: {}", r2)
}
