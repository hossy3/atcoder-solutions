use ac_library::{Max, Segtree};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        p: [Usize1; n],
    }

    let mut inv = vec![0; n];
    for (i, &x) in p.iter().enumerate() {
        inv[x] = i;
    }

    let segtree: Segtree<Max<_>> = p.clone().into();

    let mut result = 0;
    let i = inv[n - 1];
    let mut stack = vec![];
    if i > 0 {
        stack.push((0, i, (0, i))); // l..r
    }
    if i < n {
        stack.push((0, i, (i + 1, n)));
    }
    while let Some((score, i, (l, r))) = stack.pop() {
        if l == r {
            result = result.max(score);
            continue;
        }
        let x = segtree.prod(l..r);
        let j = inv[x];
        let score = score + i.abs_diff(j);
        if j > l {
            stack.push((score, j, (l, j)));
        }
        if j < r {
            stack.push((score, j, (j + 1, r)));
        }
    }

    println!("{result}");
}
