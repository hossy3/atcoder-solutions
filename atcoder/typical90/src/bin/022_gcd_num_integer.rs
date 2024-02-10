use num_integer::Integer;
use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }
    let w = a.gcd(&b).gcd(&c);
    let result = (a / w - 1) + (b / w - 1) + (c / w - 1);
    println!("{result}");
}
