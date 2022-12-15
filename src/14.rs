use std::collections::HashSet;

fn main() {
    let mut obstacles = HashSet::<(i32, i32)>::new();
    let paths = include_str!("input/14.txt")
        .lines()
        .map(|line| line.split(" -> ").collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut max_y = 0;

    let mut test = HashSet::<(i32, i32)>::new();
    for path in paths.clone() {
        for window in path.windows(2) {
            let (x1, y1) = window[0]
                .split_once(",")
                .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
                .unwrap();
            let (x2, y2) = window[1]
                .split_once(",")
                .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
                .unwrap();

            if x1 != x2 {
                let xfrom = x1.min(x2);
                let xto = x1.max(x2);

                for x in xfrom..=xto {
                    obstacles.insert((x, y1));
                    test.insert((x, y1));
                }
            } else {
                let yfrom = y1.min(y2);
                let yto = y1.max(y2);

                for y in yfrom..=yto {
                    obstacles.insert((x1, y));
                    test.insert((x1, y));

                    if y > max_y {
                        max_y = y;
                    }
                }
            }
        }
    }

    let mut point = (500, 0);
    let mut cnt = 0;
    let for_later = obstacles.clone();
    let max_y = max_y + 2;

    loop {
        let new_point = obstacles
            .iter()
            .filter(|&&(x, y)| x == point.0 && y > point.1)
            .min_by_key(|(_, y)| *y);

        if new_point.is_none() {
            break; // No more room above or no obstacles below
        };

        let obs = new_point.unwrap().to_owned();

        let left = (obs.0 - 1, obs.1);
        if !obstacles.contains(&left) {
            point = left;
            continue;
        }

        let right = (obs.0 + 1, obs.1);
        if !obstacles.contains(&right) {
            point = right;
            continue;
        }

        point = (obs.0, obs.1 - 1);
        obstacles.insert(point);
        cnt += 1;
        point = (500, 0);
        continue;
    }
    println!("Result I: {}", cnt);

    let mut obstacles = for_later;
    let mut point = (500, 0);
    cnt = 0;

    loop {
        let mut obs = point;
        while !obstacles.contains(&obs) && obs.1 <= max_y {
            obs.1 += 1;
        }
        obs.1 -= 1;

        let left = (obs.0 - 1, obs.1 + 1);
        if !obstacles.contains(&left) && obs.1 < max_y {
            point = left;
            continue;
        }

        let right = (obs.0 + 1, obs.1 + 1);
        if !obstacles.contains(&right) && obs.1 < max_y {
            point = right;
            continue;
        }

        if obs.1 >= max_y {
            point = (obs.0, max_y - 1);
        } else {
            point = obs;
        }

        obstacles.insert(point);
        cnt += 1;

        if point == (500, 0) {
            break;
        }
        point = (500, 0);
        continue;
    }

    println!("Result II: {}", cnt);
}
