use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let m = (n - 1) / 2;
    let s = vec!['-'; m].iter().join("");
    if n % 2 == 1 {
        println!("{}={}", &s, &s);
    } else {
        println!("{}=={}", &s, &s);
    }
}
