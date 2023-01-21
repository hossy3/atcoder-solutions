use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        r: usize,
        s: usize,
        a: [usize; n],
    }

    let mut v = Vec::with_capacity(n);
    for i in 1..p {
        v.push(a[i - 1]);
    }
    for i in p..=q {
        v.push(a[i + r - p - 1]);
    }
    for i in (q + 1)..r {
        v.push(a[i - 1]);
    }
    for i in r..=s {
        v.push(a[i + p - r - 1]);
    }
    for i in (s + 1)..=n {
        v.push(a[i - 1]);
    }
    let result = v.iter().join(" ");
    println!("{}", result);
}
