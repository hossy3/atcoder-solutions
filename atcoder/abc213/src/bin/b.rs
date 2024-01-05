use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut v = (0..n).map(|i| (a[i], i + 1)).collect_vec();
    v.sort();
    let result = v[v.len() - 2].1;
    println!("{}", result);
}
