use std::collections::BTreeMap;
use std::ops::Bound::{Included, Unbounded};

use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        k: usize,
        mut ac: [(usize, usize); n],
    }

    let mut m = BTreeMap::new();
    m.insert(0, (k, 0)); // (l, (r, cost))
    if k < l {
        m.insert(l, (l, 0));
    }
    ac.sort_by_key(|x| (x.1, x.0));
    for (a, c) in ac {
        if m.len() == 1 {
            break;
        }
        if let Some((&l0, &(r0, cost0))) = m.range((Unbounded, Included(a))).last() {
            if a + k <= r0 {
                continue;
            }
            if let Some((&l1, &(r1, cost1))) = m.range((Included(a), Unbounded)).next() {
                if l1 - a > k {
                    if a > r0 {
                        m.insert(a, (a + k, c * k));
                    } else {
                        m.insert(l0, (a + k, cost0 + c * (a + k - r0)));
                    }
                } else {
                    if a > r0 {
                        m.insert(a, (r1, c * (l1 - a) + cost1));
                    } else {
                        m.insert(l0, (r1, cost0 + c * (l1 - r0) + cost1));
                    }
                    m.remove(&l1);
                }
            }
        }
    }

    if m.len() == 1 {
        if let Some((_, (_, cost))) = m.iter().next() {
            println!("{}", cost);
        }
    } else {
        println!("-1");
    }
}
