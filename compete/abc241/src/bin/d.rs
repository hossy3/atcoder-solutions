use std::collections::BTreeMap;

use proconio::input;

// multimap-like

fn get_lower(x: i64, mut k: usize, map: &BTreeMap<i64, usize>) -> i64 {
    let mut it = map.range(..=x);
    while let Some((&i, &v)) = it.next_back() {
        if k <= v {
            return i;
        }
        k -= v;
    }
    -1
}

fn get_upper(x: i64, mut k: usize, map: &BTreeMap<i64, usize>) -> i64 {
    let mut it = map.range(x..);
    while let Some((&i, &v)) = it.next() {
        if k <= v {
            return i;
        }
        k -= v;
    }
    -1
}

fn main() {
    input! {
        q: usize,
    }

    let mut map = BTreeMap::new();
    for _ in 0..q {
        input! {
            t: usize,
            x: i64,
        }

        match t {
            1 => {
                *map.entry(x).or_insert(0usize) += 1;
            }
            2 => {
                input! {
                    k: usize,
                }
                let result = get_lower(x, k, &map);
                println!("{}", result);
            }
            _ => {
                input! {
                    k: usize,
                }
                let result = get_upper(x, k, &map);
                println!("{}", result);
            }
        }
    }
}
