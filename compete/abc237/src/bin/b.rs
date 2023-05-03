use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    }
    for i in 0..w {
        let result = (0..h).map(|j| a[j][i]).join(" ");
        println!("{}", result);
    }
}
