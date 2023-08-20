use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let mut m = HashMap::<usize, usize>::new();
    let mut r = 0;
    let mut max_len = 0;
    for l in 0..n {
        while r < n && (m.len() < k || (m.len() == k && m.contains_key(&a[r]))) {
            *m.entry(a[r]).or_insert(0) += 1;
            r += 1;
        }
        max_len = max_len.max(r - l);
        if let Some(x) = m.get_mut(&a[l]) {
            *x -= 1;
            if *x == 0 {
                m.remove(&a[l]);
            }
        };
    }
    println!("{}", max_len);
}
