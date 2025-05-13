use proconio::input;

type Mint = ac_library::ModInt998244353;

fn combination(l: usize, r: usize, fact: &[Mint]) -> Mint {
    let n = l + r;
    if l == 0 || r == 0 {
        Mint::new(1)
    } else {
        fact[n] / (fact[l] * fact[r])
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

    let mut result = Mint::new(0);

    // 左端のブドウの前にバナナが何個あるか
    for c0 in 0..=c  {
        // 左端のブドウの左側: オレンジの配置を決める。残りはリンゴとバナナを順に埋める
        let x = combination(b, a + c0, &fact);

        // 左端のブドウの右側: 残りのバナナと残りのブドウの配置を決める
        let y = combination(c - c0, d - 1, &fact);

        result += x * y;
    }

    println!("{result}");
}
