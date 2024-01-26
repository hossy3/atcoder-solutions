use num_integer::Integer;
use proconio::input;

fn main() {
    input! {
        a: u128,
        b: u128,
    }
    let lcm = a.lcm(&b);
    if lcm > 10u128.pow(18) {
        println!("Large");
    } else {
        println!("{lcm}");
    }
}
