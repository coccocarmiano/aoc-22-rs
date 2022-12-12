const STAR: u32 = '*' as u32;

fn rsearch(
    valley: &mut Vec<Vec<u32>>,
    from: u32,
    pos: (usize, usize),
    end: (usize, usize),
    cost: u32,
    boundaries: (usize, usize),
    best: &mut u32,
    cost_matrix: &mut Vec<Vec<u32>>,
) {
    let (x, y) = pos;
    let actual = valley[y][x];

    if cost_matrix[y][x] <= cost {
        return; // No inefficient walking
    }
    if actual > from + 1 {
        return; // At most +1
    }
    if actual == STAR {
        return; // Already visited in this path
    }

    if pos == end {
        if cost < *best {
            *best = cost;
        }
        return;
    }

    cost_matrix[y][x] = cost;
    valley[y][x] = STAR;
    if let Some(next) = x.checked_sub(1) {
        rsearch(
            valley,
            actual,
            (next, y),
            end,
            cost + 1,
            boundaries,
            best,
            cost_matrix,
        )
    }
    if x + 1 < boundaries.0 {
        rsearch(
            valley,
            actual,
            (x + 1, y),
            end,
            cost + 1,
            boundaries,
            best,
            cost_matrix,
        )
    }

    if let Some(next) = y.checked_sub(1) {
        rsearch(
            valley,
            actual,
            (x, next),
            end,
            cost + 1,
            boundaries,
            best,
            cost_matrix,
        )
    }

    if y + 1 < boundaries.1 {
        // Down
        rsearch(
            valley,
            actual,
            (x, y + 1),
            end,
            cost + 1,
            boundaries,
            best,
            cost_matrix,
        )
    }
    valley[y][x] = actual;
}

fn min_path(valley: &mut Vec<Vec<u32>>, starts: Vec<(usize, usize)>, end: (usize, usize)) -> u32 {
    let mut best = u32::MAX;
    let x_max = valley[0].len();
    let y_max = valley.len();
    let mut cost_matrix = vec![vec![u32::MAX; x_max]; y_max];
    for start in starts {
        rsearch(
            valley,
            'a' as u32,
            start,
            end,
            0,
            (x_max, y_max),
            &mut best,
            &mut cost_matrix,
        );
    }
    return best;
}

fn main() {
    let mut lines = include_str!("input.txt").lines().peekable();
    let w = lines.peek().expect("Invalid Input").chars().count();
    let mut valley = lines
        .flat_map(|line| line.chars())
        .map(|c| c as u32)
        .collect::<Vec<u32>>()
        .chunks(w)
        .map(|c| c.to_vec())
        .collect::<Vec<Vec<u32>>>();

    let start_idx = valley
        .iter()
        .flat_map(|c| c)
        .enumerate()
        .find(|(_, &c)| c == ('S' as u32))
        .expect("Invalid Input")
        .0;
    let start = (start_idx % w, start_idx / w);
    valley[start.1][start.0] = 'a' as u32;

    let all_starts = valley
        .iter()
        .flat_map(|c| c)
        .enumerate()
        .filter(|(_, &c)| c == ('a' as u32))
        .map(|(i, _)| (i % w, i / w))
        .collect::<Vec<_>>();

    let end_idx = valley
        .iter()
        .flat_map(|c| c)
        .enumerate()
        .find(|(_, &c)| c == ('E' as u32))
        .expect("Invalid Input")
        .0;
    valley[end_idx / w][end_idx % w] = 'z' as u32;

    let end = (end_idx % w, end_idx / w);

    println!("Result I: {}", min_path(&mut valley, vec![start], end));
    println!("Result II: {}", min_path(&mut valley, all_starts, end));
}
