use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let mut map = HashMap::new();
    map.insert(0, 1);
    let mut result = 0usize;
    let mut sum = 0usize;
    for a in a {
        sum = (sum + a) % k;
        if let Some(&count) = map.get(&sum) {
            result += count;
        }
        *map.entry(sum).or_insert(0) += 1;
    }
    println!("{result}");
}
