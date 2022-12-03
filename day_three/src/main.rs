use std::collections::HashSet;

fn char_to_value(c: &char) -> u32 {
    match c {
        c if c.is_uppercase() => c.to_ascii_lowercase() as u32 - 0x60 + 26, //0x46?
        _ => c.to_ascii_uppercase() as u32 - 0x40,
    }
}
fn main() {
    let r1 = include_str!("input.txt")
        .lines()
        .into_iter()
        .map(|line| (line.split_at(line.chars().count() / 2)))
        .map(|(left, right)| {
            left.chars()
                .collect::<HashSet<char>>()
                .intersection(&right.chars().collect::<HashSet<char>>())
                .map(char_to_value)
                .collect::<Vec<u32>>()
        })
        .flat_map(|x| x)
        .sum::<u32>();

    let r2 = include_str!("input.txt")
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|chunk| {
            let iter = chunk.iter().map(|line| {
                line.chars().fold(HashSet::new(), |mut acc, c| {
                    acc.insert(c);
                    acc
                })
            });
            let start = iter.clone().next().unwrap();
            iter.fold(start, |acc, x| acc.intersection(&x).map(|c| *c).collect())
                .iter()
                .map(char_to_value)
                .collect::<Vec<u32>>()
        })
        .flat_map(|x| x)
        .sum::<u32>();

    println!("Result I: {}", r1);
    println!("Result II: {}", r2);
}
