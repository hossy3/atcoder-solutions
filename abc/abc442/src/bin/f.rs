use ac_library::{Min, Segtree};
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }

    let mut segtree: Segtree<Min<usize>> = Segtree::new(n + 1); // 上から順に 白が左から n 個詰まっているときの操作最小値
    for i in 0..(n + 1) {
        segtree.set(i, 0);
    }
    for s in s {
        let mut count = s.iter().filter(|&&c| c == '.').count(); // 操作数 まずすべて黒にする
        segtree.set(0, segtree.all_prod() + count);
        for j in 0..n {
            if s[j] == '.' {
                count -= 1; // もともと白だったので操作しなくてよくなる
            } else {
                count += 1;
            }
            segtree.set(j + 1, segtree.prod((j + 1)..) + count);
        }
    }

    let result = segtree.all_prod();
    println!("{result}");
}
