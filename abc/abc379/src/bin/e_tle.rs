use num_bigint::BigUint;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        s: Chars,
    }

    let mut result = BigUint::from(0usize);
    let mut k = BigUint::from(1usize);
    for (i, &x) in s.iter().enumerate().rev() {
        let x = x as usize - '0' as usize;
        result += k.clone() * (i + 1) * x;
        k = k * 10usize + 1usize;
    }
    println!("{result}");
}
