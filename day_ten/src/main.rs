fn main() {
    let ccs = include_str!("input.txt")
        .lines()
        .scan((1, 1), |(i, j), line| {
            let tup = match line {
                "noop" => (1, 0),
                _ => (2, line.split_once(" ")?.1.parse().expect("Invalid Input")),
            };
            *i += tup.0;
            *j += tup.1;
            Some((i.to_owned(), j.to_owned()))
        })
        .collect::<Vec<(i32, i32)>>()
        .windows(2)
        .map(|w| {
            (w[0].0..w[1].0) // [1]
                .map(|cc| (cc, w[0].1))
                .collect::<Vec<(i32, i32)>>()
        })
        .flat_map(|x| x)
        .collect::<Vec<(i32, i32)>>();
    let r1 = ccs
        .iter()
        .filter(|(cc, _)| (cc - 20) % 40 == 0)
        .map(|(x, y)| x * y)
        .sum::<i32>();

    println!("Result I: {}", r1);

    println!("Result II:");
    let up_to = ccs.first().unwrap().0 as usize;
    (1..up_to) // [2]
        .map(|idx| (idx as i32, 1))
        .into_iter()
        .chain(ccs.into_iter())
        .for_each(|(pos, value)| {
            let crt = (pos - 1) % 40;
            match crt - value {
                -1..=1 => print!("#"),
                _ => print!("."),
            }
            if pos % 40 == 0 {
                println!();
            }
        });
}

// [1] The scan might leave an empty cc (e.g. cc 18, cc 19, cc 21) so this is a way to fill the holes.
// [2] The hole problem is guaranteed to happen at the start, so this prefixes the starting part
