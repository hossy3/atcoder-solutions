use std::collections::BTreeSet;

use ac_library::FenwickTree;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        c: [usize; n],
    }

    let mut fenwick = FenwickTree::new(n + 1, 0isize);
    let mut set: BTreeSet<(usize, usize)> = BTreeSet::new();
    for (i, &c) in c.iter().enumerate() {
        if i == 0 || c != set.last().unwrap().1 {
            set.insert((i, c));
            fenwick.add(i, 1);
        }
    }

    for _ in 0..q {
        input! {
            k: u8,
        }
        match k {
            1 => {
                input! {
                    l: Usize1,
                    r: usize,
                    x: usize,
                }

                // もともとの l - 1, r の値を記録する
                let (_, xl) = *set.range((0, 0)..(l, 0)).last().unwrap_or(&(0, usize::MAX));
                let (_, xr) = *set.range((0, 0)..(r + 1, 0)).last().unwrap();

                // l..=r はすべて消す
                while let Some(&(i, c)) = set.range((l, 0)..(r + 1, 0)).last() {
                    set.remove(&(i, c));
                    fenwick.add(i, -1);
                }

                // 必要なら追加する
                if xl != x {
                    set.insert((l, x));
                    fenwick.add(l, 1);
                }
                if xr != x {
                    set.insert((r, xr));
                    fenwick.add(r, 1);
                }
            }
            2 => {
                input! {
                    l: Usize1,
                    r: usize,
                }
                let result = fenwick.sum((l + 1)..r) + 1;
                // eprintln!("{:?}", &set);
                println!("{result}");
            }
            _ => unreachable!(),
        }
    }
}
