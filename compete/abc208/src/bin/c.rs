use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let mut v = Vec::with_capacity(n);
    for (i, &x) in a.iter().enumerate() {
        v.push((i, x));
    }
    v.sort_by_key(|&(_, a)| a);
    let mut v = (0..n).map(|i| (v[i].0, i)).collect_vec();
    v.sort();
    let v = v.iter().map(|&(_, x)| x).collect_vec();
    for x in v {
        let result = k / n + if k % n > x { 1 } else { 0 };
        println!("{}", result);
    }
}
