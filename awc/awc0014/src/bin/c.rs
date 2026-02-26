use proconio::input;

fn main() {
    input! {
        g: u128,
        m: u128,
        d: u128,
        k: u128,
        v: u128,
    }

    // 電車の着く時間; (m - g) / v
    // 高橋君の着く時間; (g - d * k + k) または (g / d)
    let yes = if d * k <= g {
        (g - d * k + k) * v <= m - g
    } else {
        g * v <= (m - g) * d
    };
    println!("{}", if yes { "Yes" } else { "No" });
}
