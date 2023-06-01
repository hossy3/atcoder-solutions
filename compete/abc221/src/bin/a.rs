use num_traits::PrimInt;
use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
    }
    let result = 32.pow(a - b);
    println!("{}", result);
}
