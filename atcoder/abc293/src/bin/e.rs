use num::{BigUint, Integer};
use proconio::input;

fn main() {
    input! {
        a: BigUint,
        x: BigUint,
        m: BigUint,
    }
    let one = BigUint::from(1usize);
    if a == one {
        println!("{}", x % m);
    } else {
        let result = (a.modpow(&x, &(m * (&a - &one))) - &one) / (&(a - &one));
        println!("{}", result);
    }
}
