use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [usize; n - 1],
    }
    let mut v = vec![0];
    for &d in &d {
        let Some(&x) = v.last() else { unreachable!() };
        v.push(x + d);
    }
    for i in 0..(n - 1) {
        let mut results = vec![];
        for j in (i + 1)..n {
            results.push(v[j] - v[i]);
        }
        println!("{}", results.iter().join(" "));
    }
}
