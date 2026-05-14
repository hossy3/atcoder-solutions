use ac_library::{Max, Segtree};
use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        xb: [(usize, usize); n],
        lr: [(usize, usize); q],
    }

    const N: usize = 200_000;
    const N3: usize = N * 3;

    let sum = {
        let mut sum = vec![0isize; N3 + 1];
        for &(x, b) in &xb {
            sum[N + x - b] += 1;
            sum[N + x] -= 2;
            sum[N + x + b] += 1;
        }

        for i in 1..=N3 {
            sum[i] += sum[i - 1];
        }
        for i in 1..=N3 {
            sum[i] += sum[i - 1];
        }

        sum
    };

    let segtree: Segtree<Max<isize>> = sum.iter().copied().collect();
    for &(l, r) in &lr {
        let (l, r) = (l + N - 1, r + N - 1);
        let result = segtree.prod(l..=r);
        println!("{result}");
    }
}
