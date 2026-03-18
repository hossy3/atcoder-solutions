use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut v = vec![];
    for i in (0..n).step_by(k) {
        let i = i + k - 1;
        if i < n {
            v.push(a[i]);
        }
    }

    println!("{}", v.iter().join(" "));
}
