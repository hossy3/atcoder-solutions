use ac_library::{Max, Segtree};
use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        a: [usize; n],
    }

    const N: usize = 500_000;
    let mut segtree = Segtree::<Max<usize>>::new(N + 1);
    for x in a {
        let l = if x > d { x - d } else { 0 }; // x.saturating_sub(d);
        let r = if x + d <= N { x + d } else { N }; // (x + d).min(N);
        let y = segtree.prod(l..=r) + 1;
        segtree.set(x, y);
    }
    let result = segtree.all_prod();
    println!("{result}");
}
