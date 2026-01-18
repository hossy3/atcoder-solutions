use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [isize; n],
    }

    let mut result = 0;
    let mut map0 = HashMap::new();
    let mut map1 = HashMap::new();
    for (i, &x) in a.iter().enumerate() {
        let x0 = x + i as isize;
        let x1 = x - i as isize;
        if let Some(&count) = map0.get(&(-x1)) {
            result += count;
        }
        if let Some(&count) = map1.get(&(-x0)) {
            result += count;
        }
        *map0.entry(x0).or_insert(0) += 1;
        *map1.entry(x1).or_insert(0) += 1;
    }
    println!("{result}");
}
