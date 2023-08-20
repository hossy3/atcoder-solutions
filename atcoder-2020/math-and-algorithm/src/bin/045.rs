use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }

    let mut v = vec![0usize; n];
    for &(a, b) in &ab {
        let x = a.max(b);
        v[x] += 1;
    }
    let count = v.iter().filter(|&&x| x == 1).count();
    println!("{}", count);
}
