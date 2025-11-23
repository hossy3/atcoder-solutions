use std::collections::BTreeMap;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        _n: usize,
        q: usize,
        a: [Usize1; q],
    }

    let mut results = vec![];
    let mut map = BTreeMap::new();
    for &a in &a {
        if let Some((&a0, &b0)) = map.range(..a).last() {
            if b0 > a {
                map.remove(&a0);
                map.insert(a0, a - 1);
                map.insert(a + 1, b0);
                results.push(map.len());
                continue;
            } else if b0 == a {
                map.remove(&a0);
                map.insert(a0, a - 1);
                results.push(map.len());
                continue;
            } else if b0 == a - 1 {
                if let Some((&a1, &b1)) = map.range(a..).next() {
                    if a1 == a + 1 {
                        map.remove(&a0);
                        map.remove(&a1);
                        map.insert(a0, b1);
                        results.push(map.len());
                        continue;
                    }
                }
                map.remove(&a0);
                map.insert(a0, a);
                results.push(map.len());
                continue;
            }
        }

        if let Some((&a1, &b1)) = map.range(a..).next() {
            if a1 == a + 1 {
                map.remove(&a1);
                map.insert(a, b1);
                results.push(map.len());
                continue;
            } else if a1 == a {
                map.remove(&a);
                if b1 > a {
                    map.insert(a + 1, b1);
                }
                results.push(map.len());
                continue;
            }
        }

        map.insert(a, a);
        results.push(map.len());
    }

    for result in results {
        println!("{result}");
    }
}
