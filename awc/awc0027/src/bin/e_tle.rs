use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: isize,
        k: isize,
        a: [isize; n],
    }

    let mut v = vec![0isize; n];
    for (i, &x) in a.iter().enumerate() {
        v[i] = x + m;
    }
    for i in 0..(n - 1) {
        v[i + 1] += v[i];
    }
    let mut set = BTreeSet::new();
    for (i, &x) in v.iter().enumerate() {
        set.insert((x, i));
    }

    let mut result = 0usize;
    for (i, &x) in v.iter().enumerate() {
        let offset = if i == 0 { 0 } else { v[i - 1] };
        result += set.range(..=(k + offset, usize::MAX)).count();
        set.remove(&(x, i));
    }

    println!("{result}");
}
