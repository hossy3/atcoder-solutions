use ac_library::{Max, Segtree};
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut segtree = Segtree::<Max<usize>>::new(n);
    for (i, &a) in a.iter().enumerate() {
        let x = if i == 0 {
            a
        } else if i < k {
            segtree.prod(..i).max(a)
        } else {
            segtree.prod((i - k)..i).max(segtree.prod(..(i - k)) + a)
        };
        segtree.set(i, x);
    }

    let result = segtree.get(n - 1);
    println!("{result}");
}
