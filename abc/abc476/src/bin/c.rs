use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut v = vec![];
    v.push(a[0]);
    v.push(a[1]);
    for i in 2..n {
        v.push(a[i]);
        v.sort_by_key(|&x| Reverse(x));
        println!("{}", v[2]);
        if v.len() == 4 {
            v.pop();
        }
    }
}
