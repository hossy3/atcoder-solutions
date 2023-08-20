use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut count = 0;
    let mut map = HashMap::new();
    for (i, &x) in a.iter().enumerate() {
        count += i;
        let j = map.get(&x).unwrap_or(&0);
        count -= j;
        map.insert(x, j + 1);
    }
    println!("{}", count);
}
