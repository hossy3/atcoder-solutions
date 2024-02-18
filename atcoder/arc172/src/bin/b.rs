use proconio::input;

type Mint = ac_library::ModInt998244353;

// 100 文字, 80 個選ぶ, 26 種類の文字 (20個前と同じにできない)
// 最初の文字は 26 種類 (余裕 20個)
// 最初の文字は 25 種類
// 最初の文字は 24 種類...

fn f(n: usize, k: usize, l: usize) -> u32 {
    if n - k >= l {
        return 0;
    }

    let mut result = Mint::new(1);
    let k0 = l - (n - k); // 6..=26
    for i in k0..=l {
        result *= i;
    }
    if k0 > 0 {
        let k1 = n - (l - k0) - 1; // 79
        result *= Mint::new(k0).pow(k1 as u64);
    }

    result.val()
}

fn main() {
    input! {
        (n, k, l): (usize, usize, usize),
    }
    let result = f(n, k, l);
    println!("{result}");
}
