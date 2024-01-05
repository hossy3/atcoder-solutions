use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        a: [usize],
    }

    let mut m = BTreeMap::new();
    for &key in &a {
        m.insert(key, 0);
    }
    let len = m.len();
    for (i, (_, val)) in m.iter_mut().enumerate() {
        *val = len - i - 1;
    }

    let mut v = vec![0; a.len()];
    for &key in &a {
        v[m[&key]] += 1;
    }
    for &val in &v {
        println!("{}", val);
    }
}
