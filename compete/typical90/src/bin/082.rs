use proconio::input;

const MOD: u128 =  1_000_000_007;

fn f(x: u128) -> u128 {
    let mut x = x;
    let mut count = 0;
    for i in (0..=18).rev() {
        let y = 10u128.pow(i);
        if x >= y {
            count += ((x + 1 - y) * (x + y) / 2) * (i as u128 + 1);
            count %= MOD;
            x = y - 1;
        }
    }
    count
}

fn main() {
    input! {
        l: u128,
        r: u128,
    }
    let count = (MOD + f(r) - f(l - 1)) % MOD;
    println!("{}", count);
}
