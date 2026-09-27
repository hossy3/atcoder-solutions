use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        x: [usize; n],
    }

    let mut v = vec![];
    for (i, &xi) in x.iter().enumerate() {
        v.push((xi, i + 1));
    }
    v.sort_unstable();

    let mut results = vec![];
    for i in 0..n {
        if (i == 0 || v[i].0 - v[i - 1].0 >= d) && (i == n - 1 || v[i + 1].0 - v[i].0 >= d) {
            results.push(v[i].1);
        }
    }
    results.sort_unstable();

    println!("{}", results.len());
    println!("{}", results.iter().join(" "));
}
