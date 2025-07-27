use std::{
    cmp::Reverse,
    collections::{BTreeMap, BinaryHeap},
};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut x: [usize; n],
    }

    x.sort();

    let mut map = BTreeMap::new();
    for &x in &x {
        map.insert(x, x);
    }

    let mut queue = BinaryHeap::new();
    for v in x.windows(2) {
        if v[1] > v[0] {
            queue.push((Reverse(v[1] - v[0]), v[0]));
        }
    }

    let mut result = 0;
    while map.len() > m {
        let Some((Reverse(dist), l)) = queue.pop() else {
            unreachable!()
        };
        let Some((&l0, &r0)) = map.range(..=l).last() else {
            unreachable!()
        };
        if l < r0 {
            continue; // 接続対象ではない
        }
        let Some((&l1, &r1)) = map.range((l + 1)..).next() else {
            unreachable!()
        };
        result += dist;
        map.remove(&l0);
        map.remove(&l1);
        map.insert(l0, r1);
    }

    println!("{result}");
}
