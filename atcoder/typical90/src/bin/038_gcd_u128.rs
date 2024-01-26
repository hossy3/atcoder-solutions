use proconio::input;

fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    input! {
        a: u128,
        b: u128,
    }
    let c = gcd(a, b);
    let lcm = (a / c) * b;
    if lcm > 10u128.pow(18) {
        println!("Large");
    } else {
        println!("{lcm}");
    }
}
