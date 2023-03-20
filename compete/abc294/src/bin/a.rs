use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let v: Vec<_> = a.iter().filter(|&x| *x % 2 == 0).collect_vec();
    let result = v.iter().join(" ");
    println!("{}", result);
}
