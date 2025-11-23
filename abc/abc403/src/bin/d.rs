use std::collections::{BTreeMap, HashSet};

use proconio::input;

fn f(d: usize, mut a: Vec<usize>) -> usize {
    let n = a.len();
    a.sort();

    if d == 0 {
        let count_all = a.windows(2).filter(|v| v[0] != v[1]).count() + 1;
        return n - count_all;
    }

    let mut m = BTreeMap::new();
    for &x in &a {
        *m.entry(x).or_insert(0usize) += 1;
    }

    let mut count_all = 0usize;

    let mut checked = HashSet::new();
    for (&x, _) in &m {
        if checked.contains(&x) {
            continue;
        }

        let mut x = x;
        let mut v = vec![];
        while let Some(&count) = m.get(&x) {
            v.push(count);
            checked.insert(x);
            x += d;
        }

        // DP で一番お得な選び方をする
        let k = v.len();
        for i in 0..k {
            if i >= 3 {
                v[i] += v[i - 2].max(v[i - 3]);
            } else if i >= 2 {
                v[i] += v[i - 2];
            }
        }
        let count = *v.iter().max().unwrap();
        count_all += count;
    }

    n - count_all
}

#[test]
fn test_func() {
    assert_eq!(f(0, vec![1, 1, 1, 2, 3, 4, 4, 4, 5, 6, 6]), 5);
    assert_eq!(f(1, vec![1, 1, 1, 2, 3, 4, 4, 4, 5, 6, 6]), 3);
    assert_eq!(f(2, vec![1, 1, 1, 2, 3, 4, 4, 4, 5, 6, 6]), 4);
}

fn main() {
    input! {
        n: usize,
        d: usize,
        a: [usize; n],
    }

    let result = f(d, a);
    println!("{result}");
}
