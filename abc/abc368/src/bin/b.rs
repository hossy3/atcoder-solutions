use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    let mut result = 0usize;
    a.sort_by_key(|&x| Reverse(x));
    while a[1] > 0 {
        result += 1;
        a[0] -= 1;
        a[1] -= 1;
        a.sort_by_key(|&x| Reverse(x));
    }
    println!("{result}");
}
