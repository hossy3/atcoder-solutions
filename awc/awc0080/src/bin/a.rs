use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        mut h: [isize; n],
        td: [(Usize1, isize); m],
    }

    for &(t, d) in &td {
        h[t] -= d;
        if t > 0 {
            h[t - 1] -= d / 2;
        }
        if t < n - 1 {
            h[t + 1] -= d / 2;
        }
    }
    let result = h.iter().filter(|&&h| h >= 1).count();
    println!("{result}");
}
