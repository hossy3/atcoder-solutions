use std::{collections::HashSet, mem::swap};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        c: [usize; n],
        ab: [(Usize1, Usize1); q],
    }

    let mut v = Vec::with_capacity(n);
    for x in c {
        let mut set = HashSet::new();
        set.insert(x);
        v.push(set);
    }

    for (a, b) in ab {
        if v[a].len() > v[b].len() {
            let mut set = HashSet::new();
            swap(&mut set, &mut v[a]);
            for &x in &v[b] {
                set.insert(x);
            }
            swap(&mut set, &mut v[b]);
        } else {
            let mut set = HashSet::new();
            swap(&mut set, &mut v[b]);
            for &x in &v[a] {
                set.insert(x);
            }
            swap(&mut set, &mut v[b]);
            v[a].clear();
        }

        let result = v[b].len();
        println!("{result}");
    }
}
