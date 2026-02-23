use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        q: usize,
        a: [usize; n],
        lr: [(Usize1, Usize1); q],
    }

    let mut v = vec![0usize; n];
    for i in 0..(n - 1) {
        if a[i + 1].abs_diff(a[i]) <= k {
            v[i + 1] = v[i];
        } else {
            v[i + 1] = v[i] + 1;
        }
    }

    for (l, r) in lr {
        let yes = v[l] == v[r];
        println!("{}", if yes { "Yes" } else { "No" });
    }
}
