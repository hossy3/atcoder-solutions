se itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        l: Usize1,
        r: Usize1,
    }
    let mut v = vec![];
    for i in 0..l {
        v.push(i + 1);
    }
    for i in (l..(r + 1)).rev() {
        v.push(i + 1);
    }
    for i in (r + 1)..n {
        v.push(i + 1);
    }
    println!("{}", v.iter().join(" "));
}
