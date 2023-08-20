use itertools::Itertools;
use proconio::{input, marker::Chars};

// bit search

fn main() {
    input! {
        n: Chars,
    }
    let mut v = n.iter().map(|&c| c as usize - '0' as usize).collect_vec();
    v.sort_by(|a, b| b.cmp(&a));
    let k = v.len();
    let m = 1 << k;
    let mut result = 0;
    for i in 0..m {
        let mut a = 0;
        let mut b = 0;
        for j in 0..k {
            if (1 << j) & i > 0 {
                a = a * 10 + v[j];
            } else {
                b = b * 10 + v[j];
            }
        }
        result = result.max(a * b);
    }
    println!("{}", result);
}
