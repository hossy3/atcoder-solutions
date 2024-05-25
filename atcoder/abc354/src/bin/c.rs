use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        ac: [(usize, usize); n],
    }

    let mut v = vec![];
    for (i, &(a, c)) in ac.iter().enumerate() {
        v.push(((a, c), i));
    }
    v.sort();
    v.reverse();

    let mut results = vec![];
    let mut c_max = usize::MAX;
    for &((_, c), i) in &v {
        if c < c_max {
            results.push(i + 1);
            c_max = c;
        }
    }
    results.sort();

    println!("{}", results.len());
    println!("{}", results.iter().join(" "));
}
