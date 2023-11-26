use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        td: [(usize, usize); n],
    }
    let mut sei = BTreeSet::new();
    let mut esi = BTreeSet::new();
    for (i, &(t, d)) in td.iter().enumerate() {
        sei.insert((t, d + t, i));
        esi.insert((d + t, t, i));
    }

    let mut result = 0usize;
    let mut cur = 0usize;

    while !sei.is_empty() {
        let &(e, s, i) = esi.iter().next().unwrap();
        if e < cur {
            esi.remove(&(e, s, i));
            sei.remove(&(s, e, i));
            continue;
        } else if s <= cur {
            esi.remove(&(e, s, i));
            sei.remove(&(s, e, i));
            result += 1;
            cur += 1;
            continue;
        }

        let &(s, e, i) = sei.iter().next().unwrap();
        if e < cur {
            esi.remove(&(e, s, i));
            sei.remove(&(s, e, i));
        } else {
            esi.remove(&(e, s, i));
            sei.remove(&(s, e, i));
            result += 1;
            cur = s.max(cur) + 1;
        }
    }

    println!("{result}");
}
