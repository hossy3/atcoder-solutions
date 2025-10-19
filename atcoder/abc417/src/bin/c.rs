use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let mut map = HashMap::new();
    let mut result = 0i64;
    for (i, &a) in a.iter().enumerate() {
        let b = i as i64 - a;
        result += map.get(&b).unwrap_or(&0);

        let a = a + i as i64;
        *map.entry(a).or_insert(0) += 1;
    }

    println!("{result}");
}
