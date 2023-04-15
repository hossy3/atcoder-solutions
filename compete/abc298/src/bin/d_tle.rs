use num::BigUint;
use proconio::input;

fn main() {
    input! {
        q: usize,
    }
    const MOD: usize = 998244353;
    let b10 = BigUint::from(10usize);
    let mut s = BigUint::from(1usize);
    let mut k = BigUint::from(1usize);

    for _ in 0..q {
        input! {
            t: usize,
        }
        match t {
            1 => {
                input! {
                    x: BigUint,
                }
                s = s * &b10 + x;
                k *= &b10;
            }
            2 => {
                s %= &k;
                k /= &b10;
            }
            _ => {
                let result = &s % MOD;
                println!("{}", result);
            }
        }
    }
}
