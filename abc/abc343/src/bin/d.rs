use std::collections::HashMap;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, t): (usize, usize),
        ab: [(Usize1, usize); t],
    }

    let mut v = vec![0usize; n];
    let mut map = HashMap::new();
    map.insert(0, n);

    for (a, b) in ab {
        let Some(count) = map.get_mut(&v[a]) else { unreachable!() };
        *count -= 1;
        if *count == 0 {
            map.remove(&v[a]);
        }
        v[a] += b;
        *map.entry(v[a]).or_insert(0) += 1;

        let result = map.len();
        println!("{result}");
    }
}
