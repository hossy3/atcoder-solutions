use itertools::Itertools;
use proconio::{input, marker::Chars};

fn to_usize(c: char) -> usize {
    c as usize - '0' as usize
}

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }

    let mut v = vec![(0usize, 0usize)]; // 0, 1 の数
    for i in 0..n {
        let x = to_usize(s[i]);
        if i > 0 && x == 0 && v[v.len() - 1].1 > 0 {
            v.push((0usize, 0usize));
        }
        let j = v.len() - 1;
        if x == 0 {
            v[j].0 += 1;
        } else {
            v[j].1 += 1;
        }
    }

    v[k - 2].1 += v[k - 1].1;
    v[k - 1].1 = 0;

    let mut results = vec![];
    for (n0, n1) in v {
        results.append(&mut vec![0; n0]);
        results.append(&mut vec![1; n1]);
    }
    let result = results.iter().join("");
    println!("{result}");
}
