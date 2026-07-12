use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        rc: [(Usize1, Usize1); m],
    }

    let mut rows = vec![usize::MAX; n]; // その行の何列目にコマがあるか
    let mut cols = vec![usize::MAX; n]; // その列の何行目にコマがあるか

    for &(r, c) in &rc {
        if rows[r] != usize::MAX {
            cols[rows[r]] = usize::MAX;
        }
        if cols[c] != usize::MAX {
            rows[cols[c]] = usize::MAX;
        }
        rows[r] = c;
        cols[c] = r;
    }

    let result = rows.iter().filter(|&&x| x != usize::MAX).count();
    println!("{result}");
}
