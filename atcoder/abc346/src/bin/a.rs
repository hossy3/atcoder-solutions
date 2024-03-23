use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut v = vec![];
    for a in a.windows(2) {
        v.push(a[0] * a[1]);
    }
    let result = v.iter().join(" ");
    println!("{result}");
}
