use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut a = vec![];
    for _ in 0..n {
        input! {
            a0: [usize],
        }
        a.push(a0);
    }
    input! {
        x: usize,
    }

    let mut result = (usize::MAX, vec![]);
    for (i, a) in a.iter().enumerate() {
        if a.contains(&x) {
            if a.len() == result.0 {
                result.1.push(i + 1);
            } else if a.len() < result.0 {
                result.0 = a.len();
                result.1 = vec![i + 1];
            }
        }
    }

    println!("{}", result.1.len());
    println!("{}", result.1.iter().join(" "));
}
