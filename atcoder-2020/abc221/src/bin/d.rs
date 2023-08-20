use std::collections::BTreeMap;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }

    let mut map = BTreeMap::new();
    for &(a, b) in &ab {
        map.insert(a, 0);
        map.insert(a + b, 0);
    }
    for (i, (_, v)) in map.iter_mut().enumerate() {
        *v = i;
    }

    let mut imos = vec![0i64; map.len()];
    for &(a, b) in &ab {
        let &i = map.get(&a).unwrap();
        let &j = map.get(&(a + b)).unwrap();
        imos[i] += 1;
        imos[j] -= 1;
    }
    for i in 0..(imos.len() - 1) {
        imos[i + 1] += imos[i];
    }

    let mut results = vec![0; n + 1];
    for ((&k0, &i0), (&k1, _)) in map.iter().tuple_windows() {
        let i0 = imos[i0] as usize;
        results[i0] += k1 - k0;
    }
    let result = results[1..].into_iter().join(" ");
    println!("{}", result);
}
