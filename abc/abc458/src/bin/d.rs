use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        x: usize,
        q: usize,
        ab: [(usize, usize); q],
    }

    let mut set0 = BTreeSet::new(); // 小さいほう
    let mut set1 = BTreeSet::new(); // 大きいほう
    set0.insert((x, 0));

    for (i, &(a, b)) in ab.iter().enumerate() {
        set0.insert((a, i * 2 + 1));
        set1.insert((b, i * 2 + 2));
        for _ in 0..2 {
            if let Some(xi0) = set0.pop_last() {
                if let Some(xi1) = set1.pop_first() {
                    if xi0 > xi1 {
                        set0.insert(xi1);
                        set1.insert(xi0);
                    } else {
                        set0.insert(xi0);
                        set1.insert(xi1);
                        break;
                    }
                }
            }
        }
        // eprintln!("{:?}, {:?}", &set0, &set1);
        if let Some(&(x, _)) = set0.last() {
            println!("{x}");
        };
    }
}
