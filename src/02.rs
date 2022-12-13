#![allow(non_upper_case_globals)]

type ScoreMatrix = [[u32; 3]; 3];
const sm: ScoreMatrix = [[4, 8, 3], [1, 5, 9], [7, 2, 6]];
const im: ScoreMatrix = [[3, 4, 8], [1, 5, 9], [2, 6, 7]];

fn main() {
    let rounds = include_str!("./input/02.txt")
        .lines()
        .map(|l| {
            let mut split = l.split_whitespace();

            let him = match split.next().unwrap() {
                "A" => 0,
                "B" => 1,
                "C" => 2,
                _ => panic!("Invalid input"),
            };

            let you = match split.next().unwrap() {
                "X" => 0,
                "Y" => 1,
                "Z" => 2,
                _ => panic!("Invalid input"),
            };

            (him, you)
        })
        .collect::<Vec<(usize, usize)>>();

    let result_one = rounds
        .clone()
        .into_iter()
        .map(|(him, you)| sm[him][you])
        .sum::<u32>();

    let result_two = rounds
        .clone()
        .into_iter()
        .map(|(him, you)| im[you][him])
        .sum::<u32>();

    println!("Result: {}", result_one);
    println!("Result: {}", result_two);
}
