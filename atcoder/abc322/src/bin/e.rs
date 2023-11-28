use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: usize,
        ca: [(i64, [usize; k]); n],
    }

    let mut map = HashMap::new();
    map.insert((0, 0, 0, 0, 0), 0); // state, cost
    for (c, a) in ca {
        let prev = map.clone();
        for (state, cost0) in prev {
            let key = (
                (state.0 + a[0]).min(p),
                (state.1 + a.get(1).unwrap_or(&0)).min(p),
                (state.2 + a.get(2).unwrap_or(&0)).min(p),
                (state.3 + a.get(3).unwrap_or(&0)).min(p),
                (state.4 + a.get(4).unwrap_or(&0)).min(p),
            );
            if let Some(&cost) = map.get(&key) {
                if cost > cost0 + c {
                    map.insert(key, cost0 + c);
                }
            } else {
                map.insert(key, cost0 + c);
            }
        }
    }

    let result = map
        .iter()
        .filter(|&(state, _)| {
            state.0 >= p
                && (k <= 1 || state.1 >= p)
                && (k <= 2 || state.2 >= p)
                && (k <= 3 || state.3 >= p)
                && (k <= 4 || state.4 >= p)
        })
        .map(|(_, cost)| cost)
        .min()
        .unwrap_or(&-1);
    println!("{result}");
}
