use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        lr: [(Usize1, Usize1); m],
    }
    let mut cum = vec![0i32; n + 1];
    for (l, r) in lr {
        cum[l] += 1;
        cum[r + 1] -= 1;
    }
    let mut v = vec![0i32; n + 1];
    for i in 0..n {
        v[i + 1] = v[i] + cum[i];
    }
    let result = v[1..].iter().min().unwrap_or(&0);
    println!("{result}");
}
