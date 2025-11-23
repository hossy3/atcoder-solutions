use std::collections::{BTreeMap, BTreeSet};

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [String; n],
        t: [String; m],
    }

    if n == 1 && s[0].len() < 3 {
        println!("{}", -1);
        return;
    }

    let rest = 16 - ((n - 1) + s.iter().map(|x| x.len()).sum::<usize>());
    let mut map = BTreeMap::new();
    for i in 0..n {
        map.insert(s[i].as_str(), i);
    }

    let mut set = BTreeSet::new();
    'outer: for t0 in t {
        let mut v0 = vec![];
        let mut v1 = vec![0usize; n - 1];
        for t1 in t0.split("_") {
            if t1.len() == 0 {
                if v0.len() == 0 {
                    continue 'outer;
                }
                v1[v0.len() - 1] += 1;
            } else {
                if let Some(&x) = map.get(t1) {
                    v0.push(x);
                } else {
                    continue 'outer;
                }
            }
        }
        v0.append(&mut v1);
        set.insert(v0);
    }

    for a0 in (0usize..n).permutations(n) {
        for a1 in (0usize..(n - 1 + rest)).combinations(rest) {
            let mut a = a0.clone();
            let mut a2 = vec![0usize; n];
            let mut offset = 0;
            for &x in &a1 {
                a2[x - offset] += 1;
                offset += 1;
            }
            a2.pop();
            a.append(&mut a2);
            if !set.contains(&a) {
                for i in 0..(n - 1) {
                    print!("{}{}", s[a[i]], "_".repeat(a[n + i] + 1));
                }
                println!("{}", s[a[n - 1]]);
                return;
            }
        }
    }
    println!("{}", -1);
}
