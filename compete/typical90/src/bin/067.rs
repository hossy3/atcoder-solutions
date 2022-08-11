use proconio::input;

fn conv(n: u128, k0: u128, k1: u128, b8to5: bool) -> u128 {
    let mut n = n;
    let mut x = 0;
    let mut k = 1;
    while n > 0 {
        let mut r = n % k0;
        if b8to5 && r == 8 {
            r = 5;
        };
        x += r * k;
        k *= k1;
        n /= k0;
    }
    x
}

fn main() {
    input! {
        mut n: u128,
        k: usize,
    }

    n = conv(n, 10, 8, false);
    for _ in 0..k {
        n = conv(n, 9, 8, true);
    }
    n = conv(n, 8, 10, false);
    println!("{}", n);
}
