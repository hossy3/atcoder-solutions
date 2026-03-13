use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        ab: [(Usize1, Usize1); m],
    }

    let mut v = vec![false; n];
    for i in 0..k {
        v[i] = true;
    }

    for (a, b) in ab {
        let x = v[a] | v[b];
        v[a] = x;
        v[b] = x;
    }

    let result = v.iter().filter(|&&b| b).count();
    println!("{result}");
}
