use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        uva: [(Usize1, Usize1, usize); m],
        s: [Usize1; k],
    }

    let mut ss = HashSet::new();
    for &x in &s {
        ss.insert(x);
    }

    let mut map = HashMap::<usize, Vec<(usize, usize)>>::new();
    for &(u, v, a) in &uva {
        if !map.contains_key(&u) {
            map.insert(u, Vec::new());
        }
        map.get_mut(&u).unwrap().push((v, a));

        if !map.contains_key(&v) {
            map.insert(v, Vec::new());
        }
        map.get_mut(&v).unwrap().push((u, a));
    }

    const MAX: usize = 10000000;
    let mut result = vec![vec![MAX; 2]; n];

    let mut heap = BinaryHeap::<(Reverse<usize>, usize, usize)>::new();
    heap.push((Reverse(0), 0, 1));
    while let Some((Reverse(step), i, a)) = heap.pop() {
        if result[i][a] < step {
            continue;
        }
        result[i][a] = step;
        if i == n - 1 {
            break;
        }
        if map.contains_key(&i) {
            for &(j, b) in &map[&i] {
                if a == b {
                    heap.push((Reverse(step + 1), j, b));
                }
            }
            if ss.contains(&i) {
                for &(j, b) in &map[&i] {
                    if 1 - a == b {
                        heap.push((Reverse(step + 1), j, b));
                    }
                }
            }
        }
    }

    let r = result[n - 1][0].min(result[n - 1][1]);
    if r == MAX {
        println!("-1");
    } else {
        println!("{}", r);
    }
}
