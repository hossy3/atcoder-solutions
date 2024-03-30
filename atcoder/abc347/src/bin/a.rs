use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        a: [usize; n],
    }
    let result = a.iter().filter(|&&x| x % k == 0).map(|&x| x / k).join(" ");
    println!("{result}");
}
