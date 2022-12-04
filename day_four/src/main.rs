fn main() {
    let r1 = include_str!("input.txt")
        .lines()
        .map(|line| line.split_once(","))
        .filter_map(|x| x)
        .map(|(left, right)| (left.split_once("-"), right.split_once("-")))
        .map(|(ls, rs)| (ls.unwrap(), rs.unwrap()))
        .map(|((ll, lr), (rl, rr))| {
            (
                ll.parse::<usize>().unwrap(),
                lr.parse::<usize>().unwrap(),
                rl.parse::<usize>().unwrap(),
                rr.parse::<usize>().unwrap(),
            )
        })
        .map(|(ll, lr, rl, rr)| {
            let mut left: u128 = 0;
            let mut right: u128 = 0;

            for i in ll..=lr {
                left |= 1 << i;
            }

            for i in rl..=rr {
                right |= 1 << i;
            }

            (left & right) == left || (left & right) == right
        })
        .map(|x| if x { 1 } else { 0 })
        .sum::<u32>();

    let r2 = include_str!("input.txt")
        .lines()
        .map(|line| line.split_once(","))
        .filter_map(|x| x)
        .map(|(left, right)| (left.split_once("-"), right.split_once("-")))
        .map(|(ls, rs)| (ls.unwrap(), rs.unwrap()))
        .map(|((ll, lr), (rl, rr))| {
            (
                ll.parse::<usize>().unwrap(),
                lr.parse::<usize>().unwrap(),
                rl.parse::<usize>().unwrap(),
                rr.parse::<usize>().unwrap(),
            )
        })
        .map(|(ll, lr, rl, rr)| {
            let mut left: u128 = 0;
            let mut right: u128 = 0;

            for i in ll..=lr {
                left |= 1 << i;
            }

            for i in rl..=rr {
                right |= 1 << i;
            }

            (left & right) > 0 || (left & right) > 0
        })
        .map(|x| if x { 1 } else { 0 })
        .sum::<u32>();

    println!("Result I: {}", r1);
    println!("Result II: {}", r2);
}
