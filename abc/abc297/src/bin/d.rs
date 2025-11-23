use std::mem::swap;

use proconio::input;

fn main() {
    input! {
        mut a: usize,
        mut b: usize,
    }
    let mut n = 0usize;
    while a != b {
        if a < b {
            swap(&mut a, &mut b);
        }
        if a % b == 0 {
            n += (a / b) - 1;
            break;
        }
        let i = a / b;
        n += i;
        a -= i * b;
    }
    println!("{}", n);
}
