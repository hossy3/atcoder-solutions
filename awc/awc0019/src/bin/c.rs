use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut m = BTreeMap::new();
    for &v in &a {
        *m.entry(v).or_insert(0usize) += 1;
    }

    let mut result = 0;
    while let Some((&l, &count)) = m.iter().next() {
        result += 1;
        if count == 1 {
            m.remove(&l);
        } else {
            m.insert(l, count - 1);
        }

        let mut cur = l + 1;
        while let Some(&count) = m.get(&cur) {
            if count == 1 {
                m.remove(&cur);
            } else {
                m.insert(cur, count - 1);
            }
            cur += 1;
        }
    }

    println!("{result}");
}
