use std::collections::BTreeMap;

use itertools::Itertools;
use proconio::input;

// Trie

fn f(maps: &[BTreeMap<usize, (usize, Vec<usize>)>], i: usize, results: &mut Vec<usize>) {
    for (_, (node_no, indexes)) in &maps[i] {
        results.append(&mut indexes.clone());
        f(maps, *node_no, results);
    }
}

fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n],
    }

    let mut maps = vec![BTreeMap::<_, (_, Vec<usize>)>::new(); n + 1]; // [node_no][y] = (next_node_no, indexes)
    let mut v = vec![usize::MAX; n + 1]; // [index] = node_no
    v[0] = 0;

    for (i, &(x, y)) in xy.iter().enumerate() {
        if let Some((node_no, indexes)) = maps[v[x]].get_mut(&y) {
            indexes.push(i + 1);
            v[i + 1] = *node_no;
        } else {
            maps[v[x]].insert(y, (i + 1, vec![i + 1]));
            v[i + 1] = i + 1;
        }
    }

    let mut results = vec![];
    f(&maps, 0, &mut results);
    let result = results.iter().join(" ");
    println!("{result}");
}
