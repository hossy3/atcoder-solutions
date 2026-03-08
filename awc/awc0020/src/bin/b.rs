use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        mut s: isize,
        d: [isize; n],
        pr: [(Usize1, isize); m],
    }

    let mut v = vec![0; n];
    for (p, r) in pr {
        v[p] += r;
    }

    // シミュレーション
    let mut b = false; // ばてている
    for i in 0..n {
        s -= if b { d[i] * 2 } else { d[i] };
        if s <= 0 {
            b = true;
        }
        s += v[i];
    }

    println!("{s}");
}
