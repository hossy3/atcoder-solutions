use proconio::input;

// n乗根を求める
fn powinv(x: u128, n: usize) -> u128 {
    let x0 = (x as f64).powf(1.0 / (n as f64)) as u128;
    if x0 == 0 {
        0
    } else if x0.pow(n as u32) > x {
        x0 - 1
    } else if (x0 + 1).pow(n as u32) > x {
        x0
    } else {
        x0 + 1
    }
}

fn main() {
    input! {
        n: u128,
    }

    let mut result = 1i64;

    let mut v = vec![1i64; 65];
    for i in 2..=64 {
        result += (powinv(n, i) as i64 - 1) * v[i];
        for j in 2..=(64 / i) {
            v[i * j] -= v[i];
        }
    }

    println!("{result}");
}
