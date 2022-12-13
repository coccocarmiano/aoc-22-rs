use std::collections::HashSet;

fn main() {
    let r1 = include_str!("./input/06.txt")
        .chars()
        .collect::<Vec<char>>()
        .windows(4)
        .enumerate()
        .find(|(_, w)| w.iter().map(|c| *c).collect::<HashSet<char>>().len() == 4)
        .map(|(idx, _)| idx + 4)
        .expect("Invalid input");

    let r2 = include_str!("./input/06.txt")
        .chars()
        .collect::<Vec<char>>()
        .windows(14)
        .enumerate()
        .find(|(_, w)| w.iter().map(|c| *c).collect::<HashSet<char>>().len() == 14)
        .map(|(idx, _)| idx + 14)
        .expect("Invalid input");

    println!("Result I: {}", r1);
    println!("Result II: {}", r2);
}
