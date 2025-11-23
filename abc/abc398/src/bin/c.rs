use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [isize; n],
    }

    let mut map = HashMap::new();
    for &x in &a {
        *map.entry(x).or_insert(0usize) += 1;
    }

    let mut result = (-1, -1);
    for (i, &x) in a.iter().enumerate() {
        if map.get(&x) == Some(&1) {
            result = result.max((x, i as isize + 1));
        }
    }

    println!("{}", result.1);
}
