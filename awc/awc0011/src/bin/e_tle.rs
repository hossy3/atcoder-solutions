use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); n],
    }

    let mut candidates = vec![HashSet::new(); m + 1];
    let mut max_values = vec![0; m + 1];

    for (i, &(a, b)) in ab.iter().enumerate() {
        for j in (0..=(m - a)).rev() {
            if max_values[j + a] < max_values[j] + b {
                max_values[j + a] = max_values[j] + b;
                candidates[j + a].clear();
                for &x in &candidates[j].clone() {
                    candidates[j + a].insert(x);
                }
                candidates[j + a].insert(i);
            } else if max_values[j + a] == max_values[j] + b {
                for &x in &candidates[j].clone() {
                    candidates[j + a].insert(x);
                }
                candidates[j + a].insert(i);
            }
        }
    }
    // eprintln!("{:?}", &candidates);

    for i in (0..=m).rev() {
        if candidates[i].len() > 0 {
            let mut results = vec![false; n];
            for &j in &candidates[i] {
                results[j] = true;
            }
            for yes in results {
                println!("{}", if yes { "Yes" } else { "No" });
            }
            return;
        }
    }
}
