use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        w: [isize; n],
        lrd: [(Usize1, Usize1, isize); q],
    }

    let mut imos = vec![0isize; n + 1];
    for &(l, r, d) in lrd.iter() {
        imos[l] += d;
        imos[r + 1] -= d;
    }
    for i in 1..=n {
        imos[i] += imos[i - 1];
    }

    let result = (0..n).filter(|&i| w[i] <= imos[i]).count();
    println!("{result}");
}
