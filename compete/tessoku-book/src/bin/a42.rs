use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        ab: [(Usize1, Usize1); n],
    }

    const MAX: usize = 100;

    let mut imos = vec![[0i64; MAX + 1]; MAX + 1];
    for &(a, b) in &ab {
        let a0 = (a + k + 1).min(MAX);
        let b0 = (b + k + 1).min(MAX);
        imos[a][b] += 1;
        imos[a0][b] -= 1;
        imos[a][b0] -= 1;
        imos[a0][b0] += 1;
    }

    for i in 0..MAX {
        for j in 0..=MAX {
            imos[i + 1][j] += imos[i][j];
        }
    }
    for i in 0..=MAX {
        for j in 0..MAX {
            imos[i][j + 1] += imos[i][j];
        }
    }
    let result = imos.iter().map(|a| a.iter().max().unwrap()).max().unwrap();
    println!("{}", result);
}
