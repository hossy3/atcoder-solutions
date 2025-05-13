use proconio::input;

type Mint = ac_library::ModInt998244353;
const MOD: usize = 998244353;

fn combination(l: usize, r: usize, fact: &[Mint]) -> usize {
    let n = l + r;
    if l == 0 || r == 0 {
        1
    } else {
        (fact[n] / (fact[l] * fact[r])).val() as usize
    }
}

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }

    let n = a + b + c + d;
    let mut fact = vec![Mint::new(1); n + 1];
    for i in 2..=n {
        fact[i] = fact[i - 1] * i;
    }

    let mut result = 0usize;

    // リンゴをすべて左側に配置するまでに何個のオレンジを使ってよいか
    for b0 in 0..=b {
        let x = combination(a - 1, b0, &fact);

        // ブドウをすべて右側に配置するまでに何個のバナナを使ってよいか
        for c0 in 0..=c {
            let y = combination(d - 1, c0, &fact);

            // 残ったバナナとオレンジは適当に配置して良い
            let z = combination(b - b0, c - c0, &fact);

            result += (((x * y) % MOD) * z) % MOD;
            result %= MOD;
        }
    }

    println!("{result}");
}
