use proconio::input;

fn conv(n: u128, k0: u128, k1: u128) -> u128 {
    let mut x = 0;
    for i in 0.. {
        if n < k0.pow(i) {
            break;
        }
        let mut r = (n / k0.pow(i)) % k0;
        if k0 == 9 && k1 == 8 && r == 8 {
            r = 5;
        };
        x += r * k1.pow(i);
    }
    x
}

fn main() {
    input! {
        mut n: u128,
        k: usize,
    }

    n = conv(n, 10, 8);
    for _ in 0..k {
        n = conv(n, 9, 8);
    }
    n = conv(n, 8, 10);
    println!("{n}");
}
