use proconio::input;

type Mint = ac_library::ModInt1000000007;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [isize; n],
    }

    if k == 1 {
        let result = 0; // 1 要素なら常に max == min
        println!("{result}");
        return;
    }

    a.sort_unstable();

    // [(k-1)C(k-1), kC(k-1), (k+1)C(k-1), ..., (n-1)C(k-1)] を前計算する
    let mut v = vec![Mint::new(1)];
    for i in k..n {
        let x = v[v.len() - 1] * Mint::new(i) / Mint::new(i - k + 1);
        v.push(x);
    }

    let mut min = Mint::new(0);
    for i in 0..=(n - k) {
        min += v[n - k - i] * a[i];
    }

    let mut max = Mint::new(0);
    for i in 0..=(n - k) {
        max += v[n - k - i] * a[n - i - 1];
    }

    let result = max - min;
    println!("{result}");
}
