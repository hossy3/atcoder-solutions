use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        lr: [(usize, usize); n],
    }

    let mut set_lr = BTreeSet::new(); // (l, r)
    let mut set_rl = BTreeSet::new(); // (r, l)
    for &(l, r) in &lr {
        set_lr.insert((l, r));
        set_rl.insert((r, l));
    }
    let mut count = 0;
    while let Some(&(r, _)) = set_rl.iter().next() {
        count += 1;
        let r0 = r + d - 1;
        while let Some(&(l, r)) = set_lr.iter().next() {
            if l > r0 {
                break;
            }
            set_lr.remove(&(l, r));
            set_rl.remove(&(r, l));
        }
    }
    println!("{}", count);
}
