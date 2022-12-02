fn main() {
    let mut cals = include_str!("input.txt")
        .lines()
        .fold((Vec::<u32>::new(), 0u32), |(mut vec, sum), line| {
            let Ok(num) = line.parse::<u32>() else {
                    vec.push(sum);
                    return (vec, 0);
                };
            (vec, sum + num)
        })
        .0;
    cals.sort();
    let top3: Vec<u32> = cals.into_iter().rev().take(3).collect();
    println!("Result I: {}", top3[0]);
    println!("Result II: {}", top3.iter().sum::<u32>());
}
