use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; 2 * n],
    }
    let mut result = 0;
    let mut map = HashMap::new();
    for (i, &x) in a.iter().enumerate() {
        if let Some(&i0) = map.get(&x) {
            if i.abs_diff(i0) == 2 {
                result += 1;
            }
        } else {
            map.insert(x, i);
        }
    }
    println!("{result}");
}
