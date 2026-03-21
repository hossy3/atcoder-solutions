use std::collections::HashSet;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        t: [Usize1; n],
    }

    let mut results = vec![0; n];
    for i in 0..n {
        if results[i] > 0 {
            continue;
        }

        let mut set = HashSet::new();
        let mut cur = i;
        while results[cur] == 0 {
            if !set.insert(cur) {
                break;
            }
            cur = t[cur];
        }

        let result = if results[cur] > 0 {
            results[cur]
        } else {
            let mut count = 1;
            let mut cur0 = t[cur];
            while cur0 != cur {
                count += 1;
                cur0 = t[cur0];
            }
            count
        };
        for x in set {
            results[x] = result;
        }
    }

    println!("{}", results.iter().join(" "));
}
