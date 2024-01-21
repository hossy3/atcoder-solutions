use itertools::Itertools;
use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        w: usize,
        lr: [(usize, usize)],
    }
    let mut m = BTreeMap::new();
    m.insert(1usize, 0usize);
    m.insert(w + 1, 0);

    for &(l, r) in &lr {
        let (_, &l0) = m.range(..=l).last().unwrap();
        let (_, &r0) = m.range(..=(r + 1)).last().unwrap();

        let h = m.range(l..=r).fold(l0, |acc, (_, &h)| acc.max(h)) + 1;
        let v = m.range(l..=r).map(|(&k, _)| k).collect_vec();
        for k in v {
            m.remove(&k);
        }

        m.insert(l, h);
        m.insert(r + 1, r0);

        println!("{h}");
    }
}
