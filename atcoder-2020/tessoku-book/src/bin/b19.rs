use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        w_max: usize,
        wv: [(usize, usize); n],
    }

    let mut map = BTreeMap::new();
    map.insert(0, 0);

    for &(w, v) in &wv {
        let prev = map.clone();
        for (v0, w0) in &prev {
            let w1 = w + w0;
            if w1 > w_max {
                continue;
            }
            let v1 = v + v0;
            if let Some(&w1_prev) = map.get(&v1) {
                if w1 >= w1_prev {
                    continue;
                }
            }
            map.insert(v1, w1);
        }
    }

    if let Some((v, _)) = map.iter().last() {
        println!("{}", v);
    }
}
