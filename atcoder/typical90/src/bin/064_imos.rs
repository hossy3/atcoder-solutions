use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
        lrv: [(usize, usize, i64); q]
    }

    let mut imos = vec![0i64; n + 1];
    for i in 0..(n - 1) {
        imos[i + 1] = a[i + 1] - a[i];
    }

    let mut score: i64 = imos[1..].iter().map(|x| x.abs()).sum();

    for (l, r, v) in lrv {
        if l - 1 > 0 {
            score -= imos[l - 1].abs();
            imos[l - 1] += v;
            score += imos[l - 1].abs();
        }
        if r < n {
            score -= imos[r].abs();
            imos[r] -= v;
            score += imos[r].abs();
        }
        println!("{score}");
    }
}
