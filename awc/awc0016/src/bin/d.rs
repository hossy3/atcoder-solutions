use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        q: usize,
        a: [usize; n],
        lr: [(Usize1, Usize1); q],
    }

    let mut cum_a = vec![0; n + 1];
    for i in 0..n {
        cum_a[i + 1] = cum_a[i] + a[i];
    }

    let mut v = vec![0; n];
    for i in 0..n {
        let j = cum_a.partition_point(|&x| x <= cum_a[i] + k).min(n);
        v[i] = j;
    }

    let mut cum_v = vec![0; n + 1];
    for i in 0..n {
        cum_v[i + 1] = cum_v[i] + v[i];
    }

    for (l, r) in lr {
        let result = cum_v[r + 1] - cum_v[l];
        println!("{result}");
    }
}
