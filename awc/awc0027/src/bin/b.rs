use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let Some((_, Reverse(result))) = a
        .iter()
        .enumerate()
        .map(|(i, &x)| (x, Reverse(i + 1)))
        .max()
    else {
        unreachable!()
    };

    if result == 1 {
        println!("-1");
    } else {
        println!("{result}");
    }
}
