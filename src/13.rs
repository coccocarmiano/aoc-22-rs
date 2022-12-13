use std::{
    cmp::Ordering::{self, Equal},
    fmt::{Display, Formatter},
    num::ParseIntError,
};

#[derive(Clone, PartialEq, Eq, PartialOrd)]
enum ListOrValue {
    List(Vec<ListOrValue>),
    Value(u32),
}

// Not really neeeded but cute, prints as the exercise
impl Display for ListOrValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ListOrValue::List(l) => {
                let s = l
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "[{}]", s)
            }
            ListOrValue::Value(v) => write!(f, "{}", v),
        }
    }
}

impl Ord for ListOrValue {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (ListOrValue::List(a), ListOrValue::List(b)) => {
                for (ai, bi) in a.iter().zip(b.iter()) {
                    let cmp = ai.cmp(bi);
                    if cmp != Equal {
                        return cmp;
                    }
                }

                b.len().cmp(&a.len())
            }
            (ListOrValue::List(_), ListOrValue::Value(_)) => {
                let b = ListOrValue::List(vec![other.clone()]);
                self.cmp(&b)
            }
            (ListOrValue::Value(_), ListOrValue::List(_)) => {
                let a = ListOrValue::List(vec![self.clone()]);
                a.cmp(other)
            }
            (ListOrValue::Value(a), ListOrValue::Value(b)) => b.cmp(&a),
        }
    }
}

/* If only this exercise was on 0..9 digits... */
fn numstack_to_int(num_stack: &mut Vec<char>) -> Result<u32, ParseIntError> {
    let r = num_stack.iter().collect::<String>().parse::<u32>();
    num_stack.clear();
    r
}

impl From<&str> for ListOrValue {
    fn from(s: &str) -> Self {
        let mut double_stack: Vec<Vec<ListOrValue>> = vec![vec![]];
        let mut num_stack = vec![];
        for c in s.chars() {
            match c {
                '[' => {
                    if let Ok(n) = numstack_to_int(&mut num_stack) {
                        let n = ListOrValue::Value(n);
                        double_stack.last_mut().expect("?").push(n);
                    }
                    double_stack.push(vec![]);
                }
                ']' => {
                    if let Ok(n) = numstack_to_int(&mut num_stack) {
                        let n = ListOrValue::Value(n);
                        double_stack.last_mut().expect("?").push(n);
                    }

                    let top = double_stack.pop().expect("Invalid input");
                    double_stack
                        .last_mut()
                        .expect("?")
                        .push(ListOrValue::List(top));
                }
                n if n.is_digit(10) => {
                    num_stack.push(n);
                }
                _ => {
                    if let Ok(n) = numstack_to_int(&mut num_stack) {
                        let n = ListOrValue::Value(n);
                        double_stack.last_mut().expect("?").push(n);
                    }
                }
            }
        }
        double_stack.pop().expect("Invalid input").pop().expect("?")
    }
}

fn main() {
    let r1 = include_str!("./input/13.txt")
        .split("\n\n")
        .map(|pair| pair.split_once("\n"))
        .filter_map(|x| x)
        .map(|(a, b)| (ListOrValue::from(a), ListOrValue::from(b)))
        .enumerate()
        .filter_map(|(i, (a, b))| if a.gt(&b) { Some(i + 1) } else { None })
        .sum::<usize>();

    println!("Result I: {}", r1);

    let two = ListOrValue::from("[[2]]");
    let six = ListOrValue::from("[[6]]");
    let mut sort_me = include_str!("./input/13.txt")
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| ListOrValue::from(line))
        .chain(vec![two.clone(), six.clone()].into_iter())
        .collect::<Vec<_>>();

    sort_me.sort_by(|a, b| a.cmp(b).reverse());

    let two_idx = sort_me.iter().position(|x| x == &two).expect("?") + 1;
    let six_idx = sort_me.iter().position(|x| x == &six).expect("?") + 1;
    println!("Result II: {}", two_idx * six_idx);
}
