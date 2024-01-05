use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut m = BTreeMap::new();
    for x in &a {
        if m.contains_key(x) {
            m.insert(*x, m[x] + 1);
        } else {
            m.insert(*x, 1);
        }
    }

    let mut rest = n;
    let mut i = 0;
    while rest > 0 {
        if rest == 1 {
            if !m.contains_key(&(i + 1)) {
                break;
            }
            i += 1;
            break;
        } else {
            if !m.contains_key(&(i + 1)) {
                rest -= 1;
            }
            i += 1;
        }
        rest -= 1;
    }
    println!("{}", i);
}
