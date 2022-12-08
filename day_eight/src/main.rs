use itertools::Itertools;

fn main() {
    let lines = include_str!("input.txt").lines().collect::<Vec<&str>>();

    let h = lines.len();
    let w = lines[0].len();
    let slicebuf = lines
        .iter()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).expect("Invalid ouput") as i32)
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut rm = (0..w)
        .map(|_| (0..h).map(|_| false).collect::<Vec<bool>>())
        .collect::<Vec<Vec<bool>>>();

    for y in 0..h {
        for x in 0..w {
            let value = slicebuf[x][y];
            let ltr = !slicebuf[x][0..y].iter().any(|&v| v >= value);
            let rtl = !slicebuf[x][y + 1..].iter().any(|&v| v >= value);
            let ttb = !slicebuf[0..x].iter().map(|v| v[y]).any(|v| v >= value);
            let btt = !slicebuf[x + 1..].iter().map(|v| v[y]).any(|v| v >= value);

            rm[x][y] = ltr || rtl || ttb || btt;
        }
    }

    let mut max = 0;
    let mut max_pos = (0, 0);
    let mut tree = 0;
    for y in 1..h - 1 {
        for x in 1..w - 1 {
            let value = slicebuf[x][y];

            let s = &slicebuf[x][..y]
                .iter()
                .rev()
                .take_while(|&&v| v <= value)
                .collect::<Vec<&i32>>();
            let from_left = if s.contains(&&value) {
                s.iter().position(|&&v| v == value).unwrap() + 1
            } else {
                s.iter().take_while(|&&v| v < &value).count().max(1)
            };

            let s = slicebuf[x][y + 1..]
                .iter()
                .take_while(|&&v| v <= value)
                .collect::<Vec<&i32>>();
            let to_the_right = if s.contains(&&value) {
                s.iter().position(|&&v| v == value).unwrap() + 1
            } else {
                s.iter().take_while(|&&v| v < &value).count().max(1)
            };

            let s = &slicebuf[0..x]
                .iter()
                .rev()
                .map(|v| v[y])
                .take_while(|&v| v <= value)
                .collect::<Vec<i32>>();
            let from_the_top = if s.contains(&value) {
                s.iter().position(|&v| v == value).unwrap() + 1
            } else {
                s.iter().count().max(1)
            };

            let s = &slicebuf[x + 1..]
                .iter()
                .map(|v| v[y])
                .take_while(|&v| v <= value)
                .collect::<Vec<i32>>();
            let to_the_bottom = if s.contains(&value) {
                s.iter().position(|&v| v == value).unwrap() + 1
            } else {
                s.iter().take_while(|&&v| v < value).count().max(1)
            };

            let total = from_left * to_the_right * from_the_top * to_the_bottom;
            if total > max {
                max = total;
                max_pos = (x, y);
                tree = value;
            }
        }
    }
    println!("Result I: {}", rm.iter().flatten().filter(|&x| *x).count());
    println!("Result II: {}", max);
}
