use std::collections::{BTreeMap, VecDeque};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        tws: [(usize, usize, usize); m],
    }

    let mut results = vec![0; n];

    let mut map = BTreeMap::<usize, VecDeque<usize>>::new();
    {
        let deque: VecDeque<_> = (0..n).collect();
        map.insert(0, deque);
    }
    for (t, w, s) in tws {
        let (k, mut v) = map.pop_first().unwrap();
        if k > t {
            map.insert(k, v);
            continue;
        }
        let i = v[0];
        results[i] += w;
        map.entry(t + s)
            .or_insert(VecDeque::<_>::new())
            .push_back(i);
        if v.len() > 1 {
            v.pop_front();
            map.insert(k, v);
        }
    }

    for x in results {
        println!("{x}");
    }
}
