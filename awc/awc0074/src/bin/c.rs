use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        lr: [(Usize1, Usize1); m],
    }

    let mut imos = vec![0usize; n + 1];
    for &(l, r) in &lr {
        imos[l] += 1;
        imos[r + 1] += 1;
    }
    for i in 0..n {
        imos[i + 1] += imos[i];
    }
    let result = imos[..n].iter().filter(|&&x| x % 2 == 1).count();
    println!("{result}");
}
