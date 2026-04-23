use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [isize; n],
        lrw: [(Usize1, Usize1, isize); m],
    }

    let mut imos = vec![0; n + 1];
    for (l, r, w) in lrw {
        imos[l] += w;
        imos[r + 1] -= w;
    }
    for i in 1..=n {
        imos[i] += imos[i - 1];
    }

    let result = (0..n).filter(|&i| imos[i] > s[i]).count();
    println!("{result}");
}
