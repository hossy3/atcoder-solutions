use std::collections::{HashMap, HashSet};

use proconio::input;

fn main() {
    input! {
        ab: [(usize, usize)],
    }
    let mut m = HashMap::<usize, Vec<usize>>::new();
    for &(a, b) in &ab {
        if !m.contains_key(&a) {
            m.insert(a, Vec::new());
        }
        m.get_mut(&a).unwrap().push(b);

        if !m.contains_key(&b) {
            m.insert(b, Vec::new());
        }
        m.get_mut(&b).unwrap().push(a);
    }

    let mut stack = vec![1];
    let mut set = HashSet::new();
    let mut max = 1;
    while let Some(x) = stack.pop() {
        if set.contains(&x) {
            continue;
        }
        set.insert(x);
        max = max.max(x);
        if let Some(v) = m.get(&x) {
            for &i in v {
                stack.push(i);
            }
        }
    }
    println!("{}", max);
}
