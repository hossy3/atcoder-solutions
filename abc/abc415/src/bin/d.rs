use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let mut map = BTreeMap::new();
    for &(a, b) in &ab {
        let c = a - b;
        if let Some(a0) = map.get_mut(&c) {
            if *a0 > a {
                *a0 = a;
            }
        } else {
            map.insert(c, a);
        }
    }

    let mut n = n;
    let mut result = 0usize;
    while let Some((c, a)) = map.pop_first() {
        if n >= a {
            let x = (n - a) / c + 1;
            result += x;
            n -= x * c;
        }
    } 
    println!("{result}");
}
