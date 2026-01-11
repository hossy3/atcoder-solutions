use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, usize); n],
    }
    let mut v = vec![(0usize, 0usize); m];
    for &(a, b) in &ab {
        v[a].0 += b;
        v[a].1 += 1;
    }
    for &(sum, count) in &v {
        let result = sum as f64 / count as f64;
        println!("{result}");
    }
}
