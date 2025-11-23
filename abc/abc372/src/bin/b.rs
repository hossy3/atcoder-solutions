use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        mut m: usize,
    }

    let mut results = vec![];
    for i in 0..12 {
        let k = 3_usize.pow(i);
        while m % (k * 3) > 0 {
            results.push(i);
            m -= k;
        }
    }

    println!("{}", results.len());
    println!("{}", results.iter().join(" "));
}
