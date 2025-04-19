use proconio::input;

fn isqrt(n: usize) -> usize {
    let r = (n as f64).sqrt() as usize;
    if (r + 1).pow(2) <= n {
        r + 1
    } else if r.pow(2) <= n {
        r
    } else {
        r - 1
    }
}

fn main() {
    input! {
        n: usize,
    }
    let m = (n as f64).powf(1.0 / 3.0) as usize + 1;
    for k in 1..=m {
        // k(x*x + x(x-k)+(x-k)(x-k)) = n を探す
        if n % k > 0 {
            continue;
        }
        let y = n / k;
        let z = 9 * k as i64 * k as i64 + 12 * (y as i64 - k as i64 * k as i64);
        if z < 0 {
            continue;
        }
        let z = z as usize;
        let z0 = isqrt(z);
        if z0.pow(2) != z {
            continue;
        }
        if (3 * k + z0) % 6 > 0 {
            continue;
        }
        let x = (3 * k + z0) / 6;
        if x <= k {
            continue;
        }
        let y = x - k;
        println!("{x} {y}");
        return;
    }
    println!("-1");
}
