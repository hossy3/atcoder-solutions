use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        td: [(usize, usize); n],
    }
    let mut sei = BTreeSet::new();
    for (i, &(t, d)) in td.iter().enumerate() {
        sei.insert((t, d + t, i));
    }

    let mut ei = BTreeSet::new();
    let mut result = 0usize;
    let mut cur = 1usize;

    while !sei.is_empty() || !ei.is_empty() {
        if ei.is_empty() {
            if let Some(&(s, _, _)) = sei.iter().next() {
                cur = s;
            }
        }
        while let Some(&(s, e, i)) = sei.iter().next() {
            if s != cur {
                break;
            }
            sei.remove(&(s, e, i));
            ei.insert((e, i));
        }
        while let Some(&(e, i)) = ei.iter().next() {
            if e >= cur {
                break;
            }
            ei.remove(&(e, i));
        }
        if ei.pop_first().is_some() {
            result += 1;
            cur += 1;
        }
    }

    println!("{result}");
}
