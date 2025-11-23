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

    let mut results = vec![];

    let mut stack = vec![];
    for i in 0..n {
        stack.push((1, s[i].clone()));
    }
    while let Some((j, v)) = stack.pop() {
        for i in 0..n {
            let mut v = v.clone();
            v.append(&mut s[i].clone());
            let j = j + 1;
            if j == k {
                results.push(v);
            } else {
                stack.push((j, v));
            }
        }
    }

    results.sort();
    let result = results[x].iter().join("");
    println!("{result}");
}
