use proconio::{input, marker::Usize1};

fn f(n: usize, xy: &[(usize, usize)], ab: &[(usize, usize)]) -> Vec<bool> {
    let mut v = vec![0usize; n];
    for (k, &(a, _)) in ab.iter().enumerate() {
        v[a] |= 1 << k;
    }
    for &(x, y) in xy {
        v[y] |= v[x];
    }

    let result: Vec<bool> = ab
        .iter()
        .enumerate()
        .map(|(k, &(_, b))| (v[b] >> k) & 1 == 1)
        .collect();
    result
}

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        mut xy: [(Usize1, Usize1); m],
        ab: [(Usize1, Usize1); q],
    }
    xy.sort();

    for i in (0..q).step_by(64) {
        let nbits = (q - i).min(64);
        let results = f(n, &xy, &ab[i..(i + nbits)]);
        for yes in results {
            println!("{}", if yes { "Yes" } else { "No" });
        }
    }
}
