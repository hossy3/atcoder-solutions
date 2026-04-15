use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        h: [isize; n],
        lrd: [(Usize1, Usize1, isize); m],
    }

    let mut imos = vec![0isize; n + 1];
    imos[0] = h[0];
    for i in 1..n {
        imos[i] = h[i] - h[i - 1];
    }
    imos[n] = -h[n - 1];

    for (l, r, d) in lrd {
        imos[l] -= d;
        imos[r + 1] += d;
    }

    let mut results = vec![0isize; n];
    results[0] = imos[0];
    for i in 1..n {
        results[i] = results[i - 1] + imos[i];
    }
    let result = results.into_iter().filter(|&x| x > 0).count();
    println!("{result}");
}
