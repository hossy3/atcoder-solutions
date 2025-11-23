use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut v = vec![(n + 1, n + 1); n + 2]; // (prev, next)
    v[n + 1] = (0, 0);
    for (i, &c) in s.iter().enumerate() {
        let (l, r) = if c == 'L' { (v[i].0, i) } else { (i, v[i].1) };
        v[i + 1].0 = l;
        v[i + 1].1 = r;
        v[l].1 = i + 1;
        v[r].0 = i + 1;
    }

    let mut a = vec![];
    let mut i = v[n + 1].1;
    while i != n + 1 {
        a.push(i);
        i = v[i].1;
    }
    let result = a.iter().join(" ");
    println!("{}", result);
}
