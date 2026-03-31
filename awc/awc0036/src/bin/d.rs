use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        se: [(usize, usize); n],
    }

    let mut stack = vec![];
    for (i, &(s, e)) in se.iter().enumerate() {
        stack.push((s, e, i));
    }
    stack.sort_unstable();
    stack.reverse();

    let mut set = BTreeSet::new();

    while let Some((s, e, i)) = stack.pop() {
        if let Some(&(x, i0)) = set.range(..=(s, usize::MAX)).last() {
            set.remove(&(x, i0));
        }
        set.insert((e, i));
    }

    println!("{}", set.len());
}
