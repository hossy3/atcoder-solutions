use std::collections::BTreeMap;

use proconio::input;

fn increment_or_insert(map: &mut BTreeMap<usize, usize>, key: usize) {
    *map.entry(key).or_insert(0) += 1;
}

fn decrement_or_remove(map: &mut BTreeMap<usize, usize>, key: usize) {
    if let Some(&count) = map.get(&key) {
        if count == 1 {
            map.remove(&key);
        } else {
            map.insert(key, count - 1);
        }
    }
}

fn main() {
    input! {
        q: usize,
    }
    let mut m_x = BTreeMap::new();
    let mut m_xor = BTreeMap::new();
    for _ in 0..q {
        input! {
            t: usize,
        }
        match t {
            1 => {
                input! {
                    x: usize,
                }
                increment_or_insert(&mut m_x, x);
                if *m_x.get(&x).unwrap() > 1 {
                    increment_or_insert(&mut m_xor, 0);
                    continue;
                }
                if let Some((&x0, _)) = m_x.range(..x).last() {
                    increment_or_insert(&mut m_xor, x0 ^ x);
                    if let Some((&x1, _)) = m_x.range((x + 1)..).next() {
                        decrement_or_remove(&mut m_xor, x0 ^ x1);
                        increment_or_insert(&mut m_xor, x ^ x1);
                    }
                } else if let Some((&x1, _)) = m_x.range((x + 1)..).next() {
                    increment_or_insert(&mut m_xor, x ^ x1);
                }
            }
            2 => {
                input! {
                    x: usize,
                }
                decrement_or_remove(&mut m_x, x);
                if m_x.contains_key(&x) {
                    decrement_or_remove(&mut m_xor, 0);
                    continue;
                }
                if let Some((&x0, _)) = m_x.range(..x).last() {
                    decrement_or_remove(&mut m_xor, x0 ^ x);
                    if let Some((&x1, _)) = m_x.range((x + 1)..).next() {
                        increment_or_insert(&mut m_xor, x0 ^ x1);
                        decrement_or_remove(&mut m_xor, x ^ x1);
                    }
                } else if let Some((&x1, _)) = m_x.range((x + 1)..).next() {
                    decrement_or_remove(&mut m_xor, x ^ x1);
                }
            }
            _ => {
                if let Some((&k, _)) = m_xor.iter().next() {
                    println!("{}", k);
                }
            }
        }
    }
}
