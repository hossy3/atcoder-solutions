use num_integer::Integer;
use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
    }
    let gcd = a.gcd(&b);
    if (b, 0) > 10u64.pow(18).div_mod_floor(&(a / gcd)) {
        println!("Large");
    } else {
        let lcm = a / gcd * b;
        println!("{lcm}");
    }
}
