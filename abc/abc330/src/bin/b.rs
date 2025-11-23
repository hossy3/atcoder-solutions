use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        l: i64,
        r: i64,
        a: [i64; n],
    }
    let mut v = Vec::with_capacity(n);
    for x in a {
        let y = if l <= x && x <= r {
            x
        } else if x.abs_diff(l) < x.abs_diff(r) {
            l
        } else {
            r
        };
        v.push(y);
    }
    let result = v.iter().join(" ");
    println!("{result}");
}
