use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn f(a: &[usize]) -> usize {
    let n = a.len() / 2;
    let mut v = vec![vec![]; n];
    for (i, &x) in a.iter().enumerate() {
        v[x].push(i);
    }
    let mut set = HashSet::new();
    for i in 0..n {
        if v[i][1] - v[i][0] == 1 {
            continue;
        }
        if v[i][0] > 0 {
            let j = a[v[i][0] - 1];
            if !set.contains(&(i, j)) && v[j][1] - v[j][0] > 1 {
                if (v[i][0].abs_diff(v[j][0]) == 1 && v[i][1].abs_diff(v[j][1]) == 1)
                    || (v[i][0].abs_diff(v[j][1]) == 1 && v[i][1].abs_diff(v[j][0]) == 1)
                {
                    set.insert((i, j));
                    set.insert((j, i));
                }
            }
        }
        if v[i][0] < n - 1 {
            let j = a[v[i][0] + 1];
            if !set.contains(&(i, j)) && v[j][1] - v[j][0] > 1 {
                if (v[i][0].abs_diff(v[j][0]) == 1 && v[i][1].abs_diff(v[j][1]) == 1)
                    || (v[i][0].abs_diff(v[j][1]) == 1 && v[i][1].abs_diff(v[j][0]) == 1)
                {
                    set.insert((i, j));
                    set.insert((j, i));
                }
            }
        }
    }
    set.len() / 2
}

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            a: [Usize1; n * 2],
        }
        let result = f(&a);
        println!("{result}");
    }
}
