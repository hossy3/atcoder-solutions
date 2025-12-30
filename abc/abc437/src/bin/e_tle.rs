use std::collections::BTreeMap;

use itertools::Itertools;
use proconio::input;

// Trie

fn f(map: &BTreeMap<(usize, usize), (usize, Vec<usize>)>, i: usize) -> Vec<usize> {
    let mut results = vec![];
    for (_, (node_no, indexes)) in map.range((i, 0)..(i + 1, 0)) {
        results.append(&mut indexes.clone());
        results.append(&mut f(map, *node_no));
    }
    results
}

fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n],
    }

    let mut map = BTreeMap::<_, (_, Vec<usize>)>::new(); // [(node_no, y)] = (next_node_no, indexes)
    let mut v = vec![usize::MAX; n + 1]; // [index] = node_no
    v[0] = 0;

    for (i, &(x, y)) in xy.iter().enumerate() {
        if let Some((node_no, indexes)) = map.get_mut(&(v[x], y)) {
            indexes.push(i + 1);
            v[i + 1] = *node_no;
        } else {
            map.insert((v[x], y), (i + 1, vec![i + 1]));
            v[i + 1] = i + 1;
        }
    }

    let results = f(&map, 0);
    let result = results.iter().join(" ");
    println!("{result}");
}
