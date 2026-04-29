use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        k: isize,
        lr: [(Usize1, Usize1); m],
    }

    let mut imos = vec![0isize; n + 1];
    for &(l, r) in &lr {
        imos[l] += 1;
        imos[r + 1] -= 1;
    }
    let mut cum = vec![0isize; n + 1];
    for i in 0..n {
        cum[i + 1] = cum[i] + imos[i];
    }
    let result = (1..=n).filter(|&i| cum[i] >= k).count();
    println!("{result}");
}
