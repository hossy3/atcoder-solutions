use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut v = vec![a[0]];
    for i in 1..n {
        if a[i - 1] + 1 < a[i] {
            for x in (a[i - 1] + 1)..=a[i] {
                v.push(x);
            }
        } else if a[i - 1] - 1 > a[i] {
            for x in (a[i]..=(a[i - 1] - 1)).rev() {
                v.push(x);
            }
        } else {
            v.push(a[i]);
        }
    }
    let result = v.iter().join(" ");
    println!("{}", result);
}
