use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut v = vec![];
    for x in a {
        if x % 2 == 0 {
            v.push(x);
        }
    }
    let result = v.iter().join(" ");
    println!("{}", result);
}
