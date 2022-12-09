use std::collections::HashSet;

fn dist(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    ((p1.0 - p2.0).abs()).max((p1.1 - p2.1).abs())
}

fn main() {
    let input = include_str!("input.txt")
        .lines()
        .map(|line| line.split_once(" ").expect("Invalid Input"))
        .map(|(mv, reps)| {
            let reps = reps.parse::<i32>().expect("Invalid Input");
            (mv, reps)
        })
        .collect::<Vec<(&str, i32)>>();

    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut set = HashSet::<(i32, i32)>::new();

    for (mv, reps) in input.clone() {
        let delta = match mv {
            "L" => (-1, 0),
            "R" => (1, 0),
            "U" => (0, 1),
            "D" => (0, -1),
            _ => panic!("Invalid Input"),
        };

        for _ in 0..reps {
            let old = head;
            head.0 += delta.0;
            head.1 += delta.1;
            if dist(head, tail) > 1 {
                tail = old;
            }
            set.insert(tail);
        }
    }

    println!("Result I: {}", set.len());

    set.clear();
    let mut heads = vec![(0, 0); 10];

    for (mv, reps) in input.clone() {
        let delta = match mv {
            "L" => (-1, 0),
            "R" => (1, 0),
            "U" => (0, 1),
            "D" => (0, -1),
            _ => panic!("Invalid Input"),
        };

        for _ in 0..reps {
            heads[0].0 += delta.0;
            heads[0].1 += delta.1;
            let mut old = heads[0];
            for i in 0..10 {
                // Why does this need type annotations
                let delta: (i32, i32) = (old.0 - heads[i].0, old.1 - heads[i].1);
                if delta.0.abs() > 1 || delta.1.abs() > 1 {
                    heads[i].0 += delta.0.signum();
                    heads[i].1 += delta.1.signum();
                }
                old = heads[i];
            }
            set.insert(heads[9]);
        }
    }

    println!("Result II: {}", set.len());
    // Display result visually
    // let minx = set.iter().map(|(x, _)| x).min().unwrap();
    // let maxx = set.iter().map(|(x, _)| x).max().unwrap();
    // let miny = set.iter().map(|(_, y)| y).min().unwrap();
    // let maxy = set.iter().map(|(_, y)| y).max().unwrap();

    // for y in (miny - 1)..(maxy + 2) {
    // for x in (minx - 1)..(maxx + 2) {
    // if set.contains(&(x, y)) {
    // print!("#");
    // } else {
    // print!(".");
    // }
    // }
    // println!();
    // }
}
