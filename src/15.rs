use std::collections::{HashMap, HashSet};

fn main() {
    let coords = include_str!("input/15.txt")
        .lines()
        .map(|line| line.replace("Sensor at x=", ""))
        .map(|line| line.replace(": closest beacon is at x=", " "))
        .map(|line| line.replace(",", ""))
        .map(|line| line.replace("y=", ""))
        .map(|line| {
            if let [x1, y1, x2, y2] = line
                .split_whitespace()
                .into_iter()
                .map(|n| n.parse::<isize>().unwrap())
                .collect::<Vec<isize>>()[..]
            {
                return ((x1, y1), (x2, y2));
            }
            panic!("")
        })
        .collect::<Vec<_>>();

    let ystar = 2_000_000;

    let mut impossible = HashSet::new();
    for (source, beacon) in coords.iter() {
        let (xsrc, ysrc) = source;
        let (xbeacon, ybeacon) = beacon;

        let d = (xsrc - xbeacon).abs() + (ysrc - ybeacon).abs();
        let d = d - (ysrc - ystar).abs();
        for x in xsrc - d..=xsrc + d {
            impossible.insert((x, ystar));
        }
    }

    for (source, beacon) in coords.iter() {
        impossible.remove(&source);
        impossible.remove(&beacon);
    }

    println!("Result I: {}", impossible.len());

    let min = 0;
    let max = 4_000_000;
    let map = coords
        .iter()
        .map(|&(source, beacon)| {
            let d = (source.0 - beacon.0).abs() + (source.1 - beacon.1).abs();
            (source, d)
        })
        .collect::<HashMap<(isize, isize), isize>>();
    map.iter()
        .map(|(s, &d)| {
            let (x, y) = s;
            let mut set = HashSet::new();
            for i in 0..=d {
                set.insert((x - d + i - 1, y + i + 1));
                set.insert((x - d + i - 1, y - i - 1));
                set.insert((x + d - i + 1, y + i));
                set.insert((x + d - i + 1, y - i - 1));
            }
            set.into_iter()
        })
        .flat_map(|x| x)
        .filter(|&(px, py)| {
            map.iter()
                .all(|(&source, &d)| (px - source.0).abs() + (py - source.1).abs() > d)
        })
        .filter(|&(x, y)| x >= min && x <= max && y >= min && y <= max)
        .for_each(|(x, y)| println!("Result II: {}", x * max + y));
}
