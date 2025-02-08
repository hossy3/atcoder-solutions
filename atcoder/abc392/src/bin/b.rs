use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    }

    let mut v = vec![true; n + 1];
    for &x in &a {
        v[x] = false;
    }
    let mut results = vec![];
    for i in 1..=n {
        if v[i] {
            results.push(i);
        }
    }

    let len = results.len();
    println!("{len}");

    let result = results.iter().join(" ");
    println!("{result}");
}
