use std::{
    cmp::Reverse,
    collections::{BTreeMap, BTreeSet},
};

use proconio::input;

fn main() {
    input! {
        _: usize,
        m: usize,
        a: [usize; m],
    }

    let mut map = BTreeMap::<usize, (usize, Reverse<usize>)>::new();
    let mut set = BTreeSet::new();
    for x in a {
        if let Some(&value) = map.get(&x) {
            set.remove(&value);
            let new_value = (value.0 + 1, value.1);
            map.insert(x, new_value);
            set.insert(new_value);
        } else {
            let new_value = (0usize, Reverse(x));
            map.insert(x, new_value);
            set.insert(new_value);
        }
        let result = set.last().unwrap().1 .0;
        println!("{result}");
    }
}
