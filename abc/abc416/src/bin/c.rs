use itertools::Itertools;
use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        n: usize,
        k: usize,
        x: Usize1,
        s: [Chars; n],
    }

    let mut stack = vec![vec![]];

    let mut vv = vec![];
    while let Some(v) = stack.pop() {
        for i in 0..n {
            let mut v = v.clone();
            v.push(i);
            if v.len() < k {
                stack.push(v);
            } else {
                vv.push(v);
            }
        }
    }

    let mut results = vec![];
    for v in vv {
        let result: Vec<_> = v.iter().flat_map(|&i| s[i].clone()).collect();
        results.push(result);
    }

    results.sort();
    let result = results[x].iter().join("");
    println!("{result}");
}
