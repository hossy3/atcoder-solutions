use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        p: [usize; n],
        l: [usize; m],
        d: [usize; m],
    }

    let mut sum: usize = p.iter().sum();
    let mut map = BTreeMap::new();
    for x in p {
        *map.entry(x).or_insert(0) += 1;
    }

    let mut v = Vec::with_capacity(m);
    for i in 0..m {
        v.push((d[i], l[i]));
    }
    v.sort();
    v.reverse();

    for &(d, l) in &v {
        if let Some((&x, &count)) = map.range(l..).next() {
            sum -= d;
            if count == 1 {
                map.remove(&x);
            } else {
                map.insert(x, count - 1);
            }
        }
    }

    println!("{}", sum);
}
