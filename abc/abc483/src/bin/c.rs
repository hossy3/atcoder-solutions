use ac_library::{Max, Segtree};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }

    let mut segtree: Segtree<Max<usize>> = Segtree::new(n);
    for i in 0..n {
        segtree.set(i, n - i - 1);
    }

    let mut v = vec![vec![]; n]; // v[第一候補者] = [第二候補者...]
    for &(a, b) in &ab {
        v[a].push(b);
        segtree.set(a, segtree.get(a) + n); // 第一候補者の番号を入れる
    }

    for (i, v0) in v.iter().enumerate() {
        segtree.set(i, segtree.get(i) - v[i].len() * n);
        for &j in v0 {
            segtree.set(j, segtree.get(j) + n);
        }
        let result = n - (segtree.all_prod() % n);
        println!("{result}");

        // for j in 0..n {
        //     eprint!("{} ", segtree.get(j) / n);
        // }
        // eprintln!();

        for &j in v0 {
            segtree.set(j, segtree.get(j) - n);
        }
        segtree.set(i, segtree.get(i) + v[i].len() * n);
    }
}
