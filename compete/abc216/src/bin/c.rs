use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        mut n: usize,
    }
    let mut v = vec![];
    loop {
        if n % 2 == 1 {
            v.push('A');
            if n == 1 {
                let result = v.iter().rev().join("");
                println!("{}", result);
                return;
            }
        }
        n /= 2;
        v.push('B');
    }
}
